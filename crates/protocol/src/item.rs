//! Грамматика ItemStack/DataComponentPatch (26.3): значения компонентов не имеют длины,
//! поэтому для разбора стека нужна форма каждого компонента.
// Таблицы форм компонентов (проверено по исходнику 26.3).

use crate::error::{ProtocolError, ProtocolResult};
use crate::generated::r26_3 as reg;
use crate::nbt;
use crate::wire::{Reader, Writer};

const MAX_STRING_LENGTH: usize = 32767;

/// Реестры, на которые ссылаются диспетчеры форм.
#[derive(Clone, Copy, Debug)]
pub enum ClientRegistry {
    SoundEvent,
    Item,
    DataComponentType,
    PositionSourceType,
    ParticleType,
    BlockEntityType,
    ConsumeEffectType,
    EntityType,
    Attribute,
    Block,
}

impl ClientRegistry {
    pub fn name_of(self, id: u32) -> Option<&'static str> {
        let t: &[&str] = match self {
            Self::SoundEvent => reg::SOUND_EVENT,
            Self::Item => reg::ITEM,
            Self::DataComponentType => reg::DATA_COMPONENT_TYPE,
            Self::PositionSourceType => reg::POSITION_SOURCE_TYPE,
            Self::ParticleType => reg::PARTICLE_TYPE,
            Self::BlockEntityType => reg::BLOCK_ENTITY_TYPE,
            Self::ConsumeEffectType => reg::CONSUME_EFFECT_TYPE,
            Self::EntityType => reg::ENTITY_TYPE,
            Self::Attribute => reg::ATTRIBUTE,
            Self::Block => reg::BLOCK,
        };
        t.get(id as usize).copied()
    }
}

/// fails before the stack does.
pub(crate) const MAX_DEPTH: u32 = 512;

/// `ByteBufCodecs.list()` / `map()` with no explicit limit.
const UNBOUNDED: usize = usize::MAX;

/// A component value's wire layout, mirroring a `StreamCodec`.
#[derive(Debug)]
pub enum Shape {
    /// `StreamCodec.unit`: no bytes.
    Unit,
    Bool,
    /// `ByteBufCodecs.INT`.
    Int,
    Long,
    Float,
    Double,
    VarInt,
    /// `ByteBufCodecs.stringUtf8(max)`.
    Utf(usize),
    Uuid,
    /// `ByteBufCodecs.TAG` / `fromCodecWithRegistries`: any NBT but `End`.
    Nbt,
    /// `ByteBufCodecs.COMPOUND_TAG`.
    Compound,
    /// `ByteBufCodecs.optional`: a bool, then the value when true.
    Optional(&'static Shape),
    /// `ByteBufCodecs.list(max)`.
    List(usize, &'static Shape),
    /// `ByteBufCodecs.map`.
    Map(&'static Shape, &'static Shape),
    /// `ByteBufCodecs.either`: a bool, true for the left value.
    Either(&'static Shape, &'static Shape),
    /// `StreamCodec.composite`: the fields in order.
    Struct(&'static [Shape]),
    /// A shared shape, so composites can reuse (and recurse into) statics.
    Ref(&'static Shape),
    /// `ByteBufCodecs.holder`: varint 0 then an inline value, else id + 1.
    Holder(&'static Shape),
    /// [`Shape::Holder`] over a static registry, whose ids shift between
    /// versions.
    RegistryHolder(ClientRegistry, &'static Shape),
    /// `ByteBufCodecs.registry`: a static registry's id.
    RegistryId(ClientRegistry),
    /// `ByteBufCodecs.holderSet`: varint 0 then a tag id, else n + 1 ids.
    HolderSet,
    /// [`Shape::HolderSet`] over a static registry, whose ids shift between versions.
    RegistrySet(ClientRegistry),
    /// An `idMapper` enum dispatch with `OutOfBoundsStrategy.ZERO`.
    ById(&'static [Shape]),
    /// `ByteBufCodecs.registry(..).dispatch`, by entry name.
    ByRegistry(ClientRegistry, &'static [(&'static str, Shape)]),
    /// [`Shape::ByRegistry`] where every unlisted entry shares the last shape.
    ByRegistryOr(
        ClientRegistry,
        &'static [(&'static str, Shape)],
        &'static Shape,
    ),
    /// `ItemStackTemplate.STREAM_CODEC`.
    ItemTemplate,
    /// `TypedDataComponent.STREAM_CODEC`: a component id and its value.
    TypedComponent,
    /// A block state id (`Block.BLOCK_STATE_REGISTRY`); remapped between versions.
    BlockState,
    /// Pre-26.1 `ItemStack.OPTIONAL_STREAM_CODEC` (count, item, patch; empty when
    /// count <= 0). Source-version shapes only.
    Stack,
    /// Pre-26.1 `TagKey.streamCodec`: a bare tag id. Source-version shapes only.
    TagKey,
    /// The inner value; translation then appends these canonical bytes for
    /// fields newer versions added (defaults). Source-version shapes only.
    Append(&'static Shape, &'static [u8]),
    /// A struct whose fields are the first fields of the newer struct; the
    /// newer trailing fields are appended as these canonical bytes (defaults).
    /// Source-version shapes only.
    Extend(&'static [Shape], &'static [u8]),
    /// A bare value that newer versions wrap as the left branch of an
    /// [`Shape::Either`]. Source-version shapes only.
    Left(&'static Shape),
    /// A value that newer versions wrap as the right branch of an
    /// [`Shape::Either`]. Source-version shapes only.
    Right(&'static Shape),
    /// Pre-1.21.9 `CustomData.STREAM_CODEC`: a compound whose string `id`
    /// names the registry entry; newer versions split it into
    /// `TypedEntityData` (registry id, compound without `id`). Source-version
    /// shapes only.
    IdInNbt(ClientRegistry),
    /// The inner value followed by a field that newer versions removed
    /// (e.g. pre-1.21.5 `showInTooltip`): skipped on translation, written as
    /// these bytes (its default) on the way back. Source-version shapes only.
    Trailing(&'static Shape, &'static Shape, &'static [u8]),
    /// Pre-1.21.4 `CustomModelData.STREAM_CODEC`: a bare varint; newer versions
    /// hold four lists (floats, flags, strings, colors) and the old value is the
    /// single float. Source-version shapes only.
    IntAsFloats,
    /// No bytes in the source version; translation writes these canonical bytes
    /// (e.g. pre-1.21.2 unit `fire_resistant` → `damage_resistant` with the
    /// `is_fire` tag). Source-version shapes only.
    Const(&'static [u8]),
}

use Shape::*;

const IDENTIFIER: Shape = Utf(MAX_STRING_LENGTH);
const STRING: Shape = Utf(MAX_STRING_LENGTH);
/// `ComponentSerialization.STREAM_CODEC`: text components travel as NBT.
const TEXT: Shape = Nbt;

/// `SoundEvent.STREAM_CODEC`: a holder over (location, fixed range).
pub static SOUND: Shape = RegistryHolder(
    ClientRegistry::SoundEvent,
    &Struct(&[IDENTIFIER, Optional(&Float)]),
);

/// `MobEffectInstance.Details.STREAM_CODEC`, recursive through its hidden
/// effect.
static MOB_EFFECT_DETAILS: Shape = Struct(&[
    VarInt,
    VarInt,
    Bool,
    Bool,
    Bool,
    Optional(&MOB_EFFECT_DETAILS),
]);
/// `MobEffectInstance.STREAM_CODEC`: `MobEffect.STREAM_CODEC`, then details.
pub static MOB_EFFECT_INSTANCE: Shape = Struct(&[VarInt, Ref(&MOB_EFFECT_DETAILS)]);

/// `ConsumeEffect.STREAM_CODEC` over `consume_effect_type`, given
/// `teleport_randomly`'s layout, the one arm that differs between versions.
macro_rules! consume_effect {
    ($teleport_randomly:expr) => {
        ByRegistry(
            ClientRegistry::ConsumeEffectType,
            &[
                // ApplyStatusEffectsConsumeEffect.java:31
                (
                    "apply_effects",
                    Struct(&[List(UNBOUNDED, &MOB_EFFECT_INSTANCE), Float]),
                ),
                // RemoveStatusEffectsConsumeEffect.java:32
                ("remove_effects", HolderSet),
                ("clear_all_effects", Unit),
                ("teleport_randomly", $teleport_randomly),
                // PlaySoundConsumeEffect.java:28
                ("play_sound", Ref(&SOUND)),
            ],
        )
    };
}

/// TeleportRandomlyConsumeEffect.java:37: a diameter.
static CONSUME_EFFECT: Shape = consume_effect!(Float);
/// 26.3 TeleportRandomlyConsumeEffect.java:42 added `directionalParticles`.
static CONSUME_EFFECT_26_3: Shape = consume_effect!(Struct(&[Float, Bool]));

/// `Consumable.STREAM_CODEC`'s fields before the effects (Consumable.java:49).
static CONSUMABLE_HEAD: Shape = Struct(&[Float, VarInt, Ref(&SOUND), Bool]);

/// `Consumable.STREAM_CODEC` over a version's effects.
macro_rules! consumable {
    ($effect:expr) => {
        Struct(&[Ref(&CONSUMABLE_HEAD), List(UNBOUNDED, $effect)])
    };
}

/// `TrimPattern.STREAM_CODEC` (TrimPattern.java:30).
static TRIM_PATTERN: Shape = Holder(&Struct(&[IDENTIFIER, TEXT, Bool]));

/// `ArmorTrim.STREAM_CODEC` (ArmorTrim.java:38) over a version's trim
/// material.
macro_rules! trim {
    ($material:expr) => {
        Struct(&[Ref($material), Ref(&TRIM_PATTERN)])
    };
}

/// `StatePropertiesPredicate.PropertyMatcher.STREAM_CODEC`: a name, then an
/// exact value or a range of optional bounds.
static PROPERTY_MATCHER: Shape = Struct(&[
    STRING,
    Either(&STRING, &Struct(&[Optional(&STRING), Optional(&STRING)])),
]);

/// `DataComponentMatchers.STREAM_CODEC`: exact components, then up to 64
/// partial predicates, each a predicate or component type and an NBT body
/// (`DataComponentPredicate.TypeBase` encodes through its Codec).
static COMPONENT_MATCHERS: Shape = Struct(&[
    List(UNBOUNDED, &TypedComponent),
    List(64, &Struct(&[Either(&VarInt, &VarInt), Nbt])),
]);

/// `advancements.predicates.BlockPredicate.STREAM_CODEC`.
static BLOCK_PREDICATE: Shape = Struct(&[
    Optional(&RegistrySet(ClientRegistry::Block)),
    Optional(&List(UNBOUNDED, &PROPERTY_MATCHER)),
    Optional(&Compound),
    Ref(&COMPONENT_MATCHERS),
]);

/// `DataComponentMatchers.STREAM_CODEC` (1.21.10): the partial predicate is a bare
/// `DATA_COMPONENT_PREDICATE_TYPE` id (DataComponentPredicate.java, SINGLE_STREAM_CODEC);
/// 1.21.11 made it `either(predicate type, component type)`.
static COMPONENT_MATCHERS_1_21_10: Shape = Struct(&[
    List(UNBOUNDED, &TypedComponent),
    List(64, &Struct(&[Left(&VarInt), Nbt])),
]);

static BLOCK_PREDICATE_1_21_10: Shape = Struct(&[
    Optional(&RegistrySet(ClientRegistry::Block)),
    Optional(&List(UNBOUNDED, &PROPERTY_MATCHER)),
    Optional(&Compound),
    Ref(&COMPONENT_MATCHERS_1_21_10),
]);

/// `BlockPredicate.STREAM_CODEC` (1.21.4): no component matchers (1.21.5
/// appended them; default: both lists empty).
static BLOCK_PREDICATE_1_21_4: Shape = Extend(
    &[
        Optional(&RegistrySet(ClientRegistry::Block)),
        Optional(&List(UNBOUNDED, &PROPERTY_MATCHER)),
        Optional(&Compound),
    ],
    &[0, 0],
);

/// `ByteBufCodecs.GAME_PROFILE_PROPERTIES`.
static PROFILE_PROPERTIES: Shape = List(16, &Struct(&[Utf(64), Utf(32767), Optional(&Utf(1024))]));

/// `ResolvableProfile.STREAM_CODEC`: a full `GAME_PROFILE` or a `Partial`,
/// then `PlayerSkin.Patch.STREAM_CODEC`.
pub static RESOLVABLE_PROFILE: Shape = Struct(&[
    Either(
        &Struct(&[Uuid, Utf(16), Ref(&PROFILE_PROPERTIES)]),
        &Struct(&[
            Optional(&Utf(16)),
            Optional(&Uuid),
            Ref(&PROFILE_PROPERTIES),
        ]),
    ),
    Struct(&[
        Optional(&IDENTIFIER),
        Optional(&IDENTIFIER),
        Optional(&IDENTIFIER),
        Optional(&Bool),
    ]),
]);

/// `FireworkExplosion.STREAM_CODEC`.
static FIREWORK_EXPLOSION: Shape = Struct(&[
    VarInt,
    List(UNBOUNDED, &Int),
    List(UNBOUNDED, &Int),
    Bool,
    Bool,
]);

/// `TrimMaterial.DIRECT_STREAM_CODEC`: `MaterialAssetGroup` (a base suffix,
/// then equipment-asset overrides) and a description.
static TRIM_MATERIAL: Shape = Holder(&Struct(&[
    Struct(&[STRING, Map(&IDENTIFIER, &STRING)]),
    TEXT,
]));
/// 26.3 TrimMaterial.java:28 replaced the asset group with a palette id.
static TRIM_MATERIAL_26_3: Shape = Holder(&Struct(&[IDENTIFIER, TEXT]));

/// `TypedEntityData.streamCodec`: a registry id, then the compound.
/// `TypedEntityData.streamCodec` over `EntityType.STREAM_CODEC`.
static ENTITY_DATA: Shape = Struct(&[RegistryId(ClientRegistry::EntityType), Compound]);

/// `TypedEntityData.streamCodec` over the block entity type registry.
static BLOCK_ENTITY_DATA: Shape = Struct(&[RegistryId(ClientRegistry::BlockEntityType), Compound]);

/// `KineticWeapon.Condition.STREAM_CODEC`.
static KINETIC_CONDITION: Shape = Struct(&[VarInt, Float, Float]);

/// `BlockParticleOption.streamCodec`: a block state id.
const BLOCK_STATE: Shape = BlockState;

/// `ParticleTypes.STREAM_CODEC` in 26.2: the options of each particle type
/// that has any; the rest are `SimpleParticleType`s with none.
macro_rules! particle {
    ($item:expr) => {
        particle!($item, Struct(&[Double, Double, Double, Int, VarInt]))
    };
    ($item:expr, $trail:expr) => {
        ByRegistryOr(
            ClientRegistry::ParticleType,
            &[
                ("block", BLOCK_STATE),
                ("block_marker", BLOCK_STATE),
                // GeyserParticleOptions.java:31
                ("geyser", Int),
                ("geyser_plume", Int),
                // GeyserBaseParticleOptions.java:33
                ("geyser_base", Struct(&[Int, Float])),
                ("geyser_poof", Struct(&[Int, Float])),
                // PowerParticleOption.java:29
                ("dragon_breath", Float),
                // DustParticleOptions.java:32
                ("dust", Struct(&[Int, Float])),
                // DustColorTransitionOptions.java:32
                ("dust_color_transition", Struct(&[Int, Int, Float])),
                // SpellParticleOption.java:38
                ("effect", Struct(&[Int, Float])),
                ("instant_effect", Struct(&[Int, Float])),
                // ColorParticleOption.java:29
                ("entity_effect", Int),
                ("tinted_leaves", Int),
                ("flash", Int),
                ("falling_dust", BLOCK_STATE),
                // SculkChargeParticleOptions.java:28
                ("sculk_charge", Float),
                // ItemParticleOption.java:27
                ("item", $item),
                // VibrationParticleOption.java:33: a `PositionSource`, then ticks.
                (
                    "vibration",
                    Struct(&[
                        ByRegistry(
                            ClientRegistry::PositionSourceType,
                            &[
                                // BlockPositionSource.java:29
                                ("block", Long),
                                // EntityPositionSource.java:38
                                ("entity", Struct(&[VarInt, Float])),
                            ],
                        ),
                        VarInt,
                    ]),
                ),
                // TrailParticleOption.java:28: a `Vec3`, colour and duration.
                ("trail", $trail),
                // ShriekParticleOption.java:28
                ("shriek", VarInt),
                ("dust_pillar", BLOCK_STATE),
                ("block_crumble", BLOCK_STATE),
            ],
            &Unit,
        )
    };
}

pub static PARTICLE: Shape = particle!(ItemTemplate);
/// 1.21.11 ItemParticleOption.java:20: `ItemStack.STREAM_CODEC` instead of a template.
pub static PARTICLE_1_21_11: Shape = particle!(Stack);
/// 1.21.2/1.21.3 TargetColorParticleOption: `trail` without duration.
// VANILLA-UNSURE: TrailParticle.Provider (1.21.3) picks lifetime 10..49 at random;
// the canonical duration is fixed at 30 (the mean).
pub static PARTICLE_1_21_3: Shape = particle!(Stack, Extend(&[Double, Double, Double, Int], &[30]));

/// The 26.2 value layout of each `DataComponents` entry, by registry name.
/// Components without a network codec fall back to their persistent Codec as
/// NBT (`DataComponentType.Builder.build`): a compound wherever that Codec
/// only parses one.
static COMPONENTS_26_2: &[(&str, Shape)] = &[
    ("custom_data", Compound),
    ("max_stack_size", VarInt),
    ("max_damage", VarInt),
    ("damage", VarInt),
    ("repair_cost", VarInt),
    ("enchantable", VarInt),
    ("additional_trade_cost", VarInt),
    ("map_id", VarInt),
    ("ominous_bottle_amplifier", VarInt),
    ("unbreakable", Unit),
    ("creative_slot_lock", Unit),
    ("glider", Unit),
    // UseEffects.java:24
    ("use_effects", Struct(&[Bool, Bool, Float])),
    ("custom_name", TEXT),
    ("item_name", TEXT),
    ("minimum_attack_charge", Float),
    ("potion_duration_scale", Float),
    ("damage_type", VarInt),
    ("item_model", IDENTIFIER),
    ("tooltip_style", IDENTIFIER),
    ("note_block_sound", IDENTIFIER),
    // ItemLore.java:34
    ("lore", List(256, &TEXT)),
    ("rarity", VarInt),
    ("dye", VarInt),
    ("map_post_processing", VarInt),
    ("base_color", VarInt),
    // ItemEnchantments.java:44
    ("enchantments", Map(&VarInt, &VarInt)),
    ("stored_enchantments", Map(&VarInt, &VarInt)),
    // AdventureModePredicate.java:37
    ("can_place_on", List(UNBOUNDED, &BLOCK_PREDICATE)),
    ("can_break", List(UNBOUNDED, &BLOCK_PREDICATE)),
    // ItemAttributeModifiers.java:56,132,145; AttributeModifier.java:30
    (
        "attribute_modifiers",
        List(
            UNBOUNDED,
            &Struct(&[
                RegistryId(ClientRegistry::Attribute),
                Struct(&[IDENTIFIER, Double, VarInt]),
                VarInt,
                ById(&[Unit, Unit, TEXT]),
            ]),
        ),
    ),
    // CustomModelData.java:28
    (
        "custom_model_data",
        Struct(&[
            List(UNBOUNDED, &Float),
            List(UNBOUNDED, &Bool),
            List(UNBOUNDED, &STRING),
            List(UNBOUNDED, &Int),
        ]),
    ),
    // TooltipDisplay.java:30
    ("tooltip_display", Struct(&[Bool, List(UNBOUNDED, &VarInt)])),
    ("enchantment_glint_override", Bool),
    ("intangible_projectile", Compound),
    // FoodProperties.java:35
    ("food", Struct(&[VarInt, Float, Bool])),
    ("consumable", consumable!(&CONSUME_EFFECT)),
    // UseRemainder.java:17
    ("use_remainder", ItemTemplate),
    ("sulfur_cube_content", ItemTemplate),
    // UseCooldown.java:28
    ("use_cooldown", Struct(&[Float, Optional(&IDENTIFIER)])),
    ("damage_resistant", HolderSet),
    ("repairable", RegistrySet(ClientRegistry::Item)),
    ("provides_banner_patterns", HolderSet),
    // Tool.java:30,50
    (
        "tool",
        Struct(&[
            List(
                UNBOUNDED,
                &Struct(&[
                    RegistrySet(ClientRegistry::Block),
                    Optional(&Float),
                    Optional(&Bool),
                ]),
            ),
            Float,
            VarInt,
            Bool,
        ]),
    ),
    // Weapon.java:24
    ("weapon", Struct(&[VarInt, Float])),
    // AttackRange.java:42
    (
        "attack_range",
        Struct(&[Float, Float, Float, Float, Float, Float]),
    ),
    // Equippable.java:48
    (
        "equippable",
        Struct(&[
            VarInt,
            Ref(&SOUND),
            Optional(&IDENTIFIER),
            Optional(&IDENTIFIER),
            Optional(&RegistrySet(ClientRegistry::EntityType)),
            Bool,
            Bool,
            Bool,
            Bool,
            Bool,
            Ref(&SOUND),
        ]),
    ),
    // DeathProtection.java:30
    ("death_protection", List(UNBOUNDED, &CONSUME_EFFECT)),
    // BlocksAttacks.java:42,96,109
    (
        "blocks_attacks",
        Struct(&[
            Float,
            Float,
            List(
                UNBOUNDED,
                &Struct(&[Float, Optional(&HolderSet), Float, Float]),
            ),
            Struct(&[Float, Float, Float]),
            Optional(&HolderSet),
            Optional(&SOUND),
            Optional(&SOUND),
        ]),
    ),
    // PiercingWeapon.java:39
    (
        "piercing_weapon",
        Struct(&[Bool, Bool, Optional(&SOUND), Optional(&SOUND)]),
    ),
    // KineticWeapon.java:47,110
    (
        "kinetic_weapon",
        Struct(&[
            VarInt,
            VarInt,
            Optional(&KINETIC_CONDITION),
            Optional(&KINETIC_CONDITION),
            Optional(&KINETIC_CONDITION),
            Float,
            Float,
            Optional(&SOUND),
            Optional(&SOUND),
        ]),
    ),
    // SwingAnimation.java:26
    ("swing_animation", Struct(&[VarInt, VarInt])),
    ("dyed_color", Int),
    ("map_color", Int),
    ("map_decorations", Compound),
    ("debug_stick_state", Compound),
    ("recipes", Nbt),
    ("lock", Compound),
    ("container_loot", Compound),
    // ChargedProjectiles.java:38
    ("charged_projectiles", List(1024, &ItemTemplate)),
    // BundleContents.java:42
    ("bundle_contents", List(UNBOUNDED, &ItemTemplate)),
    // PotionContents.java:66
    (
        "potion_contents",
        Struct(&[
            Optional(&VarInt),
            Optional(&Int),
            List(UNBOUNDED, &MOB_EFFECT_INSTANCE),
            Optional(&STRING),
        ]),
    ),
    // SuspiciousStewEffects.java:44,70
    (
        "suspicious_stew_effects",
        List(UNBOUNDED, &Struct(&[VarInt, VarInt])),
    ),
    // WritableBookContent.java:33
    (
        "writable_book_content",
        List(100, &Struct(&[Utf(1024), Optional(&Utf(1024))])),
    ),
    // WrittenBookContent.java:65
    (
        "written_book_content",
        Struct(&[
            Struct(&[Utf(32), Optional(&Utf(32))]),
            STRING,
            VarInt,
            List(UNBOUNDED, &Struct(&[TEXT, Optional(&TEXT)])),
            Bool,
        ]),
    ),
    ("trim", trim!(&TRIM_MATERIAL)),
    ("provides_trim_material", Ref(&TRIM_MATERIAL)),
    ("entity_data", Ref(&ENTITY_DATA)),
    ("block_entity_data", Ref(&BLOCK_ENTITY_DATA)),
    // CustomData.java:26
    ("bucket_entity_data", Compound),
    // InstrumentComponent.java:27; Instrument.java:29
    (
        "instrument",
        Holder(&Struct(&[Ref(&SOUND), Float, Float, TEXT])),
    ),
    // JukeboxPlayable.java:40; JukeboxSong.java:34
    (
        "jukebox_playable",
        Holder(&Struct(&[Ref(&SOUND), TEXT, Float, VarInt])),
    ),
    // LodestoneTracker.java:28; GlobalPos.java:29
    (
        "lodestone_tracker",
        Struct(&[Optional(&Struct(&[IDENTIFIER, Long])), Bool]),
    ),
    ("firework_explosion", Ref(&FIREWORK_EXPLOSION)),
    // Fireworks.java:37
    (
        "fireworks",
        Struct(&[VarInt, List(256, &FIREWORK_EXPLOSION)]),
    ),
    ("profile", Ref(&RESOLVABLE_PROFILE)),
    // BannerPatternLayers.java:47,62; BannerPattern.java:26
    (
        "banner_patterns",
        List(
            UNBOUNDED,
            &Struct(&[Holder(&Struct(&[IDENTIFIER, STRING])), VarInt]),
        ),
    ),
    // PotDecorations.java:33
    ("pot_decorations", List(4, &VarInt)),
    // ItemContainerContents.java:42
    ("container", List(256, &Optional(&ItemTemplate))),
    // BlockItemStateProperties.java:37
    ("block_state", Map(&STRING, &STRING)),
    // Bees.java:26; BeehiveBlockEntity.java:310
    (
        "bees",
        List(UNBOUNDED, &Struct(&[Ref(&ENTITY_DATA), VarInt, VarInt])),
    ),
    ("break_sound", Ref(&SOUND)),
    // Registry-holder variants (`holderRegistry`) and `idMapper` enums.
    ("villager/variant", VarInt),
    ("wolf/variant", VarInt),
    ("wolf/sound_variant", VarInt),
    ("wolf/collar", VarInt),
    ("fox/variant", VarInt),
    ("salmon/size", VarInt),
    ("parrot/variant", VarInt),
    ("tropical_fish/pattern", VarInt),
    ("tropical_fish/base_color", VarInt),
    ("tropical_fish/pattern_color", VarInt),
    ("mooshroom/variant", VarInt),
    ("rabbit/variant", VarInt),
    ("pig/variant", VarInt),
    ("pig/sound_variant", VarInt),
    ("cow/variant", VarInt),
    ("cow/sound_variant", VarInt),
    ("chicken/variant", VarInt),
    ("chicken/sound_variant", VarInt),
    ("zombie_nautilus/variant", VarInt),
    ("frog/variant", VarInt),
    ("horse/variant", VarInt),
    ("llama/variant", VarInt),
    ("axolotl/variant", VarInt),
    ("cat/variant", VarInt),
    ("cat/sound_variant", VarInt),
    ("cat/collar", VarInt),
    ("sheep/color", VarInt),
    ("shulker/color", VarInt),
    // PaintingVariant.java:30,32
    (
        "painting/variant",
        Holder(&Struct(&[
            VarInt,
            VarInt,
            IDENTIFIER,
            Optional(&TEXT),
            Optional(&TEXT),
        ])),
    ),
];

/// 26.3's components that differ from 26.2's, layered over
/// [`COMPONENTS_26_2`]: the four whose layout changed, then the added ones.
static COMPONENTS_26_3: &[(&str, Shape)] = &[
    ("consumable", consumable!(&CONSUME_EFFECT_26_3)),
    ("death_protection", List(UNBOUNDED, &CONSUME_EFFECT_26_3)),
    ("trim", trim!(&TRIM_MATERIAL_26_3)),
    ("provides_trim_material", Ref(&TRIM_MATERIAL_26_3)),
    // Instrument.java:29 added `durabilityDamage`.
    (
        "instrument",
        Holder(&Struct(&[Ref(&SOUND), Float, Float, VarInt, TEXT])),
    ),
    // PotDecorations.java:37: back, left, right, front.
    (
        "pot_decorations",
        Struct(&[
            Optional(&ItemTemplate),
            Optional(&ItemTemplate),
            Optional(&ItemTemplate),
            Optional(&ItemTemplate),
        ]),
    ),
    // SwingAnimation.java:26
    ("attack_animation", Struct(&[VarInt, VarInt])),
    ("interact_animation", Struct(&[VarInt, VarInt])),
    // BlockTransformer.java:66, a registry holder.
    ("block_transformer", VarInt),
    // VillagerFood.java:24
    ("villager_food", VarInt),
    // Compostable.java:25; ResolvableInt: a constant or a reference.
    ("compostable", Either(&Int, &IDENTIFIER)),
    // CookingFuel.java:27, BrewingFuel.java:27: a ResolvableInt, then a
    // ResolvableFloat.
    (
        "cooking_fuel",
        Struct(&[Either(&Int, &IDENTIFIER), Either(&Float, &IDENTIFIER)]),
    ),
    (
        "brewing_fuel",
        Struct(&[Either(&Int, &IDENTIFIER), Either(&Float, &IDENTIFIER)]),
    ),
    // MobVisibility.java:29
    (
        "mob_visibility",
        Struct(&[RegistrySet(ClientRegistry::EntityType), Float]),
    ),
    // DecoratedPotPatterns.java:46, a registry holder.
    ("provides_pottery_pattern", VarInt),
    ("sign_text_front", Ref(&SIGN_TEXT)),
    ("sign_text_back", Ref(&SIGN_TEXT)),
    ("waxed", Unit),
    // DyeColor.java:172
    ("cushion/color", VarInt),
];

/// 1.21.11's components whose layout differs from 26.2's, layered over
/// [`COMPONENTS_26_2`] (source side of the 1.21.11 → canon translation).
static COMPONENTS_1_21_11: &[(&str, Shape)] = &[
    // EitherHolder.streamCodec (1.21.11): a holder, or a registry key.
    ("damage_type", Either(&VarInt, &IDENTIFIER)),
    ("chicken/variant", Either(&VarInt, &IDENTIFIER)),
    ("zombie_nautilus/variant", Either(&VarInt, &IDENTIFIER)),
    (
        "provides_trim_material",
        Either(&TRIM_MATERIAL, &IDENTIFIER),
    ),
    (
        "instrument",
        Either(
            &Holder(&Struct(&[Ref(&SOUND), Float, Float, TEXT])),
            &IDENTIFIER,
        ),
    ),
    (
        "jukebox_playable",
        Either(
            &Holder(&Struct(&[Ref(&SOUND), TEXT, Float, VarInt])),
            &IDENTIFIER,
        ),
    ),
    // TagKey.streamCodec (1.21.11) where 26.x has a holder set.
    ("provides_banner_patterns", TagKey),
    ("damage_resistant", TagKey),
    (
        "blocks_attacks",
        Struct(&[
            Float,
            Float,
            List(
                UNBOUNDED,
                &Struct(&[Float, Optional(&HolderSet), Float, Float]),
            ),
            Struct(&[Float, Float, Float]),
            Optional(&TagKey),
            Optional(&SOUND),
            Optional(&SOUND),
        ]),
    ),
    // ItemStack codecs (1.21.11) where 26.x has ItemStackTemplate.
    ("charged_projectiles", List(UNBOUNDED, &Stack)),
    ("bundle_contents", List(UNBOUNDED, &Stack)),
    ("use_remainder", Stack),
    ("container", List(256, &Stack)),
    // PotDecorations.java:30 (1.21.11): up to four item ids, brick = empty side.
    (
        "pot_decorations",
        List(4, &RegistryId(ClientRegistry::Item)),
    ),
    // TeleportRandomlyConsumeEffect: 26.3 added `directionalParticles` (default true).
    ("consumable", consumable!(&CONSUME_EFFECT_1_21_11)),
    ("death_protection", List(UNBOUNDED, &CONSUME_EFFECT_1_21_11)),
];

static CONSUME_EFFECT_1_21_11: Shape = consume_effect!(Append(&Float, &[1]));

/// 1.21.10's components whose layout differs from 1.21.11's, layered over
/// [`COMPONENTS_1_21_11`] and [`COMPONENTS_26_2`].
static COMPONENTS_1_21_10: &[(&str, Shape)] = &[
    // AdventureModePredicate (1.21.10): BlockPredicate with the bare predicate type id.
    ("can_place_on", List(UNBOUNDED, &BLOCK_PREDICATE_1_21_10)),
    ("can_break", List(UNBOUNDED, &BLOCK_PREDICATE_1_21_10)),
];

/// `ResolvableProfile.STREAM_CODEC` before 1.21.9: optional name, optional
/// id, properties — now the `Partial` branch, followed by an empty skin patch.
static RESOLVABLE_PROFILE_1_21_8: Shape = Append(
    &Right(&Struct(&[
        Optional(&Utf(16)),
        Optional(&Uuid),
        Ref(&PROFILE_PROPERTIES),
    ])),
    &[0, 0, 0, 0],
);

/// Codecs changed in 1.21.9 (`TypedEntityData`, `ResolvableProfile`).
static COMPONENTS_1_21_8: &[(&str, Shape)] = &[
    // CustomData.STREAM_CODEC (1.21.8): the type is the compound's `id`.
    ("entity_data", IdInNbt(ClientRegistry::EntityType)),
    (
        "block_entity_data",
        IdInNbt(ClientRegistry::BlockEntityType),
    ),
    // BeehiveBlockEntity.Occupant (1.21.8): CustomData, ticks in hive, min ticks.
    (
        "bees",
        List(
            UNBOUNDED,
            &Struct(&[IdInNbt(ClientRegistry::EntityType), VarInt, VarInt]),
        ),
    ),
    ("profile", Ref(&RESOLVABLE_PROFILE_1_21_8)),
];

/// Codecs changed in 1.21.6 (`Equippable` shearing, attribute modifier `Display`).
static COMPONENTS_1_21_5: &[(&str, Shape)] = &[
    // ItemAttributeModifiers.Entry (1.21.5): attribute, modifier, slot group; 1.21.6
    // appended Display (Default = id 0).
    ("attribute_modifiers", Ref(&ATTRIBUTE_MODIFIERS_1_21_5)),
    // Equippable (1.21.5): no canBeSheared (default false) / shearingSound (default
    // SoundEvents.SHEARS_SNIP, appended as a direct holder by location).
    (
        "equippable",
        Extend(
            &[
                VarInt,
                Ref(&SOUND),
                Optional(&IDENTIFIER),
                Optional(&IDENTIFIER),
                Optional(&RegistrySet(ClientRegistry::EntityType)),
                Bool,
                Bool,
                Bool,
                Bool,
            ],
            b"\x00\x00\x1aminecraft:item.shears.snip\x00",
        ),
    ),
];

static ATTRIBUTE_MODIFIERS_1_21_5: Shape = List(
    UNBOUNDED,
    &Extend(
        &[
            RegistryId(ClientRegistry::Attribute),
            Struct(&[IDENTIFIER, Double, VarInt]),
            VarInt,
        ],
        &[0],
    ),
);

/// `TrimMaterial.STREAM_CODEC` (1.21.4): asset name, ingredient item, asset
/// overrides, description. 1.21.5 dropped the ingredient, so an inline material
/// does not translate (registry references do).
static TRIM_MATERIAL_1_21_4: Shape = Holder(&Struct(&[
    STRING,
    RegistryId(ClientRegistry::Item),
    Map(&IDENTIFIER, &STRING),
    TEXT,
]));

/// `TrimPattern.STREAM_CODEC` (1.21.4): asset id, template item, description, decal.
static TRIM_PATTERN_1_21_4: Shape = Holder(&Struct(&[
    IDENTIFIER,
    RegistryId(ClientRegistry::Item),
    TEXT,
    Bool,
]));

/// 1.21.4 → 1.21.5: `showInTooltip` booleans moved into `tooltip_display`
/// (dropped here, default true on the way back), `hide_tooltip` /
/// `hide_additional_tooltip` have no 1.21.5 counterpart (not translated).
// VANILLA: ItemEnchantments, ItemAttributeModifiers, AdventureModePredicate,
// ArmorTrim, DyedItemColor, JukeboxPlayable, Unbreakable, Tool, Equippable,
// DataComponents.INSTRUMENT (1.21.4).
// VANILLA-UNSURE: hidden-in-tooltip flags are not mapped to tooltip_display.
static COMPONENTS_1_21_4: &[(&str, Shape)] = &[
    (
        "enchantments",
        Trailing(&Map(&VarInt, &VarInt), &Bool, &[1]),
    ),
    (
        "stored_enchantments",
        Trailing(&Map(&VarInt, &VarInt), &Bool, &[1]),
    ),
    (
        "attribute_modifiers",
        Trailing(&ATTRIBUTE_MODIFIERS_1_21_5, &Bool, &[1]),
    ),
    (
        "can_place_on",
        Trailing(&List(UNBOUNDED, &BLOCK_PREDICATE_1_21_4), &Bool, &[1]),
    ),
    (
        "can_break",
        Trailing(&List(UNBOUNDED, &BLOCK_PREDICATE_1_21_4), &Bool, &[1]),
    ),
    (
        "trim",
        Trailing(
            &Struct(&[Ref(&TRIM_MATERIAL_1_21_4), Ref(&TRIM_PATTERN_1_21_4)]),
            &Bool,
            &[1],
        ),
    ),
    ("dyed_color", Trailing(&Int, &Bool, &[1])),
    (
        "jukebox_playable",
        Trailing(
            &Either(
                &Holder(&Struct(&[Ref(&SOUND), TEXT, Float, VarInt])),
                &IDENTIFIER,
            ),
            &Bool,
            &[1],
        ),
    ),
    ("unbreakable", Trailing(&Unit, &Bool, &[1])),
    ("hide_tooltip", Unit),
    ("hide_additional_tooltip", Unit),
    // Tool (1.21.4): no canDestroyBlocksInCreative (default true).
    (
        "tool",
        Extend(
            &[
                List(
                    UNBOUNDED,
                    &Struct(&[
                        RegistrySet(ClientRegistry::Block),
                        Optional(&Float),
                        Optional(&Bool),
                    ]),
                ),
                Float,
                VarInt,
            ],
            &[1],
        ),
    ),
    // Holder<Instrument> without EitherHolder.
    (
        "instrument",
        Holder(&Struct(&[Ref(&SOUND), Float, Float, TEXT])),
    ),
    // Equippable (1.21.4): also no equipOnInteract (default false).
    (
        "equippable",
        Extend(
            &[
                VarInt,
                Ref(&SOUND),
                Optional(&IDENTIFIER),
                Optional(&IDENTIFIER),
                Optional(&RegistrySet(ClientRegistry::EntityType)),
                Bool,
                Bool,
                Bool,
            ],
            b"\x00\x00\x00\x1aminecraft:item.shears.snip\x00",
        ),
    ),
];

/// `TrimMaterial.STREAM_CODEC` (1.21.2/1.21.3): asset name, ingredient item,
/// item model index, armor-material overrides, description.
static TRIM_MATERIAL_1_21_3: Shape = Holder(&Struct(&[
    STRING,
    RegistryId(ClientRegistry::Item),
    Float,
    Map(&IDENTIFIER, &STRING),
    TEXT,
]));

/// 1.21.3 → 1.21.4: `custom_model_data` was one int (1.21.4 datafixer moves it
/// into `floats`), trim materials carried `item_model_index`. Equippable `model`
/// became `asset_id` with the same encoding.
// VANILLA: CustomModelData, TrimMaterial, Equippable (1.21.3).
static COMPONENTS_1_21_3: &[(&str, Shape)] = &[
    ("custom_model_data", IntAsFloats),
    (
        "trim",
        Trailing(
            &Struct(&[Ref(&TRIM_MATERIAL_1_21_3), Ref(&TRIM_PATTERN_1_21_4)]),
            &Bool,
            &[1],
        ),
    ),
];

/// `TrimMaterial.STREAM_CODEC` (1.21/1.21.1): overrides keyed by the
/// `armor_material` registry id (1.21.2: by identifier).
static TRIM_MATERIAL_1_21_1: Shape = Holder(&Struct(&[
    STRING,
    RegistryId(ClientRegistry::Item),
    Float,
    Map(&VarInt, &STRING),
    TEXT,
]));

/// 1.21.1 → 1.21.2: food lost eat_seconds/using_converts_to/effects (moved to
/// `consumable`), `fire_resistant` became `damage_resistant` (#is_fire), potion
/// contents got `custom_name`, instruments a float duration and description.
// VANILLA: FoodProperties, PotionContents, Instrument, TrimMaterial,
// DataComponents#FIRE_RESISTANT (1.21.1).
static COMPONENTS_1_21_1: &[(&str, Shape)] = &[
    (
        "food",
        Trailing(
            &Struct(&[VarInt, Float, Bool]),
            &Struct(&[
                Float,
                Optional(&Stack),
                List(UNBOUNDED, &Struct(&[Ref(&MOB_EFFECT_INSTANCE), Float])),
            ]),
            // eat_seconds 1.6F, no using_converts_to, no effects
            &[0x3f, 0xcc, 0xcc, 0xcd, 0, 0],
        ),
    ),
    // HolderSet: 0 = tag, then the tag id.
    ("damage_resistant", Const(b"\x00\x11minecraft:is_fire")),
    (
        "potion_contents",
        Extend(
            &[
                Optional(&VarInt),
                Optional(&Int),
                List(UNBOUNDED, &MOB_EFFECT_INSTANCE),
            ],
            &[0],
        ),
    ),
    // VANILLA-UNSURE: an inline instrument (int use_duration, no description)
    // is not translated; registry ids are.
    ("instrument", Holder(&Struct(&[Ref(&SOUND), VarInt, Float]))),
    (
        "trim",
        Trailing(
            &Struct(&[Ref(&TRIM_MATERIAL_1_21_1), Ref(&TRIM_PATTERN_1_21_4)]),
            &Bool,
            &[1],
        ),
    ),
];

/// Component shapes of 1.21/1.21.1 by their own component type id (with
/// `fire_resistant` named `damage_resistant`, see `translate::names_1_21_1`).
pub(crate) fn components_1_21_1() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_1::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                if *name == "fire_resistant" {
                    "damage_resistant"
                } else {
                    name
                }
            })
            .map(|name| {
                [
                    COMPONENTS_1_21_1,
                    COMPONENTS_1_21_3,
                    COMPONENTS_1_21_4,
                    COMPONENTS_1_21_5,
                    COMPONENTS_1_21_8,
                    COMPONENTS_1_21_10,
                    COMPONENTS_1_21_11,
                    COMPONENTS_26_2,
                ]
                .iter()
                .find_map(|layer| layer.iter().find(|(n, _)| *n == name))
                .map(|(_, s)| s)
                .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.2/1.21.3 by their own component type id.
pub(crate) fn components_1_21_3() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_3::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [
                    COMPONENTS_1_21_3,
                    COMPONENTS_1_21_4,
                    COMPONENTS_1_21_5,
                    COMPONENTS_1_21_8,
                    COMPONENTS_1_21_10,
                    COMPONENTS_1_21_11,
                    COMPONENTS_26_2,
                ]
                .iter()
                .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                .map(|(_, s)| s)
                .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.4 by its own component type id.
pub(crate) fn components_1_21_4() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_4::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [
                    COMPONENTS_1_21_4,
                    COMPONENTS_1_21_5,
                    COMPONENTS_1_21_8,
                    COMPONENTS_1_21_10,
                    COMPONENTS_1_21_11,
                    COMPONENTS_26_2,
                ]
                .iter()
                .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                .map(|(_, s)| s)
                .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.5 by its own component type id.
pub(crate) fn components_1_21_5() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_5::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [
                    COMPONENTS_1_21_5,
                    COMPONENTS_1_21_8,
                    COMPONENTS_1_21_10,
                    COMPONENTS_1_21_11,
                    COMPONENTS_26_2,
                ]
                .iter()
                .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                .map(|(_, s)| s)
                .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.7/1.21.8 by its own component type id.
pub(crate) fn components_1_21_8() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_8::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [
                    COMPONENTS_1_21_8,
                    COMPONENTS_1_21_10,
                    COMPONENTS_1_21_11,
                    COMPONENTS_26_2,
                ]
                .iter()
                .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                .map(|(_, s)| s)
                .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.10 by its own component type id.
pub(crate) fn components_1_21_10() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_10::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [COMPONENTS_1_21_10, COMPONENTS_1_21_11, COMPONENTS_26_2]
                    .iter()
                    .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                    .map(|(_, s)| s)
                    .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Component shapes of 1.21.11 by its own component type id.
pub(crate) fn components_1_21_11() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        crate::generated::r1_21_11::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [COMPONENTS_1_21_11, COMPONENTS_26_2]
                    .iter()
                    .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                    .map(|(_, s)| s)
                    .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// `SignText.STREAM_CODEC` (SignText.java:43): four lines, optional filtered
/// lines, a dye color and the glowing flag; fixed-size lists carry no count.
static SIGN_TEXT: Shape = Struct(&[
    Struct(&[TEXT, TEXT, TEXT, TEXT]),
    Optional(&Struct(&[TEXT, TEXT, TEXT, TEXT])),
    VarInt,
    Bool,
]);

/// Стек или шаблон: id предмета, количество и патч компонентов.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub item: u32,
    pub count: i32,
    pub patch: ComponentPatch,
}

/// `DataComponentPatch`: добавленные (точные байты значения) и удалённые id компонентов.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentPatch {
    pub added: Vec<(u32, Box<[u8]>)>,
    pub removed: Vec<u32>,
}

fn bad(what: &'static str) -> ProtocolError {
    ProtocolError::InvalidValue { what, value: -1 }
}

/// Таблица форм компонентов 26.3 по id типа компонента.
pub(crate) fn components() -> &'static [&'static Shape] {
    static T: std::sync::LazyLock<Vec<&'static Shape>> = std::sync::LazyLock::new(|| {
        reg::DATA_COMPONENT_TYPE
            .iter()
            .map(|name| {
                [COMPONENTS_26_3, COMPONENTS_26_2]
                    .iter()
                    .find_map(|layer| layer.iter().find(|(n, _)| n == name))
                    .map(|(_, s)| s)
                    .unwrap_or(&Shape::Nbt)
            })
            .collect()
    });
    &T
}

/// Номер предмета `air`.
pub fn air_id() -> u32 {
    reg::ITEM.iter().position(|n| *n == "air").unwrap_or(0) as u32
}

/// `ItemStack.OPTIONAL_STREAM_CODEC`: количество (пусто если ≤ 0), предмет, патч.
pub fn read_stack(r: &mut Reader) -> ProtocolResult<Option<ItemStack>> {
    let count = r.var_int()?;
    if count <= 0 {
        return Ok(None);
    }
    let item = read_item(r)?;
    let patch = read_patch(r, 0)?;
    Ok(Some(ItemStack { item, count, patch }))
}

/// `ItemStackTemplate.STREAM_CODEC`: предмет, количество, патч.
pub fn read_template(r: &mut Reader) -> ProtocolResult<ItemStack> {
    read_template_at(r, 0)
}

/// `ItemCost.STREAM_CODEC`: предмет, количество, `DataComponentExactPredicate` (список компонентов).
/// VANILLA: ItemCost#itemStack = стек с патчем из предиката (asPatch).
pub fn read_item_cost(r: &mut Reader) -> ProtocolResult<ItemStack> {
    let item = read_item(r)?;
    let count = r.var_int()?.max(1);
    let n = r.length("cost components", 4096)?;
    let mut patch = ComponentPatch::default();
    for _ in 0..n {
        let id = r.var_int()? as u32;
        let start = r.pos();
        walk(r, component(id)?, 1)?;
        patch.added.push((id, r.since(start).into()));
    }
    Ok(ItemStack { item, count, patch })
}

fn read_template_at(r: &mut Reader, depth: u32) -> ProtocolResult<ItemStack> {
    let item = read_item(r)?;
    let count = r.var_int()?;
    if count == 0 || item == air_id() {
        return Err(bad("empty item template"));
    }
    let patch = read_patch(r, depth + 1)?;
    Ok(ItemStack { item, count, patch })
}

/// Канонический id модового предмета Forge 1.20.1: `MOD_ITEM_BASE + id сервера` (TODO-508);
/// имя — снимок реестра FML (`ezzmc_world::modded::server_name`).
pub const MOD_ITEM_BASE: u32 = 1 << 20;

pub fn read_item(r: &mut Reader) -> ProtocolResult<u32> {
    let item = r.var_int()? as u32;
    if (MOD_ITEM_BASE..MOD_ITEM_BASE * 2).contains(&item) {
        return Ok(item);
    }
    if ClientRegistry::Item.name_of(item).is_none() {
        return Err(bad("unknown item id"));
    }
    Ok(item)
}

fn component(id: u32) -> ProtocolResult<&'static Shape> {
    components()
        .get(id as usize)
        .copied()
        .ok_or_else(|| bad("unknown data component type"))
}

/// `ItemStack.OPTIONAL_UNTRUSTED_STREAM_CODEC` (DELIMITED: значение компонента с префиксом длины).
pub fn write_stack_untrusted(w: &mut Writer, stack: Option<&ItemStack>) {
    let Some(s) = stack else {
        w.var_int(0);
        return;
    };
    w.var_int(s.count)
        .var_int(s.item as i32)
        .var_int(s.patch.added.len() as i32)
        .var_int(s.patch.removed.len() as i32);
    for (id, v) in &s.patch.added {
        w.var_int(*id as i32).var_int(v.len() as i32).bytes(v);
    }
    for id in &s.patch.removed {
        w.var_int(*id as i32);
    }
}

/// `DataComponentPatch.STREAM_CODEC`.
pub fn read_patch(r: &mut Reader, depth: u32) -> ProtocolResult<ComponentPatch> {
    if depth > MAX_DEPTH {
        return Err(bad("item nested too deeply"));
    }
    let added_count = r.length("added components", 4096)?;
    let removed_count = r.length("removed components", 4096)?;
    let mut patch = ComponentPatch::default();
    for _ in 0..added_count {
        let id = r.var_int()? as u32;
        let start = r.pos();
        walk(r, component(id)?, depth + 1)?;
        patch.added.push((id, r.since(start).into()));
    }
    for _ in 0..removed_count {
        let id = r.var_int()? as u32;
        component(id)?;
        patch.removed.push(id);
    }
    Ok(patch)
}

fn len(r: &mut Reader, max: usize) -> ProtocolResult<usize> {
    let n = r.length("collection", max.min(1 << 20))?;
    if n > r.remaining() {
        return Err(bad("collection longer than the frame"));
    }
    Ok(n)
}

/// Проверяет и пропускает значение формы `shape`.
pub fn walk(r: &mut Reader, shape: &'static Shape, depth: u32) -> ProtocolResult<()> {
    use Shape::*;
    if depth > MAX_DEPTH {
        return Err(bad("component nested too deeply"));
    }
    let depth = depth + 1;
    match shape {
        Unit => {}
        Bool => {
            r.bool()?;
        }
        Int | Float => {
            r.bytes(4)?;
        }
        Long | Double => {
            r.bytes(8)?;
        }
        VarInt => {
            r.var_int()?;
        }
        Utf(max) => {
            r.string(*max)?;
        }
        Uuid => {
            r.uuid()?;
        }
        Nbt => {
            if nbt::read_network(r)?.is_none() {
                return Err(bad("expected a non-empty NBT tag"));
            }
        }
        Compound | IdInNbt(_) => match nbt::read_network(r)? {
            Some(t) if t.id() == 10 => {}
            _ => return Err(bad("expected an NBT compound")),
        },
        Optional(inner) => {
            if r.bool()? {
                walk(r, inner, depth)?;
            }
        }
        List(max, inner) => {
            for _ in 0..len(r, *max)? {
                walk(r, inner, depth)?;
            }
        }
        Map(key, value) => {
            for _ in 0..len(r, UNBOUNDED)? {
                walk(r, key, depth)?;
                walk(r, value, depth)?;
            }
        }
        Either(left, right) => {
            let inner = if r.bool()? { left } else { right };
            walk(r, inner, depth)?;
        }
        Struct(fields) => {
            for field in *fields {
                walk(r, field, depth)?;
            }
        }
        Ref(inner) => walk(r, inner, depth)?,
        Holder(inline) | RegistryHolder(_, inline) => {
            if r.var_int()? == 0 {
                walk(r, inline, depth)?;
            }
        }
        RegistryId(_) | BlockState => {
            r.var_int()?;
        }
        Stack => {
            let count = r.var_int()?;
            if count > 0 {
                r.var_int()?;
                crate::translate::skip_patch_legacy(r, depth)?;
            }
        }
        TagKey => {
            r.string(MAX_STRING_LENGTH)?;
        }
        Append(inner, _) | Left(inner) | Right(inner) => walk(r, inner, depth)?,
        IntAsFloats => {
            r.var_int()?;
        }
        Const(_) => {}
        Trailing(inner, skipped, _) => {
            walk(r, inner, depth)?;
            walk(r, skipped, depth)?;
        }
        Extend(fields, _) => {
            for f in *fields {
                walk(r, f, depth)?;
            }
        }
        HolderSet | RegistrySet(_) => match r.var_int()? {
            0 => {
                r.string(MAX_STRING_LENGTH)?;
            }
            n => {
                let n = (n as usize).saturating_sub(1);
                if n > r.remaining() {
                    return Err(bad("holder set longer than the frame"));
                }
                for _ in 0..n {
                    r.var_int()?;
                }
            }
        },
        ById(arms) => {
            let id = r.var_int()? as usize;
            walk(r, arms.get(id).unwrap_or(&arms[0]), depth)?;
        }
        ByRegistry(registry, arms) => {
            let inner = dispatch(r, *registry, arms)?;
            walk(
                r,
                inner.ok_or_else(|| bad("unported dispatch type"))?,
                depth,
            )?;
        }
        ByRegistryOr(registry, arms, rest) => {
            let inner = dispatch(r, *registry, arms)?;
            walk(r, inner.unwrap_or(rest), depth)?;
        }
        ItemTemplate => {
            read_template_at(r, depth)?;
        }
        TypedComponent => {
            let id = r.var_int()? as u32;
            walk(r, component(id)?, depth)?;
        }
    }
    Ok(())
}

fn dispatch(
    r: &mut Reader,
    registry: ClientRegistry,
    arms: &'static [(&'static str, Shape)],
) -> ProtocolResult<Option<&'static Shape>> {
    let id = r.var_int()? as u32;
    let name = registry
        .name_of(id)
        .ok_or_else(|| bad("unknown dispatch type"))?;
    Ok(arms.iter().find(|(arm, _)| *arm == name).map(|(_, s)| s))
}

/// `ItemStack.OPTIONAL_STREAM_CODEC`.
pub fn write_stack(w: &mut Writer, stack: Option<&ItemStack>) {
    let Some(s) = stack else {
        w.var_int(0);
        return;
    };
    w.var_int(s.count).var_int(s.item as i32);
    write_patch(w, &s.patch);
}

/// `DataComponentPatch.STREAM_CODEC`.
pub fn write_patch(w: &mut Writer, p: &ComponentPatch) {
    w.var_int(p.added.len() as i32)
        .var_int(p.removed.len() as i32);
    for (id, v) in &p.added {
        w.var_int(*id as i32).bytes(v);
    }
    for id in &p.removed {
        w.var_int(*id as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comp(name: &str) -> i32 {
        reg::DATA_COMPONENT_TYPE
            .iter()
            .position(|n| *n == name)
            .unwrap_or(0) as i32
    }

    #[test]
    #[allow(clippy::unwrap_used)] // тест на собственных буферах: unwrap = assert
    fn stack_roundtrip_and_framing() {
        let item = reg::ITEM
            .iter()
            .position(|n| *n == "diamond_pickaxe")
            .unwrap_or(1) as i32;
        let mut w = Writer::new();
        w.var_int(1).var_int(item).var_int(2).var_int(1);
        // damage=7 (VarInt); custom_name = строковый NBT-тег "x"; удалён max_stack_size
        w.var_int(comp("damage")).var_int(7);
        w.var_int(comp("custom_name")).u8(8).i16(1).u8(b'x');
        w.var_int(comp("max_stack_size"));
        w.u8(0xAB); // хвост — не должен быть съеден
        let buf = w.into_vec();
        let mut r = Reader::new(&buf);
        let s = read_stack(&mut r).unwrap().unwrap();
        assert_eq!(r.remaining(), 1);
        assert_eq!((s.count, s.item as i32), (1, item));
        assert_eq!(s.patch.added.len(), 2);
        assert_eq!(s.patch.removed, vec![comp("max_stack_size") as u32]);
        let mut o = Writer::new();
        write_stack(&mut o, Some(&s));
        assert_eq!(o.into_vec(), buf[..buf.len() - 1]);
        let mut r = Reader::new(&[0]);
        assert_eq!(read_stack(&mut r).unwrap(), None);
    }
}
