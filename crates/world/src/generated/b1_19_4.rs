// @generated `cargo xtask gen-blockstates 1.19.4` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.19.4";
pub const STATE_COUNT: u32 = 23725;

/// Индекс = id блока в реестре `minecraft:block` (порядок по первому id состояния).
pub static BLOCKS: &[BlockInfo] = &[
    BlockInfo {
        name: "air",
        first_state: 0,
        state_count: 1,
        default_state: 0,
        props: &[],
    },
    BlockInfo {
        name: "stone",
        first_state: 1,
        state_count: 1,
        default_state: 1,
        props: &[],
    },
    BlockInfo {
        name: "granite",
        first_state: 2,
        state_count: 1,
        default_state: 2,
        props: &[],
    },
    BlockInfo {
        name: "polished_granite",
        first_state: 3,
        state_count: 1,
        default_state: 3,
        props: &[],
    },
    BlockInfo {
        name: "diorite",
        first_state: 4,
        state_count: 1,
        default_state: 4,
        props: &[],
    },
    BlockInfo {
        name: "polished_diorite",
        first_state: 5,
        state_count: 1,
        default_state: 5,
        props: &[],
    },
    BlockInfo {
        name: "andesite",
        first_state: 6,
        state_count: 1,
        default_state: 6,
        props: &[],
    },
    BlockInfo {
        name: "polished_andesite",
        first_state: 7,
        state_count: 1,
        default_state: 7,
        props: &[],
    },
    BlockInfo {
        name: "grass_block",
        first_state: 8,
        state_count: 2,
        default_state: 9,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dirt",
        first_state: 10,
        state_count: 1,
        default_state: 10,
        props: &[],
    },
    BlockInfo {
        name: "coarse_dirt",
        first_state: 11,
        state_count: 1,
        default_state: 11,
        props: &[],
    },
    BlockInfo {
        name: "podzol",
        first_state: 12,
        state_count: 2,
        default_state: 13,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cobblestone",
        first_state: 14,
        state_count: 1,
        default_state: 14,
        props: &[],
    },
    BlockInfo {
        name: "oak_planks",
        first_state: 15,
        state_count: 1,
        default_state: 15,
        props: &[],
    },
    BlockInfo {
        name: "spruce_planks",
        first_state: 16,
        state_count: 1,
        default_state: 16,
        props: &[],
    },
    BlockInfo {
        name: "birch_planks",
        first_state: 17,
        state_count: 1,
        default_state: 17,
        props: &[],
    },
    BlockInfo {
        name: "jungle_planks",
        first_state: 18,
        state_count: 1,
        default_state: 18,
        props: &[],
    },
    BlockInfo {
        name: "acacia_planks",
        first_state: 19,
        state_count: 1,
        default_state: 19,
        props: &[],
    },
    BlockInfo {
        name: "cherry_planks",
        first_state: 20,
        state_count: 1,
        default_state: 20,
        props: &[],
    },
    BlockInfo {
        name: "dark_oak_planks",
        first_state: 21,
        state_count: 1,
        default_state: 21,
        props: &[],
    },
    BlockInfo {
        name: "mangrove_planks",
        first_state: 22,
        state_count: 1,
        default_state: 22,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_planks",
        first_state: 23,
        state_count: 1,
        default_state: 23,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_mosaic",
        first_state: 24,
        state_count: 1,
        default_state: 24,
        props: &[],
    },
    BlockInfo {
        name: "oak_sapling",
        first_state: 25,
        state_count: 2,
        default_state: 25,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "spruce_sapling",
        first_state: 27,
        state_count: 2,
        default_state: 27,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "birch_sapling",
        first_state: 29,
        state_count: 2,
        default_state: 29,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "jungle_sapling",
        first_state: 31,
        state_count: 2,
        default_state: 31,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "acacia_sapling",
        first_state: 33,
        state_count: 2,
        default_state: 33,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "cherry_sapling",
        first_state: 35,
        state_count: 2,
        default_state: 35,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "dark_oak_sapling",
        first_state: 37,
        state_count: 2,
        default_state: 37,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "mangrove_propagule",
        first_state: 39,
        state_count: 40,
        default_state: 44,
        props: &[
            PropInfo {
                name: "age",
                values: &["0", "1", "2", "3", "4"],
            },
            PropInfo {
                name: "hanging",
                values: &["true", "false"],
            },
            PropInfo {
                name: "stage",
                values: &["0", "1"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bedrock",
        first_state: 79,
        state_count: 1,
        default_state: 79,
        props: &[],
    },
    BlockInfo {
        name: "water",
        first_state: 80,
        state_count: 16,
        default_state: 80,
        props: &[PropInfo {
            name: "level",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "lava",
        first_state: 96,
        state_count: 16,
        default_state: 96,
        props: &[PropInfo {
            name: "level",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "sand",
        first_state: 112,
        state_count: 1,
        default_state: 112,
        props: &[],
    },
    BlockInfo {
        name: "suspicious_sand",
        first_state: 113,
        state_count: 4,
        default_state: 113,
        props: &[PropInfo {
            name: "dusted",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "red_sand",
        first_state: 117,
        state_count: 1,
        default_state: 117,
        props: &[],
    },
    BlockInfo {
        name: "gravel",
        first_state: 118,
        state_count: 1,
        default_state: 118,
        props: &[],
    },
    BlockInfo {
        name: "gold_ore",
        first_state: 119,
        state_count: 1,
        default_state: 119,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_gold_ore",
        first_state: 120,
        state_count: 1,
        default_state: 120,
        props: &[],
    },
    BlockInfo {
        name: "iron_ore",
        first_state: 121,
        state_count: 1,
        default_state: 121,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_iron_ore",
        first_state: 122,
        state_count: 1,
        default_state: 122,
        props: &[],
    },
    BlockInfo {
        name: "coal_ore",
        first_state: 123,
        state_count: 1,
        default_state: 123,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_coal_ore",
        first_state: 124,
        state_count: 1,
        default_state: 124,
        props: &[],
    },
    BlockInfo {
        name: "nether_gold_ore",
        first_state: 125,
        state_count: 1,
        default_state: 125,
        props: &[],
    },
    BlockInfo {
        name: "oak_log",
        first_state: 126,
        state_count: 3,
        default_state: 127,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 129,
        state_count: 3,
        default_state: 130,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 132,
        state_count: 3,
        default_state: 133,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 135,
        state_count: 3,
        default_state: 136,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 138,
        state_count: 3,
        default_state: 139,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cherry_log",
        first_state: 141,
        state_count: 3,
        default_state: 142,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 144,
        state_count: 3,
        default_state: 145,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_log",
        first_state: 147,
        state_count: 3,
        default_state: 148,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_roots",
        first_state: 150,
        state_count: 2,
        default_state: 151,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "muddy_mangrove_roots",
        first_state: 152,
        state_count: 3,
        default_state: 153,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "bamboo_block",
        first_state: 155,
        state_count: 3,
        default_state: 156,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 158,
        state_count: 3,
        default_state: 159,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 161,
        state_count: 3,
        default_state: 162,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 164,
        state_count: 3,
        default_state: 165,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 167,
        state_count: 3,
        default_state: 168,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_cherry_log",
        first_state: 170,
        state_count: 3,
        default_state: 171,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 173,
        state_count: 3,
        default_state: 174,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 176,
        state_count: 3,
        default_state: 177,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_log",
        first_state: 179,
        state_count: 3,
        default_state: 180,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_bamboo_block",
        first_state: 182,
        state_count: 3,
        default_state: 183,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 185,
        state_count: 3,
        default_state: 186,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 188,
        state_count: 3,
        default_state: 189,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 191,
        state_count: 3,
        default_state: 192,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 194,
        state_count: 3,
        default_state: 195,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 197,
        state_count: 3,
        default_state: 198,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cherry_wood",
        first_state: 200,
        state_count: 3,
        default_state: 201,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 203,
        state_count: 3,
        default_state: 204,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_wood",
        first_state: 206,
        state_count: 3,
        default_state: 207,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 209,
        state_count: 3,
        default_state: 210,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 212,
        state_count: 3,
        default_state: 213,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 215,
        state_count: 3,
        default_state: 216,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 218,
        state_count: 3,
        default_state: 219,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 221,
        state_count: 3,
        default_state: 222,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_cherry_wood",
        first_state: 224,
        state_count: 3,
        default_state: 225,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 227,
        state_count: 3,
        default_state: 228,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_wood",
        first_state: 230,
        state_count: 3,
        default_state: 231,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 233,
        state_count: 28,
        default_state: 260,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_leaves",
        first_state: 261,
        state_count: 28,
        default_state: 288,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_leaves",
        first_state: 289,
        state_count: 28,
        default_state: 316,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_leaves",
        first_state: 317,
        state_count: 28,
        default_state: 344,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_leaves",
        first_state: 345,
        state_count: 28,
        default_state: 372,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_leaves",
        first_state: 373,
        state_count: 28,
        default_state: 400,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_leaves",
        first_state: 401,
        state_count: 28,
        default_state: 428,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_leaves",
        first_state: 429,
        state_count: 28,
        default_state: 456,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "azalea_leaves",
        first_state: 457,
        state_count: 28,
        default_state: 484,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "flowering_azalea_leaves",
        first_state: 485,
        state_count: 28,
        default_state: 512,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sponge",
        first_state: 513,
        state_count: 1,
        default_state: 513,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 514,
        state_count: 1,
        default_state: 514,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 515,
        state_count: 1,
        default_state: 515,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 516,
        state_count: 1,
        default_state: 516,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_lapis_ore",
        first_state: 517,
        state_count: 1,
        default_state: 517,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 518,
        state_count: 1,
        default_state: 518,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 519,
        state_count: 12,
        default_state: 520,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "triggered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sandstone",
        first_state: 531,
        state_count: 1,
        default_state: 531,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 532,
        state_count: 1,
        default_state: 532,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 533,
        state_count: 1,
        default_state: 533,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 534,
        state_count: 1150,
        default_state: 535,
        props: &[
            PropInfo {
                name: "instrument",
                values: &[
                    "harp",
                    "basedrum",
                    "snare",
                    "hat",
                    "bass",
                    "flute",
                    "bell",
                    "guitar",
                    "chime",
                    "xylophone",
                    "iron_xylophone",
                    "cow_bell",
                    "didgeridoo",
                    "bit",
                    "banjo",
                    "pling",
                    "zombie",
                    "skeleton",
                    "creeper",
                    "dragon",
                    "wither_skeleton",
                    "piglin",
                    "custom_head",
                ],
            },
            PropInfo {
                name: "note",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24",
                ],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "white_bed",
        first_state: 1684,
        state_count: 16,
        default_state: 1687,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "orange_bed",
        first_state: 1700,
        state_count: 16,
        default_state: 1703,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "magenta_bed",
        first_state: 1716,
        state_count: 16,
        default_state: 1719,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "light_blue_bed",
        first_state: 1732,
        state_count: 16,
        default_state: 1735,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "yellow_bed",
        first_state: 1748,
        state_count: 16,
        default_state: 1751,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "lime_bed",
        first_state: 1764,
        state_count: 16,
        default_state: 1767,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "pink_bed",
        first_state: 1780,
        state_count: 16,
        default_state: 1783,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "gray_bed",
        first_state: 1796,
        state_count: 16,
        default_state: 1799,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "light_gray_bed",
        first_state: 1812,
        state_count: 16,
        default_state: 1815,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "cyan_bed",
        first_state: 1828,
        state_count: 16,
        default_state: 1831,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "purple_bed",
        first_state: 1844,
        state_count: 16,
        default_state: 1847,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "blue_bed",
        first_state: 1860,
        state_count: 16,
        default_state: 1863,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "brown_bed",
        first_state: 1876,
        state_count: 16,
        default_state: 1879,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "green_bed",
        first_state: 1892,
        state_count: 16,
        default_state: 1895,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "red_bed",
        first_state: 1908,
        state_count: 16,
        default_state: 1911,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "black_bed",
        first_state: 1924,
        state_count: 16,
        default_state: 1927,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "part",
                values: &["head", "foot"],
            },
        ],
    },
    BlockInfo {
        name: "powered_rail",
        first_state: 1940,
        state_count: 24,
        default_state: 1953,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "north_south",
                    "east_west",
                    "ascending_east",
                    "ascending_west",
                    "ascending_north",
                    "ascending_south",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "detector_rail",
        first_state: 1964,
        state_count: 24,
        default_state: 1977,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "north_south",
                    "east_west",
                    "ascending_east",
                    "ascending_west",
                    "ascending_north",
                    "ascending_south",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sticky_piston",
        first_state: 1988,
        state_count: 12,
        default_state: 1994,
        props: &[
            PropInfo {
                name: "extended",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
        ],
    },
    BlockInfo {
        name: "cobweb",
        first_state: 2000,
        state_count: 1,
        default_state: 2000,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 2001,
        state_count: 1,
        default_state: 2001,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 2002,
        state_count: 1,
        default_state: 2002,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 2003,
        state_count: 1,
        default_state: 2003,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 2004,
        state_count: 1,
        default_state: 2004,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 2005,
        state_count: 2,
        default_state: 2006,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 2007,
        state_count: 12,
        default_state: 2013,
        props: &[
            PropInfo {
                name: "extended",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
        ],
    },
    BlockInfo {
        name: "piston_head",
        first_state: 2019,
        state_count: 24,
        default_state: 2021,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "short",
                values: &["true", "false"],
            },
            PropInfo {
                name: "type",
                values: &["normal", "sticky"],
            },
        ],
    },
    BlockInfo {
        name: "white_wool",
        first_state: 2043,
        state_count: 1,
        default_state: 2043,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 2044,
        state_count: 1,
        default_state: 2044,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 2045,
        state_count: 1,
        default_state: 2045,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 2046,
        state_count: 1,
        default_state: 2046,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 2047,
        state_count: 1,
        default_state: 2047,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 2048,
        state_count: 1,
        default_state: 2048,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 2049,
        state_count: 1,
        default_state: 2049,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 2050,
        state_count: 1,
        default_state: 2050,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 2051,
        state_count: 1,
        default_state: 2051,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 2052,
        state_count: 1,
        default_state: 2052,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 2053,
        state_count: 1,
        default_state: 2053,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 2054,
        state_count: 1,
        default_state: 2054,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 2055,
        state_count: 1,
        default_state: 2055,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 2056,
        state_count: 1,
        default_state: 2056,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 2057,
        state_count: 1,
        default_state: 2057,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 2058,
        state_count: 1,
        default_state: 2058,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 2059,
        state_count: 12,
        default_state: 2059,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "type",
                values: &["normal", "sticky"],
            },
        ],
    },
    BlockInfo {
        name: "dandelion",
        first_state: 2071,
        state_count: 1,
        default_state: 2071,
        props: &[],
    },
    BlockInfo {
        name: "torchflower",
        first_state: 2072,
        state_count: 1,
        default_state: 2072,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 2073,
        state_count: 1,
        default_state: 2073,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 2074,
        state_count: 1,
        default_state: 2074,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 2075,
        state_count: 1,
        default_state: 2075,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 2076,
        state_count: 1,
        default_state: 2076,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 2077,
        state_count: 1,
        default_state: 2077,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 2078,
        state_count: 1,
        default_state: 2078,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 2079,
        state_count: 1,
        default_state: 2079,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 2080,
        state_count: 1,
        default_state: 2080,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 2081,
        state_count: 1,
        default_state: 2081,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 2082,
        state_count: 1,
        default_state: 2082,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 2083,
        state_count: 1,
        default_state: 2083,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 2084,
        state_count: 1,
        default_state: 2084,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 2085,
        state_count: 1,
        default_state: 2085,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 2086,
        state_count: 1,
        default_state: 2086,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 2087,
        state_count: 1,
        default_state: 2087,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 2088,
        state_count: 1,
        default_state: 2088,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 2089,
        state_count: 1,
        default_state: 2089,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 2090,
        state_count: 2,
        default_state: 2091,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 2092,
        state_count: 1,
        default_state: 2092,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_bookshelf",
        first_state: 2093,
        state_count: 256,
        default_state: 2156,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "slot_0_occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "slot_1_occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "slot_2_occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "slot_3_occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "slot_4_occupied",
                values: &["true", "false"],
            },
            PropInfo {
                name: "slot_5_occupied",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_cobblestone",
        first_state: 2349,
        state_count: 1,
        default_state: 2349,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 2350,
        state_count: 1,
        default_state: 2350,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 2351,
        state_count: 1,
        default_state: 2351,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 2352,
        state_count: 4,
        default_state: 2352,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 2356,
        state_count: 512,
        default_state: 2387,
        props: &[
            PropInfo {
                name: "age",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "soul_fire",
        first_state: 2868,
        state_count: 1,
        default_state: 2868,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 2869,
        state_count: 1,
        default_state: 2869,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 2870,
        state_count: 80,
        default_state: 2881,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "chest",
        first_state: 2950,
        state_count: 24,
        default_state: 2951,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "type",
                values: &["single", "left", "right"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "redstone_wire",
        first_state: 2974,
        state_count: 1296,
        default_state: 4134,
        props: &[
            PropInfo {
                name: "east",
                values: &["up", "side", "none"],
            },
            PropInfo {
                name: "north",
                values: &["up", "side", "none"],
            },
            PropInfo {
                name: "power",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "south",
                values: &["up", "side", "none"],
            },
            PropInfo {
                name: "west",
                values: &["up", "side", "none"],
            },
        ],
    },
    BlockInfo {
        name: "diamond_ore",
        first_state: 4270,
        state_count: 1,
        default_state: 4270,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_diamond_ore",
        first_state: 4271,
        state_count: 1,
        default_state: 4271,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 4272,
        state_count: 1,
        default_state: 4272,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 4273,
        state_count: 1,
        default_state: 4273,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 4274,
        state_count: 8,
        default_state: 4274,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 4282,
        state_count: 8,
        default_state: 4282,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 4290,
        state_count: 8,
        default_state: 4291,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_sign",
        first_state: 4298,
        state_count: 32,
        default_state: 4299,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_sign",
        first_state: 4330,
        state_count: 32,
        default_state: 4331,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_sign",
        first_state: 4362,
        state_count: 32,
        default_state: 4363,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_sign",
        first_state: 4394,
        state_count: 32,
        default_state: 4395,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_sign",
        first_state: 4426,
        state_count: 32,
        default_state: 4427,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_sign",
        first_state: 4458,
        state_count: 32,
        default_state: 4459,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_sign",
        first_state: 4490,
        state_count: 32,
        default_state: 4491,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_sign",
        first_state: 4522,
        state_count: 32,
        default_state: 4523,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_sign",
        first_state: 4554,
        state_count: 32,
        default_state: 4555,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_door",
        first_state: 4586,
        state_count: 64,
        default_state: 4597,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "ladder",
        first_state: 4650,
        state_count: 8,
        default_state: 4651,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "rail",
        first_state: 4658,
        state_count: 20,
        default_state: 4659,
        props: &[
            PropInfo {
                name: "shape",
                values: &[
                    "north_south",
                    "east_west",
                    "ascending_east",
                    "ascending_west",
                    "ascending_north",
                    "ascending_south",
                    "south_east",
                    "south_west",
                    "north_west",
                    "north_east",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cobblestone_stairs",
        first_state: 4678,
        state_count: 80,
        default_state: 4689,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_wall_sign",
        first_state: 4758,
        state_count: 8,
        default_state: 4759,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_wall_sign",
        first_state: 4766,
        state_count: 8,
        default_state: 4767,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_wall_sign",
        first_state: 4774,
        state_count: 8,
        default_state: 4775,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_wall_sign",
        first_state: 4782,
        state_count: 8,
        default_state: 4783,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_wall_sign",
        first_state: 4790,
        state_count: 8,
        default_state: 4791,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_wall_sign",
        first_state: 4798,
        state_count: 8,
        default_state: 4799,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_wall_sign",
        first_state: 4806,
        state_count: 8,
        default_state: 4807,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_wall_sign",
        first_state: 4814,
        state_count: 8,
        default_state: 4815,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_wall_sign",
        first_state: 4822,
        state_count: 8,
        default_state: 4823,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_hanging_sign",
        first_state: 4830,
        state_count: 64,
        default_state: 4863,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_hanging_sign",
        first_state: 4894,
        state_count: 64,
        default_state: 4927,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_hanging_sign",
        first_state: 4958,
        state_count: 64,
        default_state: 4991,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_hanging_sign",
        first_state: 5022,
        state_count: 64,
        default_state: 5055,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_hanging_sign",
        first_state: 5086,
        state_count: 64,
        default_state: 5119,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_hanging_sign",
        first_state: 5150,
        state_count: 64,
        default_state: 5183,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_hanging_sign",
        first_state: 5214,
        state_count: 64,
        default_state: 5247,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_hanging_sign",
        first_state: 5278,
        state_count: 64,
        default_state: 5311,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_hanging_sign",
        first_state: 5342,
        state_count: 64,
        default_state: 5375,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_hanging_sign",
        first_state: 5406,
        state_count: 64,
        default_state: 5439,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_hanging_sign",
        first_state: 5470,
        state_count: 64,
        default_state: 5503,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_wall_hanging_sign",
        first_state: 5534,
        state_count: 8,
        default_state: 5535,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_wall_hanging_sign",
        first_state: 5542,
        state_count: 8,
        default_state: 5543,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_wall_hanging_sign",
        first_state: 5550,
        state_count: 8,
        default_state: 5551,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_wall_hanging_sign",
        first_state: 5558,
        state_count: 8,
        default_state: 5559,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_wall_hanging_sign",
        first_state: 5566,
        state_count: 8,
        default_state: 5567,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_wall_hanging_sign",
        first_state: 5574,
        state_count: 8,
        default_state: 5575,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_wall_hanging_sign",
        first_state: 5582,
        state_count: 8,
        default_state: 5583,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_wall_hanging_sign",
        first_state: 5590,
        state_count: 8,
        default_state: 5591,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_wall_hanging_sign",
        first_state: 5598,
        state_count: 8,
        default_state: 5599,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_wall_hanging_sign",
        first_state: 5606,
        state_count: 8,
        default_state: 5607,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_wall_hanging_sign",
        first_state: 5614,
        state_count: 8,
        default_state: 5615,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lever",
        first_state: 5622,
        state_count: 24,
        default_state: 5631,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_pressure_plate",
        first_state: 5646,
        state_count: 2,
        default_state: 5647,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 5648,
        state_count: 64,
        default_state: 5659,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_pressure_plate",
        first_state: 5712,
        state_count: 2,
        default_state: 5713,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 5714,
        state_count: 2,
        default_state: 5715,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 5716,
        state_count: 2,
        default_state: 5717,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 5718,
        state_count: 2,
        default_state: 5719,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 5720,
        state_count: 2,
        default_state: 5721,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cherry_pressure_plate",
        first_state: 5722,
        state_count: 2,
        default_state: 5723,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 5724,
        state_count: 2,
        default_state: 5725,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "mangrove_pressure_plate",
        first_state: 5726,
        state_count: 2,
        default_state: 5727,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_pressure_plate",
        first_state: 5728,
        state_count: 2,
        default_state: 5729,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 5730,
        state_count: 2,
        default_state: 5731,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "deepslate_redstone_ore",
        first_state: 5732,
        state_count: 2,
        default_state: 5733,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 5734,
        state_count: 2,
        default_state: 5734,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 5736,
        state_count: 8,
        default_state: 5736,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_button",
        first_state: 5744,
        state_count: 24,
        default_state: 5753,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "snow",
        first_state: 5768,
        state_count: 8,
        default_state: 5768,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 5776,
        state_count: 1,
        default_state: 5776,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 5777,
        state_count: 1,
        default_state: 5777,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 5778,
        state_count: 16,
        default_state: 5778,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "clay",
        first_state: 5794,
        state_count: 1,
        default_state: 5794,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 5795,
        state_count: 16,
        default_state: 5795,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "jukebox",
        first_state: 5811,
        state_count: 2,
        default_state: 5812,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 5813,
        state_count: 32,
        default_state: 5844,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pumpkin",
        first_state: 5845,
        state_count: 1,
        default_state: 5845,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 5846,
        state_count: 1,
        default_state: 5846,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 5847,
        state_count: 1,
        default_state: 5847,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 5848,
        state_count: 1,
        default_state: 5848,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 5849,
        state_count: 3,
        default_state: 5850,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 5852,
        state_count: 3,
        default_state: 5853,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 5855,
        state_count: 1,
        default_state: 5855,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 5856,
        state_count: 4,
        default_state: 5856,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 5860,
        state_count: 1,
        default_state: 5860,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 5861,
        state_count: 2,
        default_state: 5861,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 5863,
        state_count: 4,
        default_state: 5863,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 5867,
        state_count: 4,
        default_state: 5867,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 5871,
        state_count: 7,
        default_state: 5871,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 5878,
        state_count: 64,
        default_state: 5881,
        props: &[
            PropInfo {
                name: "delay",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "locked",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "white_stained_glass",
        first_state: 5942,
        state_count: 1,
        default_state: 5942,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 5943,
        state_count: 1,
        default_state: 5943,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 5944,
        state_count: 1,
        default_state: 5944,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 5945,
        state_count: 1,
        default_state: 5945,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 5946,
        state_count: 1,
        default_state: 5946,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 5947,
        state_count: 1,
        default_state: 5947,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 5948,
        state_count: 1,
        default_state: 5948,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 5949,
        state_count: 1,
        default_state: 5949,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 5950,
        state_count: 1,
        default_state: 5950,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 5951,
        state_count: 1,
        default_state: 5951,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 5952,
        state_count: 1,
        default_state: 5952,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 5953,
        state_count: 1,
        default_state: 5953,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 5954,
        state_count: 1,
        default_state: 5954,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 5955,
        state_count: 1,
        default_state: 5955,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 5956,
        state_count: 1,
        default_state: 5956,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 5957,
        state_count: 1,
        default_state: 5957,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 5958,
        state_count: 64,
        default_state: 5973,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_trapdoor",
        first_state: 6022,
        state_count: 64,
        default_state: 6037,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_trapdoor",
        first_state: 6086,
        state_count: 64,
        default_state: 6101,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_trapdoor",
        first_state: 6150,
        state_count: 64,
        default_state: 6165,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_trapdoor",
        first_state: 6214,
        state_count: 64,
        default_state: 6229,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_trapdoor",
        first_state: 6278,
        state_count: 64,
        default_state: 6293,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_trapdoor",
        first_state: 6342,
        state_count: 64,
        default_state: 6357,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_trapdoor",
        first_state: 6406,
        state_count: 64,
        default_state: 6421,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_trapdoor",
        first_state: 6470,
        state_count: 64,
        default_state: 6485,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_bricks",
        first_state: 6534,
        state_count: 1,
        default_state: 6534,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 6535,
        state_count: 1,
        default_state: 6535,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 6536,
        state_count: 1,
        default_state: 6536,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 6537,
        state_count: 1,
        default_state: 6537,
        props: &[],
    },
    BlockInfo {
        name: "packed_mud",
        first_state: 6538,
        state_count: 1,
        default_state: 6538,
        props: &[],
    },
    BlockInfo {
        name: "mud_bricks",
        first_state: 6539,
        state_count: 1,
        default_state: 6539,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 6540,
        state_count: 1,
        default_state: 6540,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 6541,
        state_count: 1,
        default_state: 6541,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 6542,
        state_count: 1,
        default_state: 6542,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 6543,
        state_count: 1,
        default_state: 6543,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 6544,
        state_count: 1,
        default_state: 6544,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 6545,
        state_count: 1,
        default_state: 6545,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 6546,
        state_count: 64,
        default_state: 6546,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_mushroom_block",
        first_state: 6610,
        state_count: 64,
        default_state: 6610,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mushroom_stem",
        first_state: 6674,
        state_count: 64,
        default_state: 6674,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "iron_bars",
        first_state: 6738,
        state_count: 32,
        default_state: 6769,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "chain",
        first_state: 6770,
        state_count: 6,
        default_state: 6773,
        props: &[
            PropInfo {
                name: "axis",
                values: &["x", "y", "z"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "glass_pane",
        first_state: 6776,
        state_count: 32,
        default_state: 6807,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "melon",
        first_state: 6808,
        state_count: 1,
        default_state: 6808,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 6809,
        state_count: 4,
        default_state: 6809,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 6813,
        state_count: 4,
        default_state: 6813,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 6817,
        state_count: 8,
        default_state: 6817,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 6825,
        state_count: 8,
        default_state: 6825,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 6833,
        state_count: 32,
        default_state: 6864,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "glow_lichen",
        first_state: 6865,
        state_count: 128,
        default_state: 6992,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_fence_gate",
        first_state: 6993,
        state_count: 32,
        default_state: 7000,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brick_stairs",
        first_state: 7025,
        state_count: 80,
        default_state: 7036,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_brick_stairs",
        first_state: 7105,
        state_count: 80,
        default_state: 7116,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mud_brick_stairs",
        first_state: 7185,
        state_count: 80,
        default_state: 7196,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mycelium",
        first_state: 7265,
        state_count: 2,
        default_state: 7266,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 7267,
        state_count: 1,
        default_state: 7267,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 7268,
        state_count: 1,
        default_state: 7268,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 7269,
        state_count: 32,
        default_state: 7300,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "nether_brick_stairs",
        first_state: 7301,
        state_count: 80,
        default_state: 7312,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "nether_wart",
        first_state: 7381,
        state_count: 4,
        default_state: 7381,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 7385,
        state_count: 1,
        default_state: 7385,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 7386,
        state_count: 8,
        default_state: 7393,
        props: &[
            PropInfo {
                name: "has_bottle_0",
                values: &["true", "false"],
            },
            PropInfo {
                name: "has_bottle_1",
                values: &["true", "false"],
            },
            PropInfo {
                name: "has_bottle_2",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cauldron",
        first_state: 7394,
        state_count: 1,
        default_state: 7394,
        props: &[],
    },
    BlockInfo {
        name: "water_cauldron",
        first_state: 7395,
        state_count: 3,
        default_state: 7395,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "lava_cauldron",
        first_state: 7398,
        state_count: 1,
        default_state: 7398,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow_cauldron",
        first_state: 7399,
        state_count: 3,
        default_state: 7399,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 7402,
        state_count: 1,
        default_state: 7402,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 7403,
        state_count: 8,
        default_state: 7407,
        props: &[
            PropInfo {
                name: "eye",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
        ],
    },
    BlockInfo {
        name: "end_stone",
        first_state: 7411,
        state_count: 1,
        default_state: 7411,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 7412,
        state_count: 1,
        default_state: 7412,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 7413,
        state_count: 2,
        default_state: 7414,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 7415,
        state_count: 12,
        default_state: 7415,
        props: &[
            PropInfo {
                name: "age",
                values: &["0", "1", "2"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
        ],
    },
    BlockInfo {
        name: "sandstone_stairs",
        first_state: 7427,
        state_count: 80,
        default_state: 7438,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "emerald_ore",
        first_state: 7507,
        state_count: 1,
        default_state: 7507,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_emerald_ore",
        first_state: 7508,
        state_count: 1,
        default_state: 7508,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 7509,
        state_count: 8,
        default_state: 7510,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tripwire_hook",
        first_state: 7517,
        state_count: 16,
        default_state: 7526,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tripwire",
        first_state: 7533,
        state_count: 128,
        default_state: 7660,
        props: &[
            PropInfo {
                name: "attached",
                values: &["true", "false"],
            },
            PropInfo {
                name: "disarmed",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "emerald_block",
        first_state: 7661,
        state_count: 1,
        default_state: 7661,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 7662,
        state_count: 80,
        default_state: 7673,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_stairs",
        first_state: 7742,
        state_count: 80,
        default_state: 7753,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_stairs",
        first_state: 7822,
        state_count: 80,
        default_state: 7833,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "command_block",
        first_state: 7902,
        state_count: 12,
        default_state: 7908,
        props: &[
            PropInfo {
                name: "conditional",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
        ],
    },
    BlockInfo {
        name: "beacon",
        first_state: 7914,
        state_count: 1,
        default_state: 7914,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 7915,
        state_count: 324,
        default_state: 7918,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_cobblestone_wall",
        first_state: 8239,
        state_count: 324,
        default_state: 8242,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "flower_pot",
        first_state: 8563,
        state_count: 1,
        default_state: 8563,
        props: &[],
    },
    BlockInfo {
        name: "potted_torchflower",
        first_state: 8564,
        state_count: 1,
        default_state: 8564,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 8565,
        state_count: 1,
        default_state: 8565,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 8566,
        state_count: 1,
        default_state: 8566,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 8567,
        state_count: 1,
        default_state: 8567,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 8568,
        state_count: 1,
        default_state: 8568,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 8569,
        state_count: 1,
        default_state: 8569,
        props: &[],
    },
    BlockInfo {
        name: "potted_cherry_sapling",
        first_state: 8570,
        state_count: 1,
        default_state: 8570,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 8571,
        state_count: 1,
        default_state: 8571,
        props: &[],
    },
    BlockInfo {
        name: "potted_mangrove_propagule",
        first_state: 8572,
        state_count: 1,
        default_state: 8572,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 8573,
        state_count: 1,
        default_state: 8573,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 8574,
        state_count: 1,
        default_state: 8574,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 8575,
        state_count: 1,
        default_state: 8575,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 8576,
        state_count: 1,
        default_state: 8576,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 8577,
        state_count: 1,
        default_state: 8577,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 8578,
        state_count: 1,
        default_state: 8578,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 8579,
        state_count: 1,
        default_state: 8579,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 8580,
        state_count: 1,
        default_state: 8580,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 8581,
        state_count: 1,
        default_state: 8581,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 8582,
        state_count: 1,
        default_state: 8582,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 8583,
        state_count: 1,
        default_state: 8583,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 8584,
        state_count: 1,
        default_state: 8584,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 8585,
        state_count: 1,
        default_state: 8585,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 8586,
        state_count: 1,
        default_state: 8586,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 8587,
        state_count: 1,
        default_state: 8587,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 8588,
        state_count: 1,
        default_state: 8588,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 8589,
        state_count: 1,
        default_state: 8589,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 8590,
        state_count: 1,
        default_state: 8590,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 8591,
        state_count: 8,
        default_state: 8591,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 8599,
        state_count: 8,
        default_state: 8599,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 8607,
        state_count: 24,
        default_state: 8616,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_button",
        first_state: 8631,
        state_count: 24,
        default_state: 8640,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_button",
        first_state: 8655,
        state_count: 24,
        default_state: 8664,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_button",
        first_state: 8679,
        state_count: 24,
        default_state: 8688,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_button",
        first_state: 8703,
        state_count: 24,
        default_state: 8712,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_button",
        first_state: 8727,
        state_count: 24,
        default_state: 8736,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_button",
        first_state: 8751,
        state_count: 24,
        default_state: 8760,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_button",
        first_state: 8775,
        state_count: 24,
        default_state: 8784,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_button",
        first_state: 8799,
        state_count: 24,
        default_state: 8808,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "skeleton_skull",
        first_state: 8823,
        state_count: 16,
        default_state: 8823,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "skeleton_wall_skull",
        first_state: 8839,
        state_count: 4,
        default_state: 8839,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 8843,
        state_count: 16,
        default_state: 8843,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_wall_skull",
        first_state: 8859,
        state_count: 4,
        default_state: 8859,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 8863,
        state_count: 16,
        default_state: 8863,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "zombie_wall_head",
        first_state: 8879,
        state_count: 4,
        default_state: 8879,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 8883,
        state_count: 16,
        default_state: 8883,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "player_wall_head",
        first_state: 8899,
        state_count: 4,
        default_state: 8899,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 8903,
        state_count: 16,
        default_state: 8903,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "creeper_wall_head",
        first_state: 8919,
        state_count: 4,
        default_state: 8919,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 8923,
        state_count: 16,
        default_state: 8923,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "dragon_wall_head",
        first_state: 8939,
        state_count: 4,
        default_state: 8939,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "piglin_head",
        first_state: 8943,
        state_count: 16,
        default_state: 8943,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "piglin_wall_head",
        first_state: 8959,
        state_count: 4,
        default_state: 8959,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 8963,
        state_count: 4,
        default_state: 8963,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 8967,
        state_count: 4,
        default_state: 8967,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 8971,
        state_count: 4,
        default_state: 8971,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 8975,
        state_count: 24,
        default_state: 8976,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "type",
                values: &["single", "left", "right"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "light_weighted_pressure_plate",
        first_state: 8999,
        state_count: 16,
        default_state: 8999,
        props: &[PropInfo {
            name: "power",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "heavy_weighted_pressure_plate",
        first_state: 9015,
        state_count: 16,
        default_state: 9015,
        props: &[PropInfo {
            name: "power",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "comparator",
        first_state: 9031,
        state_count: 16,
        default_state: 9032,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "mode",
                values: &["compare", "subtract"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "daylight_detector",
        first_state: 9047,
        state_count: 32,
        default_state: 9063,
        props: &[
            PropInfo {
                name: "inverted",
                values: &["true", "false"],
            },
            PropInfo {
                name: "power",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "redstone_block",
        first_state: 9079,
        state_count: 1,
        default_state: 9079,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 9080,
        state_count: 1,
        default_state: 9080,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 9081,
        state_count: 10,
        default_state: 9081,
        props: &[
            PropInfo {
                name: "enabled",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["down", "north", "south", "west", "east"],
            },
        ],
    },
    BlockInfo {
        name: "quartz_block",
        first_state: 9091,
        state_count: 1,
        default_state: 9091,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 9092,
        state_count: 1,
        default_state: 9092,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 9093,
        state_count: 3,
        default_state: 9094,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 9096,
        state_count: 80,
        default_state: 9107,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "activator_rail",
        first_state: 9176,
        state_count: 24,
        default_state: 9189,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "north_south",
                    "east_west",
                    "ascending_east",
                    "ascending_west",
                    "ascending_north",
                    "ascending_south",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dropper",
        first_state: 9200,
        state_count: 12,
        default_state: 9201,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "triggered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "white_terracotta",
        first_state: 9212,
        state_count: 1,
        default_state: 9212,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 9213,
        state_count: 1,
        default_state: 9213,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 9214,
        state_count: 1,
        default_state: 9214,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 9215,
        state_count: 1,
        default_state: 9215,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 9216,
        state_count: 1,
        default_state: 9216,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 9217,
        state_count: 1,
        default_state: 9217,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 9218,
        state_count: 1,
        default_state: 9218,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 9219,
        state_count: 1,
        default_state: 9219,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 9220,
        state_count: 1,
        default_state: 9220,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 9221,
        state_count: 1,
        default_state: 9221,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 9222,
        state_count: 1,
        default_state: 9222,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 9223,
        state_count: 1,
        default_state: 9223,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 9224,
        state_count: 1,
        default_state: 9224,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 9225,
        state_count: 1,
        default_state: 9225,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 9226,
        state_count: 1,
        default_state: 9226,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 9227,
        state_count: 1,
        default_state: 9227,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 9228,
        state_count: 32,
        default_state: 9259,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "orange_stained_glass_pane",
        first_state: 9260,
        state_count: 32,
        default_state: 9291,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "magenta_stained_glass_pane",
        first_state: 9292,
        state_count: 32,
        default_state: 9323,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "light_blue_stained_glass_pane",
        first_state: 9324,
        state_count: 32,
        default_state: 9355,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "yellow_stained_glass_pane",
        first_state: 9356,
        state_count: 32,
        default_state: 9387,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lime_stained_glass_pane",
        first_state: 9388,
        state_count: 32,
        default_state: 9419,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pink_stained_glass_pane",
        first_state: 9420,
        state_count: 32,
        default_state: 9451,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "gray_stained_glass_pane",
        first_state: 9452,
        state_count: 32,
        default_state: 9483,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "light_gray_stained_glass_pane",
        first_state: 9484,
        state_count: 32,
        default_state: 9515,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cyan_stained_glass_pane",
        first_state: 9516,
        state_count: 32,
        default_state: 9547,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "purple_stained_glass_pane",
        first_state: 9548,
        state_count: 32,
        default_state: 9579,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "blue_stained_glass_pane",
        first_state: 9580,
        state_count: 32,
        default_state: 9611,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brown_stained_glass_pane",
        first_state: 9612,
        state_count: 32,
        default_state: 9643,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "green_stained_glass_pane",
        first_state: 9644,
        state_count: 32,
        default_state: 9675,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_stained_glass_pane",
        first_state: 9676,
        state_count: 32,
        default_state: 9707,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "black_stained_glass_pane",
        first_state: 9708,
        state_count: 32,
        default_state: 9739,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_stairs",
        first_state: 9740,
        state_count: 80,
        default_state: 9751,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_stairs",
        first_state: 9820,
        state_count: 80,
        default_state: 9831,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_stairs",
        first_state: 9900,
        state_count: 80,
        default_state: 9911,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_stairs",
        first_state: 9980,
        state_count: 80,
        default_state: 9991,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_stairs",
        first_state: 10060,
        state_count: 80,
        default_state: 10071,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_mosaic_stairs",
        first_state: 10140,
        state_count: 80,
        default_state: 10151,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "slime_block",
        first_state: 10220,
        state_count: 1,
        default_state: 10220,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 10221,
        state_count: 1,
        default_state: 10221,
        props: &[],
    },
    BlockInfo {
        name: "light",
        first_state: 10222,
        state_count: 32,
        default_state: 10253,
        props: &[
            PropInfo {
                name: "level",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "iron_trapdoor",
        first_state: 10254,
        state_count: 64,
        default_state: 10269,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "prismarine",
        first_state: 10318,
        state_count: 1,
        default_state: 10318,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 10319,
        state_count: 1,
        default_state: 10319,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 10320,
        state_count: 1,
        default_state: 10320,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 10321,
        state_count: 80,
        default_state: 10332,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "prismarine_brick_stairs",
        first_state: 10401,
        state_count: 80,
        default_state: 10412,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_prismarine_stairs",
        first_state: 10481,
        state_count: 80,
        default_state: 10492,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "prismarine_slab",
        first_state: 10561,
        state_count: 6,
        default_state: 10564,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "prismarine_brick_slab",
        first_state: 10567,
        state_count: 6,
        default_state: 10570,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_prismarine_slab",
        first_state: 10573,
        state_count: 6,
        default_state: 10576,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sea_lantern",
        first_state: 10579,
        state_count: 1,
        default_state: 10579,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 10580,
        state_count: 3,
        default_state: 10581,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 10583,
        state_count: 1,
        default_state: 10583,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 10584,
        state_count: 1,
        default_state: 10584,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 10585,
        state_count: 1,
        default_state: 10585,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 10586,
        state_count: 1,
        default_state: 10586,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 10587,
        state_count: 1,
        default_state: 10587,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 10588,
        state_count: 1,
        default_state: 10588,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 10589,
        state_count: 1,
        default_state: 10589,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 10590,
        state_count: 1,
        default_state: 10590,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 10591,
        state_count: 1,
        default_state: 10591,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 10592,
        state_count: 1,
        default_state: 10592,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 10593,
        state_count: 1,
        default_state: 10593,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 10594,
        state_count: 1,
        default_state: 10594,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 10595,
        state_count: 1,
        default_state: 10595,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 10596,
        state_count: 1,
        default_state: 10596,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 10597,
        state_count: 1,
        default_state: 10597,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 10598,
        state_count: 1,
        default_state: 10598,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 10599,
        state_count: 1,
        default_state: 10599,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 10600,
        state_count: 1,
        default_state: 10600,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 10601,
        state_count: 1,
        default_state: 10601,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 10602,
        state_count: 2,
        default_state: 10603,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 10604,
        state_count: 2,
        default_state: 10605,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 10606,
        state_count: 2,
        default_state: 10607,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 10608,
        state_count: 2,
        default_state: 10609,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 10610,
        state_count: 2,
        default_state: 10611,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 10612,
        state_count: 2,
        default_state: 10613,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 10614,
        state_count: 16,
        default_state: 10614,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "orange_banner",
        first_state: 10630,
        state_count: 16,
        default_state: 10630,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "magenta_banner",
        first_state: 10646,
        state_count: 16,
        default_state: 10646,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "light_blue_banner",
        first_state: 10662,
        state_count: 16,
        default_state: 10662,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "yellow_banner",
        first_state: 10678,
        state_count: 16,
        default_state: 10678,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "lime_banner",
        first_state: 10694,
        state_count: 16,
        default_state: 10694,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "pink_banner",
        first_state: 10710,
        state_count: 16,
        default_state: 10710,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "gray_banner",
        first_state: 10726,
        state_count: 16,
        default_state: 10726,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "light_gray_banner",
        first_state: 10742,
        state_count: 16,
        default_state: 10742,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "cyan_banner",
        first_state: 10758,
        state_count: 16,
        default_state: 10758,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "purple_banner",
        first_state: 10774,
        state_count: 16,
        default_state: 10774,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "blue_banner",
        first_state: 10790,
        state_count: 16,
        default_state: 10790,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "brown_banner",
        first_state: 10806,
        state_count: 16,
        default_state: 10806,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "green_banner",
        first_state: 10822,
        state_count: 16,
        default_state: 10822,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "red_banner",
        first_state: 10838,
        state_count: 16,
        default_state: 10838,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "black_banner",
        first_state: 10854,
        state_count: 16,
        default_state: 10854,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "white_wall_banner",
        first_state: 10870,
        state_count: 4,
        default_state: 10870,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 10874,
        state_count: 4,
        default_state: 10874,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 10878,
        state_count: 4,
        default_state: 10878,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 10882,
        state_count: 4,
        default_state: 10882,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 10886,
        state_count: 4,
        default_state: 10886,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 10890,
        state_count: 4,
        default_state: 10890,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 10894,
        state_count: 4,
        default_state: 10894,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 10898,
        state_count: 4,
        default_state: 10898,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 10902,
        state_count: 4,
        default_state: 10902,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 10906,
        state_count: 4,
        default_state: 10906,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 10910,
        state_count: 4,
        default_state: 10910,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 10914,
        state_count: 4,
        default_state: 10914,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 10918,
        state_count: 4,
        default_state: 10918,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 10922,
        state_count: 4,
        default_state: 10922,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 10926,
        state_count: 4,
        default_state: 10926,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 10930,
        state_count: 4,
        default_state: 10930,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 10934,
        state_count: 1,
        default_state: 10934,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 10935,
        state_count: 1,
        default_state: 10935,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 10936,
        state_count: 1,
        default_state: 10936,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 10937,
        state_count: 80,
        default_state: 10948,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_slab",
        first_state: 11017,
        state_count: 6,
        default_state: 11020,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_slab",
        first_state: 11023,
        state_count: 6,
        default_state: 11026,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_slab",
        first_state: 11029,
        state_count: 6,
        default_state: 11032,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_slab",
        first_state: 11035,
        state_count: 6,
        default_state: 11038,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_slab",
        first_state: 11041,
        state_count: 6,
        default_state: 11044,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_slab",
        first_state: 11047,
        state_count: 6,
        default_state: 11050,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_slab",
        first_state: 11053,
        state_count: 6,
        default_state: 11056,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_slab",
        first_state: 11059,
        state_count: 6,
        default_state: 11062,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_slab",
        first_state: 11065,
        state_count: 6,
        default_state: 11068,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_mosaic_slab",
        first_state: 11071,
        state_count: 6,
        default_state: 11074,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_slab",
        first_state: 11077,
        state_count: 6,
        default_state: 11080,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_stone_slab",
        first_state: 11083,
        state_count: 6,
        default_state: 11086,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sandstone_slab",
        first_state: 11089,
        state_count: 6,
        default_state: 11092,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cut_sandstone_slab",
        first_state: 11095,
        state_count: 6,
        default_state: 11098,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "petrified_oak_slab",
        first_state: 11101,
        state_count: 6,
        default_state: 11104,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cobblestone_slab",
        first_state: 11107,
        state_count: 6,
        default_state: 11110,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brick_slab",
        first_state: 11113,
        state_count: 6,
        default_state: 11116,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_brick_slab",
        first_state: 11119,
        state_count: 6,
        default_state: 11122,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mud_brick_slab",
        first_state: 11125,
        state_count: 6,
        default_state: 11128,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "nether_brick_slab",
        first_state: 11131,
        state_count: 6,
        default_state: 11134,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "quartz_slab",
        first_state: 11137,
        state_count: 6,
        default_state: 11140,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_sandstone_slab",
        first_state: 11143,
        state_count: 6,
        default_state: 11146,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cut_red_sandstone_slab",
        first_state: 11149,
        state_count: 6,
        default_state: 11152,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "purpur_slab",
        first_state: 11155,
        state_count: 6,
        default_state: 11158,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_stone",
        first_state: 11161,
        state_count: 1,
        default_state: 11161,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 11162,
        state_count: 1,
        default_state: 11162,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 11163,
        state_count: 1,
        default_state: 11163,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 11164,
        state_count: 1,
        default_state: 11164,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 11165,
        state_count: 32,
        default_state: 11172,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_fence_gate",
        first_state: 11197,
        state_count: 32,
        default_state: 11204,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_fence_gate",
        first_state: 11229,
        state_count: 32,
        default_state: 11236,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_fence_gate",
        first_state: 11261,
        state_count: 32,
        default_state: 11268,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_fence_gate",
        first_state: 11293,
        state_count: 32,
        default_state: 11300,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_fence_gate",
        first_state: 11325,
        state_count: 32,
        default_state: 11332,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_fence_gate",
        first_state: 11357,
        state_count: 32,
        default_state: 11364,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_fence_gate",
        first_state: 11389,
        state_count: 32,
        default_state: 11396,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_fence",
        first_state: 11421,
        state_count: 32,
        default_state: 11452,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_fence",
        first_state: 11453,
        state_count: 32,
        default_state: 11484,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_fence",
        first_state: 11485,
        state_count: 32,
        default_state: 11516,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_fence",
        first_state: 11517,
        state_count: 32,
        default_state: 11548,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_fence",
        first_state: 11549,
        state_count: 32,
        default_state: 11580,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_fence",
        first_state: 11581,
        state_count: 32,
        default_state: 11612,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_fence",
        first_state: 11613,
        state_count: 32,
        default_state: 11644,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_fence",
        first_state: 11645,
        state_count: 32,
        default_state: 11676,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_door",
        first_state: 11677,
        state_count: 64,
        default_state: 11688,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_door",
        first_state: 11741,
        state_count: 64,
        default_state: 11752,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_door",
        first_state: 11805,
        state_count: 64,
        default_state: 11816,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_door",
        first_state: 11869,
        state_count: 64,
        default_state: 11880,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cherry_door",
        first_state: 11933,
        state_count: 64,
        default_state: 11944,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_door",
        first_state: 11997,
        state_count: 64,
        default_state: 12008,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mangrove_door",
        first_state: 12061,
        state_count: 64,
        default_state: 12072,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bamboo_door",
        first_state: 12125,
        state_count: 64,
        default_state: 12136,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "end_rod",
        first_state: 12189,
        state_count: 6,
        default_state: 12193,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 12195,
        state_count: 64,
        default_state: 12258,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "chorus_flower",
        first_state: 12259,
        state_count: 6,
        default_state: 12259,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 12265,
        state_count: 1,
        default_state: 12265,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 12266,
        state_count: 3,
        default_state: 12267,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 12269,
        state_count: 80,
        default_state: 12280,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "end_stone_bricks",
        first_state: 12349,
        state_count: 1,
        default_state: 12349,
        props: &[],
    },
    BlockInfo {
        name: "torchflower_crop",
        first_state: 12350,
        state_count: 3,
        default_state: 12350,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2"],
        }],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 12353,
        state_count: 4,
        default_state: 12353,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "dirt_path",
        first_state: 12357,
        state_count: 1,
        default_state: 12357,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 12358,
        state_count: 1,
        default_state: 12358,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 12359,
        state_count: 12,
        default_state: 12365,
        props: &[
            PropInfo {
                name: "conditional",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
        ],
    },
    BlockInfo {
        name: "chain_command_block",
        first_state: 12371,
        state_count: 12,
        default_state: 12377,
        props: &[
            PropInfo {
                name: "conditional",
                values: &["true", "false"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
        ],
    },
    BlockInfo {
        name: "frosted_ice",
        first_state: 12383,
        state_count: 4,
        default_state: 12383,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 12387,
        state_count: 1,
        default_state: 12387,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 12388,
        state_count: 1,
        default_state: 12388,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 12389,
        state_count: 1,
        default_state: 12389,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 12390,
        state_count: 3,
        default_state: 12391,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 12393,
        state_count: 1,
        default_state: 12393,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 12394,
        state_count: 12,
        default_state: 12399,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "shulker_box",
        first_state: 12406,
        state_count: 6,
        default_state: 12410,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 12412,
        state_count: 6,
        default_state: 12416,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 12418,
        state_count: 6,
        default_state: 12422,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 12424,
        state_count: 6,
        default_state: 12428,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 12430,
        state_count: 6,
        default_state: 12434,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 12436,
        state_count: 6,
        default_state: 12440,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 12442,
        state_count: 6,
        default_state: 12446,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 12448,
        state_count: 6,
        default_state: 12452,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 12454,
        state_count: 6,
        default_state: 12458,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 12460,
        state_count: 6,
        default_state: 12464,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 12466,
        state_count: 6,
        default_state: 12470,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 12472,
        state_count: 6,
        default_state: 12476,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 12478,
        state_count: 6,
        default_state: 12482,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 12484,
        state_count: 6,
        default_state: 12488,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 12490,
        state_count: 6,
        default_state: 12494,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 12496,
        state_count: 6,
        default_state: 12500,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 12502,
        state_count: 6,
        default_state: 12506,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 12508,
        state_count: 4,
        default_state: 12508,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 12512,
        state_count: 4,
        default_state: 12512,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 12516,
        state_count: 4,
        default_state: 12516,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 12520,
        state_count: 4,
        default_state: 12520,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 12524,
        state_count: 4,
        default_state: 12524,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 12528,
        state_count: 4,
        default_state: 12528,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 12532,
        state_count: 4,
        default_state: 12532,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 12536,
        state_count: 4,
        default_state: 12536,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 12540,
        state_count: 4,
        default_state: 12540,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 12544,
        state_count: 4,
        default_state: 12544,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 12548,
        state_count: 4,
        default_state: 12548,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 12552,
        state_count: 4,
        default_state: 12552,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 12556,
        state_count: 4,
        default_state: 12556,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 12560,
        state_count: 4,
        default_state: 12560,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 12564,
        state_count: 4,
        default_state: 12564,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 12568,
        state_count: 4,
        default_state: 12568,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 12572,
        state_count: 1,
        default_state: 12572,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 12573,
        state_count: 1,
        default_state: 12573,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 12574,
        state_count: 1,
        default_state: 12574,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 12575,
        state_count: 1,
        default_state: 12575,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 12576,
        state_count: 1,
        default_state: 12576,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 12577,
        state_count: 1,
        default_state: 12577,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 12578,
        state_count: 1,
        default_state: 12578,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 12579,
        state_count: 1,
        default_state: 12579,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 12580,
        state_count: 1,
        default_state: 12580,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 12581,
        state_count: 1,
        default_state: 12581,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 12582,
        state_count: 1,
        default_state: 12582,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 12583,
        state_count: 1,
        default_state: 12583,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 12584,
        state_count: 1,
        default_state: 12584,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 12585,
        state_count: 1,
        default_state: 12585,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 12586,
        state_count: 1,
        default_state: 12586,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 12587,
        state_count: 1,
        default_state: 12587,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 12588,
        state_count: 1,
        default_state: 12588,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 12589,
        state_count: 1,
        default_state: 12589,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 12590,
        state_count: 1,
        default_state: 12590,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 12591,
        state_count: 1,
        default_state: 12591,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 12592,
        state_count: 1,
        default_state: 12592,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 12593,
        state_count: 1,
        default_state: 12593,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 12594,
        state_count: 1,
        default_state: 12594,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 12595,
        state_count: 1,
        default_state: 12595,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 12596,
        state_count: 1,
        default_state: 12596,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 12597,
        state_count: 1,
        default_state: 12597,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 12598,
        state_count: 1,
        default_state: 12598,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 12599,
        state_count: 1,
        default_state: 12599,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 12600,
        state_count: 1,
        default_state: 12600,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 12601,
        state_count: 1,
        default_state: 12601,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 12602,
        state_count: 1,
        default_state: 12602,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 12603,
        state_count: 1,
        default_state: 12603,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 12604,
        state_count: 26,
        default_state: 12604,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
            ],
        }],
    },
    BlockInfo {
        name: "kelp_plant",
        first_state: 12630,
        state_count: 1,
        default_state: 12630,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 12631,
        state_count: 1,
        default_state: 12631,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 12632,
        state_count: 12,
        default_state: 12632,
        props: &[
            PropInfo {
                name: "eggs",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "hatch",
                values: &["0", "1", "2"],
            },
        ],
    },
    BlockInfo {
        name: "dead_tube_coral_block",
        first_state: 12644,
        state_count: 1,
        default_state: 12644,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 12645,
        state_count: 1,
        default_state: 12645,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 12646,
        state_count: 1,
        default_state: 12646,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 12647,
        state_count: 1,
        default_state: 12647,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 12648,
        state_count: 1,
        default_state: 12648,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 12649,
        state_count: 1,
        default_state: 12649,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 12650,
        state_count: 1,
        default_state: 12650,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 12651,
        state_count: 1,
        default_state: 12651,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 12652,
        state_count: 1,
        default_state: 12652,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 12653,
        state_count: 1,
        default_state: 12653,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 12654,
        state_count: 2,
        default_state: 12654,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 12656,
        state_count: 2,
        default_state: 12656,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 12658,
        state_count: 2,
        default_state: 12658,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 12660,
        state_count: 2,
        default_state: 12660,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 12662,
        state_count: 2,
        default_state: 12662,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 12664,
        state_count: 2,
        default_state: 12664,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 12666,
        state_count: 2,
        default_state: 12666,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 12668,
        state_count: 2,
        default_state: 12668,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 12670,
        state_count: 2,
        default_state: 12670,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 12672,
        state_count: 2,
        default_state: 12672,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 12674,
        state_count: 2,
        default_state: 12674,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 12676,
        state_count: 2,
        default_state: 12676,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 12678,
        state_count: 2,
        default_state: 12678,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 12680,
        state_count: 2,
        default_state: 12680,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 12682,
        state_count: 2,
        default_state: 12682,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 12684,
        state_count: 2,
        default_state: 12684,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 12686,
        state_count: 2,
        default_state: 12686,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 12688,
        state_count: 2,
        default_state: 12688,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 12690,
        state_count: 2,
        default_state: 12690,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 12692,
        state_count: 2,
        default_state: 12692,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 12694,
        state_count: 8,
        default_state: 12694,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dead_brain_coral_wall_fan",
        first_state: 12702,
        state_count: 8,
        default_state: 12702,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dead_bubble_coral_wall_fan",
        first_state: 12710,
        state_count: 8,
        default_state: 12710,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dead_fire_coral_wall_fan",
        first_state: 12718,
        state_count: 8,
        default_state: 12718,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dead_horn_coral_wall_fan",
        first_state: 12726,
        state_count: 8,
        default_state: 12726,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tube_coral_wall_fan",
        first_state: 12734,
        state_count: 8,
        default_state: 12734,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brain_coral_wall_fan",
        first_state: 12742,
        state_count: 8,
        default_state: 12742,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "bubble_coral_wall_fan",
        first_state: 12750,
        state_count: 8,
        default_state: 12750,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "fire_coral_wall_fan",
        first_state: 12758,
        state_count: 8,
        default_state: 12758,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "horn_coral_wall_fan",
        first_state: 12766,
        state_count: 8,
        default_state: 12766,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sea_pickle",
        first_state: 12774,
        state_count: 8,
        default_state: 12774,
        props: &[
            PropInfo {
                name: "pickles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "blue_ice",
        first_state: 12782,
        state_count: 1,
        default_state: 12782,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 12783,
        state_count: 2,
        default_state: 12783,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 12785,
        state_count: 1,
        default_state: 12785,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 12786,
        state_count: 12,
        default_state: 12786,
        props: &[
            PropInfo {
                name: "age",
                values: &["0", "1"],
            },
            PropInfo {
                name: "leaves",
                values: &["none", "small", "large"],
            },
            PropInfo {
                name: "stage",
                values: &["0", "1"],
            },
        ],
    },
    BlockInfo {
        name: "potted_bamboo",
        first_state: 12798,
        state_count: 1,
        default_state: 12798,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 12799,
        state_count: 1,
        default_state: 12799,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 12800,
        state_count: 1,
        default_state: 12800,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 12801,
        state_count: 2,
        default_state: 12801,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 12803,
        state_count: 80,
        default_state: 12814,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_red_sandstone_stairs",
        first_state: 12883,
        state_count: 80,
        default_state: 12894,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_stone_brick_stairs",
        first_state: 12963,
        state_count: 80,
        default_state: 12974,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_diorite_stairs",
        first_state: 13043,
        state_count: 80,
        default_state: 13054,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_cobblestone_stairs",
        first_state: 13123,
        state_count: 80,
        default_state: 13134,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "end_stone_brick_stairs",
        first_state: 13203,
        state_count: 80,
        default_state: 13214,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "stone_stairs",
        first_state: 13283,
        state_count: 80,
        default_state: 13294,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_sandstone_stairs",
        first_state: 13363,
        state_count: 80,
        default_state: 13374,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_quartz_stairs",
        first_state: 13443,
        state_count: 80,
        default_state: 13454,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "granite_stairs",
        first_state: 13523,
        state_count: 80,
        default_state: 13534,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "andesite_stairs",
        first_state: 13603,
        state_count: 80,
        default_state: 13614,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_nether_brick_stairs",
        first_state: 13683,
        state_count: 80,
        default_state: 13694,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_andesite_stairs",
        first_state: 13763,
        state_count: 80,
        default_state: 13774,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "diorite_stairs",
        first_state: 13843,
        state_count: 80,
        default_state: 13854,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_granite_slab",
        first_state: 13923,
        state_count: 6,
        default_state: 13926,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_red_sandstone_slab",
        first_state: 13929,
        state_count: 6,
        default_state: 13932,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_stone_brick_slab",
        first_state: 13935,
        state_count: 6,
        default_state: 13938,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_diorite_slab",
        first_state: 13941,
        state_count: 6,
        default_state: 13944,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_cobblestone_slab",
        first_state: 13947,
        state_count: 6,
        default_state: 13950,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "end_stone_brick_slab",
        first_state: 13953,
        state_count: 6,
        default_state: 13956,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_sandstone_slab",
        first_state: 13959,
        state_count: 6,
        default_state: 13962,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smooth_quartz_slab",
        first_state: 13965,
        state_count: 6,
        default_state: 13968,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "granite_slab",
        first_state: 13971,
        state_count: 6,
        default_state: 13974,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "andesite_slab",
        first_state: 13977,
        state_count: 6,
        default_state: 13980,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_nether_brick_slab",
        first_state: 13983,
        state_count: 6,
        default_state: 13986,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_andesite_slab",
        first_state: 13989,
        state_count: 6,
        default_state: 13992,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "diorite_slab",
        first_state: 13995,
        state_count: 6,
        default_state: 13998,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brick_wall",
        first_state: 14001,
        state_count: 324,
        default_state: 14004,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "prismarine_wall",
        first_state: 14325,
        state_count: 324,
        default_state: 14328,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "red_sandstone_wall",
        first_state: 14649,
        state_count: 324,
        default_state: 14652,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "mossy_stone_brick_wall",
        first_state: 14973,
        state_count: 324,
        default_state: 14976,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "granite_wall",
        first_state: 15297,
        state_count: 324,
        default_state: 15300,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "stone_brick_wall",
        first_state: 15621,
        state_count: 324,
        default_state: 15624,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "mud_brick_wall",
        first_state: 15945,
        state_count: 324,
        default_state: 15948,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "nether_brick_wall",
        first_state: 16269,
        state_count: 324,
        default_state: 16272,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "andesite_wall",
        first_state: 16593,
        state_count: 324,
        default_state: 16596,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "red_nether_brick_wall",
        first_state: 16917,
        state_count: 324,
        default_state: 16920,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "sandstone_wall",
        first_state: 17241,
        state_count: 324,
        default_state: 17244,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "end_stone_brick_wall",
        first_state: 17565,
        state_count: 324,
        default_state: 17568,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "diorite_wall",
        first_state: 17889,
        state_count: 324,
        default_state: 17892,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "scaffolding",
        first_state: 18213,
        state_count: 32,
        default_state: 18244,
        props: &[
            PropInfo {
                name: "bottom",
                values: &["true", "false"],
            },
            PropInfo {
                name: "distance",
                values: &["0", "1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "loom",
        first_state: 18245,
        state_count: 4,
        default_state: 18245,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 18249,
        state_count: 12,
        default_state: 18250,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smoker",
        first_state: 18261,
        state_count: 8,
        default_state: 18262,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "blast_furnace",
        first_state: 18269,
        state_count: 8,
        default_state: 18270,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cartography_table",
        first_state: 18277,
        state_count: 1,
        default_state: 18277,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 18278,
        state_count: 1,
        default_state: 18278,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 18279,
        state_count: 12,
        default_state: 18283,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
        ],
    },
    BlockInfo {
        name: "lectern",
        first_state: 18291,
        state_count: 16,
        default_state: 18294,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "has_book",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "smithing_table",
        first_state: 18307,
        state_count: 1,
        default_state: 18307,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 18308,
        state_count: 4,
        default_state: 18308,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 18312,
        state_count: 32,
        default_state: 18313,
        props: &[
            PropInfo {
                name: "attachment",
                values: &["floor", "ceiling", "single_wall", "double_wall"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lantern",
        first_state: 18344,
        state_count: 4,
        default_state: 18347,
        props: &[
            PropInfo {
                name: "hanging",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "soul_lantern",
        first_state: 18348,
        state_count: 4,
        default_state: 18351,
        props: &[
            PropInfo {
                name: "hanging",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "campfire",
        first_state: 18352,
        state_count: 32,
        default_state: 18355,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "signal_fire",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "soul_campfire",
        first_state: 18384,
        state_count: 32,
        default_state: 18387,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "signal_fire",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sweet_berry_bush",
        first_state: 18416,
        state_count: 4,
        default_state: 18416,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 18420,
        state_count: 3,
        default_state: 18421,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 18423,
        state_count: 3,
        default_state: 18424,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 18426,
        state_count: 3,
        default_state: 18427,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 18429,
        state_count: 3,
        default_state: 18430,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 18432,
        state_count: 1,
        default_state: 18432,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 18433,
        state_count: 1,
        default_state: 18433,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 18434,
        state_count: 1,
        default_state: 18434,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 18435,
        state_count: 1,
        default_state: 18435,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 18436,
        state_count: 1,
        default_state: 18436,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 18437,
        state_count: 3,
        default_state: 18438,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 18440,
        state_count: 3,
        default_state: 18441,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 18443,
        state_count: 3,
        default_state: 18444,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 18446,
        state_count: 3,
        default_state: 18447,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 18449,
        state_count: 1,
        default_state: 18449,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 18450,
        state_count: 1,
        default_state: 18450,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 18451,
        state_count: 1,
        default_state: 18451,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 18452,
        state_count: 26,
        default_state: 18452,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
            ],
        }],
    },
    BlockInfo {
        name: "weeping_vines_plant",
        first_state: 18478,
        state_count: 1,
        default_state: 18478,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 18479,
        state_count: 26,
        default_state: 18479,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
            ],
        }],
    },
    BlockInfo {
        name: "twisting_vines_plant",
        first_state: 18505,
        state_count: 1,
        default_state: 18505,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 18506,
        state_count: 1,
        default_state: 18506,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 18507,
        state_count: 1,
        default_state: 18507,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 18508,
        state_count: 1,
        default_state: 18508,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 18509,
        state_count: 6,
        default_state: 18512,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_slab",
        first_state: 18515,
        state_count: 6,
        default_state: 18518,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_pressure_plate",
        first_state: 18521,
        state_count: 2,
        default_state: 18522,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 18523,
        state_count: 2,
        default_state: 18524,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 18525,
        state_count: 32,
        default_state: 18556,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_fence",
        first_state: 18557,
        state_count: 32,
        default_state: 18588,
        props: &[
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_trapdoor",
        first_state: 18589,
        state_count: 64,
        default_state: 18604,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_trapdoor",
        first_state: 18653,
        state_count: 64,
        default_state: 18668,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_fence_gate",
        first_state: 18717,
        state_count: 32,
        default_state: 18724,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_fence_gate",
        first_state: 18749,
        state_count: 32,
        default_state: 18756,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "in_wall",
                values: &["true", "false"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_stairs",
        first_state: 18781,
        state_count: 80,
        default_state: 18792,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_stairs",
        first_state: 18861,
        state_count: 80,
        default_state: 18872,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_button",
        first_state: 18941,
        state_count: 24,
        default_state: 18950,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_button",
        first_state: 18965,
        state_count: 24,
        default_state: 18974,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_door",
        first_state: 18989,
        state_count: 64,
        default_state: 19000,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_door",
        first_state: 19053,
        state_count: 64,
        default_state: 19064,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "hinge",
                values: &["left", "right"],
            },
            PropInfo {
                name: "open",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_sign",
        first_state: 19117,
        state_count: 32,
        default_state: 19118,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_sign",
        first_state: 19149,
        state_count: 32,
        default_state: 19150,
        props: &[
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "crimson_wall_sign",
        first_state: 19181,
        state_count: 8,
        default_state: 19182,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "warped_wall_sign",
        first_state: 19189,
        state_count: 8,
        default_state: 19190,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "structure_block",
        first_state: 19197,
        state_count: 4,
        default_state: 19198,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 19201,
        state_count: 12,
        default_state: 19211,
        props: &[PropInfo {
            name: "orientation",
            values: &[
                "down_east",
                "down_north",
                "down_south",
                "down_west",
                "up_east",
                "up_north",
                "up_south",
                "up_west",
                "west_up",
                "east_up",
                "north_up",
                "south_up",
            ],
        }],
    },
    BlockInfo {
        name: "composter",
        first_state: 19213,
        state_count: 9,
        default_state: 19213,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 19222,
        state_count: 16,
        default_state: 19222,
        props: &[PropInfo {
            name: "power",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "bee_nest",
        first_state: 19238,
        state_count: 24,
        default_state: 19238,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "honey_level",
                values: &["0", "1", "2", "3", "4", "5"],
            },
        ],
    },
    BlockInfo {
        name: "beehive",
        first_state: 19262,
        state_count: 24,
        default_state: 19262,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "honey_level",
                values: &["0", "1", "2", "3", "4", "5"],
            },
        ],
    },
    BlockInfo {
        name: "honey_block",
        first_state: 19286,
        state_count: 1,
        default_state: 19286,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 19287,
        state_count: 1,
        default_state: 19287,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 19288,
        state_count: 1,
        default_state: 19288,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 19289,
        state_count: 1,
        default_state: 19289,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 19290,
        state_count: 1,
        default_state: 19290,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 19291,
        state_count: 5,
        default_state: 19291,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 19296,
        state_count: 1,
        default_state: 19296,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 19297,
        state_count: 1,
        default_state: 19297,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 19298,
        state_count: 1,
        default_state: 19298,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 19299,
        state_count: 1,
        default_state: 19299,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 19300,
        state_count: 1,
        default_state: 19300,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 19301,
        state_count: 1,
        default_state: 19301,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 19302,
        state_count: 80,
        default_state: 19313,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "blackstone_wall",
        first_state: 19382,
        state_count: 324,
        default_state: 19385,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "blackstone_slab",
        first_state: 19706,
        state_count: 6,
        default_state: 19709,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone",
        first_state: 19712,
        state_count: 1,
        default_state: 19712,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 19713,
        state_count: 1,
        default_state: 19713,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 19714,
        state_count: 1,
        default_state: 19714,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 19715,
        state_count: 1,
        default_state: 19715,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 19716,
        state_count: 6,
        default_state: 19719,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone_brick_stairs",
        first_state: 19722,
        state_count: 80,
        default_state: 19733,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone_brick_wall",
        first_state: 19802,
        state_count: 324,
        default_state: 19805,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "gilded_blackstone",
        first_state: 20126,
        state_count: 1,
        default_state: 20126,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 20127,
        state_count: 80,
        default_state: 20138,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone_slab",
        first_state: 20207,
        state_count: 6,
        default_state: 20210,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone_pressure_plate",
        first_state: 20213,
        state_count: 2,
        default_state: 20214,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 20215,
        state_count: 24,
        default_state: 20224,
        props: &[
            PropInfo {
                name: "face",
                values: &["floor", "wall", "ceiling"],
            },
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_blackstone_wall",
        first_state: 20239,
        state_count: 324,
        default_state: 20242,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "chiseled_nether_bricks",
        first_state: 20563,
        state_count: 1,
        default_state: 20563,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 20564,
        state_count: 1,
        default_state: 20564,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 20565,
        state_count: 1,
        default_state: 20565,
        props: &[],
    },
    BlockInfo {
        name: "candle",
        first_state: 20566,
        state_count: 16,
        default_state: 20569,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "white_candle",
        first_state: 20582,
        state_count: 16,
        default_state: 20585,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "orange_candle",
        first_state: 20598,
        state_count: 16,
        default_state: 20601,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "magenta_candle",
        first_state: 20614,
        state_count: 16,
        default_state: 20617,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "light_blue_candle",
        first_state: 20630,
        state_count: 16,
        default_state: 20633,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "yellow_candle",
        first_state: 20646,
        state_count: 16,
        default_state: 20649,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lime_candle",
        first_state: 20662,
        state_count: 16,
        default_state: 20665,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pink_candle",
        first_state: 20678,
        state_count: 16,
        default_state: 20681,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "gray_candle",
        first_state: 20694,
        state_count: 16,
        default_state: 20697,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "light_gray_candle",
        first_state: 20710,
        state_count: 16,
        default_state: 20713,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cyan_candle",
        first_state: 20726,
        state_count: 16,
        default_state: 20729,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "purple_candle",
        first_state: 20742,
        state_count: 16,
        default_state: 20745,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "blue_candle",
        first_state: 20758,
        state_count: 16,
        default_state: 20761,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "brown_candle",
        first_state: 20774,
        state_count: 16,
        default_state: 20777,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "green_candle",
        first_state: 20790,
        state_count: 16,
        default_state: 20793,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "red_candle",
        first_state: 20806,
        state_count: 16,
        default_state: 20809,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "black_candle",
        first_state: 20822,
        state_count: 16,
        default_state: 20825,
        props: &[
            PropInfo {
                name: "candles",
                values: &["1", "2", "3", "4"],
            },
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "candle_cake",
        first_state: 20838,
        state_count: 2,
        default_state: 20839,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "white_candle_cake",
        first_state: 20840,
        state_count: 2,
        default_state: 20841,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "orange_candle_cake",
        first_state: 20842,
        state_count: 2,
        default_state: 20843,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "magenta_candle_cake",
        first_state: 20844,
        state_count: 2,
        default_state: 20845,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_blue_candle_cake",
        first_state: 20846,
        state_count: 2,
        default_state: 20847,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "yellow_candle_cake",
        first_state: 20848,
        state_count: 2,
        default_state: 20849,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lime_candle_cake",
        first_state: 20850,
        state_count: 2,
        default_state: 20851,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pink_candle_cake",
        first_state: 20852,
        state_count: 2,
        default_state: 20853,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "gray_candle_cake",
        first_state: 20854,
        state_count: 2,
        default_state: 20855,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_gray_candle_cake",
        first_state: 20856,
        state_count: 2,
        default_state: 20857,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cyan_candle_cake",
        first_state: 20858,
        state_count: 2,
        default_state: 20859,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "purple_candle_cake",
        first_state: 20860,
        state_count: 2,
        default_state: 20861,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "blue_candle_cake",
        first_state: 20862,
        state_count: 2,
        default_state: 20863,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brown_candle_cake",
        first_state: 20864,
        state_count: 2,
        default_state: 20865,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "green_candle_cake",
        first_state: 20866,
        state_count: 2,
        default_state: 20867,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "red_candle_cake",
        first_state: 20868,
        state_count: 2,
        default_state: 20869,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "black_candle_cake",
        first_state: 20870,
        state_count: 2,
        default_state: 20871,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "amethyst_block",
        first_state: 20872,
        state_count: 1,
        default_state: 20872,
        props: &[],
    },
    BlockInfo {
        name: "budding_amethyst",
        first_state: 20873,
        state_count: 1,
        default_state: 20873,
        props: &[],
    },
    BlockInfo {
        name: "amethyst_cluster",
        first_state: 20874,
        state_count: 12,
        default_state: 20883,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "large_amethyst_bud",
        first_state: 20886,
        state_count: 12,
        default_state: 20895,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "medium_amethyst_bud",
        first_state: 20898,
        state_count: 12,
        default_state: 20907,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "small_amethyst_bud",
        first_state: 20910,
        state_count: 12,
        default_state: 20919,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tuff",
        first_state: 20922,
        state_count: 1,
        default_state: 20922,
        props: &[],
    },
    BlockInfo {
        name: "calcite",
        first_state: 20923,
        state_count: 1,
        default_state: 20923,
        props: &[],
    },
    BlockInfo {
        name: "tinted_glass",
        first_state: 20924,
        state_count: 1,
        default_state: 20924,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow",
        first_state: 20925,
        state_count: 1,
        default_state: 20925,
        props: &[],
    },
    BlockInfo {
        name: "sculk_sensor",
        first_state: 20926,
        state_count: 96,
        default_state: 20927,
        props: &[
            PropInfo {
                name: "power",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
            PropInfo {
                name: "sculk_sensor_phase",
                values: &["inactive", "active", "cooldown"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sculk",
        first_state: 21022,
        state_count: 1,
        default_state: 21022,
        props: &[],
    },
    BlockInfo {
        name: "sculk_vein",
        first_state: 21023,
        state_count: 128,
        default_state: 21150,
        props: &[
            PropInfo {
                name: "down",
                values: &["true", "false"],
            },
            PropInfo {
                name: "east",
                values: &["true", "false"],
            },
            PropInfo {
                name: "north",
                values: &["true", "false"],
            },
            PropInfo {
                name: "south",
                values: &["true", "false"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sculk_catalyst",
        first_state: 21151,
        state_count: 2,
        default_state: 21152,
        props: &[PropInfo {
            name: "bloom",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sculk_shrieker",
        first_state: 21153,
        state_count: 8,
        default_state: 21160,
        props: &[
            PropInfo {
                name: "can_summon",
                values: &["true", "false"],
            },
            PropInfo {
                name: "shrieking",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oxidized_copper",
        first_state: 21161,
        state_count: 1,
        default_state: 21161,
        props: &[],
    },
    BlockInfo {
        name: "weathered_copper",
        first_state: 21162,
        state_count: 1,
        default_state: 21162,
        props: &[],
    },
    BlockInfo {
        name: "exposed_copper",
        first_state: 21163,
        state_count: 1,
        default_state: 21163,
        props: &[],
    },
    BlockInfo {
        name: "copper_block",
        first_state: 21164,
        state_count: 1,
        default_state: 21164,
        props: &[],
    },
    BlockInfo {
        name: "copper_ore",
        first_state: 21165,
        state_count: 1,
        default_state: 21165,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_copper_ore",
        first_state: 21166,
        state_count: 1,
        default_state: 21166,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper",
        first_state: 21167,
        state_count: 1,
        default_state: 21167,
        props: &[],
    },
    BlockInfo {
        name: "weathered_cut_copper",
        first_state: 21168,
        state_count: 1,
        default_state: 21168,
        props: &[],
    },
    BlockInfo {
        name: "exposed_cut_copper",
        first_state: 21169,
        state_count: 1,
        default_state: 21169,
        props: &[],
    },
    BlockInfo {
        name: "cut_copper",
        first_state: 21170,
        state_count: 1,
        default_state: 21170,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper_stairs",
        first_state: 21171,
        state_count: 80,
        default_state: 21182,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "weathered_cut_copper_stairs",
        first_state: 21251,
        state_count: 80,
        default_state: 21262,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "exposed_cut_copper_stairs",
        first_state: 21331,
        state_count: 80,
        default_state: 21342,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cut_copper_stairs",
        first_state: 21411,
        state_count: 80,
        default_state: 21422,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oxidized_cut_copper_slab",
        first_state: 21491,
        state_count: 6,
        default_state: 21494,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "weathered_cut_copper_slab",
        first_state: 21497,
        state_count: 6,
        default_state: 21500,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "exposed_cut_copper_slab",
        first_state: 21503,
        state_count: 6,
        default_state: 21506,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cut_copper_slab",
        first_state: 21509,
        state_count: 6,
        default_state: 21512,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_copper_block",
        first_state: 21515,
        state_count: 1,
        default_state: 21515,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_copper",
        first_state: 21516,
        state_count: 1,
        default_state: 21516,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_copper",
        first_state: 21517,
        state_count: 1,
        default_state: 21517,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_copper",
        first_state: 21518,
        state_count: 1,
        default_state: 21518,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper",
        first_state: 21519,
        state_count: 1,
        default_state: 21519,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper",
        first_state: 21520,
        state_count: 1,
        default_state: 21520,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper",
        first_state: 21521,
        state_count: 1,
        default_state: 21521,
        props: &[],
    },
    BlockInfo {
        name: "waxed_cut_copper",
        first_state: 21522,
        state_count: 1,
        default_state: 21522,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_stairs",
        first_state: 21523,
        state_count: 80,
        default_state: 21534,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper_stairs",
        first_state: 21603,
        state_count: 80,
        default_state: 21614,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper_stairs",
        first_state: 21683,
        state_count: 80,
        default_state: 21694,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_cut_copper_stairs",
        first_state: 21763,
        state_count: 80,
        default_state: 21774,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_slab",
        first_state: 21843,
        state_count: 6,
        default_state: 21846,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper_slab",
        first_state: 21849,
        state_count: 6,
        default_state: 21852,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper_slab",
        first_state: 21855,
        state_count: 6,
        default_state: 21858,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_cut_copper_slab",
        first_state: 21861,
        state_count: 6,
        default_state: 21864,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lightning_rod",
        first_state: 21867,
        state_count: 24,
        default_state: 21886,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "east", "south", "west", "up", "down"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pointed_dripstone",
        first_state: 21891,
        state_count: 20,
        default_state: 21896,
        props: &[
            PropInfo {
                name: "thickness",
                values: &["tip_merge", "tip", "frustum", "middle", "base"],
            },
            PropInfo {
                name: "vertical_direction",
                values: &["up", "down"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dripstone_block",
        first_state: 21911,
        state_count: 1,
        default_state: 21911,
        props: &[],
    },
    BlockInfo {
        name: "cave_vines",
        first_state: 21912,
        state_count: 52,
        default_state: 21913,
        props: &[
            PropInfo {
                name: "age",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
                ],
            },
            PropInfo {
                name: "berries",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cave_vines_plant",
        first_state: 21964,
        state_count: 2,
        default_state: 21965,
        props: &[PropInfo {
            name: "berries",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spore_blossom",
        first_state: 21966,
        state_count: 1,
        default_state: 21966,
        props: &[],
    },
    BlockInfo {
        name: "azalea",
        first_state: 21967,
        state_count: 1,
        default_state: 21967,
        props: &[],
    },
    BlockInfo {
        name: "flowering_azalea",
        first_state: 21968,
        state_count: 1,
        default_state: 21968,
        props: &[],
    },
    BlockInfo {
        name: "moss_carpet",
        first_state: 21969,
        state_count: 1,
        default_state: 21969,
        props: &[],
    },
    BlockInfo {
        name: "pink_petals",
        first_state: 21970,
        state_count: 16,
        default_state: 21970,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "flower_amount",
                values: &["1", "2", "3", "4"],
            },
        ],
    },
    BlockInfo {
        name: "moss_block",
        first_state: 21986,
        state_count: 1,
        default_state: 21986,
        props: &[],
    },
    BlockInfo {
        name: "big_dripleaf",
        first_state: 21987,
        state_count: 32,
        default_state: 21988,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "tilt",
                values: &["none", "unstable", "partial", "full"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "big_dripleaf_stem",
        first_state: 22019,
        state_count: 8,
        default_state: 22020,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "small_dripleaf",
        first_state: 22027,
        state_count: 16,
        default_state: 22030,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "hanging_roots",
        first_state: 22043,
        state_count: 2,
        default_state: 22044,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "rooted_dirt",
        first_state: 22045,
        state_count: 1,
        default_state: 22045,
        props: &[],
    },
    BlockInfo {
        name: "mud",
        first_state: 22046,
        state_count: 1,
        default_state: 22046,
        props: &[],
    },
    BlockInfo {
        name: "deepslate",
        first_state: 22047,
        state_count: 3,
        default_state: 22048,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cobbled_deepslate",
        first_state: 22050,
        state_count: 1,
        default_state: 22050,
        props: &[],
    },
    BlockInfo {
        name: "cobbled_deepslate_stairs",
        first_state: 22051,
        state_count: 80,
        default_state: 22062,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cobbled_deepslate_slab",
        first_state: 22131,
        state_count: 6,
        default_state: 22134,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "cobbled_deepslate_wall",
        first_state: 22137,
        state_count: 324,
        default_state: 22140,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "polished_deepslate",
        first_state: 22461,
        state_count: 1,
        default_state: 22461,
        props: &[],
    },
    BlockInfo {
        name: "polished_deepslate_stairs",
        first_state: 22462,
        state_count: 80,
        default_state: 22473,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_deepslate_slab",
        first_state: 22542,
        state_count: 6,
        default_state: 22545,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_deepslate_wall",
        first_state: 22548,
        state_count: 324,
        default_state: 22551,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_tiles",
        first_state: 22872,
        state_count: 1,
        default_state: 22872,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_tile_stairs",
        first_state: 22873,
        state_count: 80,
        default_state: 22884,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_tile_slab",
        first_state: 22953,
        state_count: 6,
        default_state: 22956,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_tile_wall",
        first_state: 22959,
        state_count: 324,
        default_state: 22962,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_bricks",
        first_state: 23283,
        state_count: 1,
        default_state: 23283,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_brick_stairs",
        first_state: 23284,
        state_count: 80,
        default_state: 23295,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_brick_slab",
        first_state: 23364,
        state_count: 6,
        default_state: 23367,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "deepslate_brick_wall",
        first_state: 23370,
        state_count: 324,
        default_state: 23373,
        props: &[
            PropInfo {
                name: "east",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "north",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "south",
                values: &["none", "low", "tall"],
            },
            PropInfo {
                name: "up",
                values: &["true", "false"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
            PropInfo {
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "chiseled_deepslate",
        first_state: 23694,
        state_count: 1,
        default_state: 23694,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_bricks",
        first_state: 23695,
        state_count: 1,
        default_state: 23695,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_tiles",
        first_state: 23696,
        state_count: 1,
        default_state: 23696,
        props: &[],
    },
    BlockInfo {
        name: "infested_deepslate",
        first_state: 23697,
        state_count: 3,
        default_state: 23698,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "smooth_basalt",
        first_state: 23700,
        state_count: 1,
        default_state: 23700,
        props: &[],
    },
    BlockInfo {
        name: "raw_iron_block",
        first_state: 23701,
        state_count: 1,
        default_state: 23701,
        props: &[],
    },
    BlockInfo {
        name: "raw_copper_block",
        first_state: 23702,
        state_count: 1,
        default_state: 23702,
        props: &[],
    },
    BlockInfo {
        name: "raw_gold_block",
        first_state: 23703,
        state_count: 1,
        default_state: 23703,
        props: &[],
    },
    BlockInfo {
        name: "potted_azalea_bush",
        first_state: 23704,
        state_count: 1,
        default_state: 23704,
        props: &[],
    },
    BlockInfo {
        name: "potted_flowering_azalea_bush",
        first_state: 23705,
        state_count: 1,
        default_state: 23705,
        props: &[],
    },
    BlockInfo {
        name: "ochre_froglight",
        first_state: 23706,
        state_count: 3,
        default_state: 23707,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "verdant_froglight",
        first_state: 23709,
        state_count: 3,
        default_state: 23710,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pearlescent_froglight",
        first_state: 23712,
        state_count: 3,
        default_state: 23713,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "frogspawn",
        first_state: 23715,
        state_count: 1,
        default_state: 23715,
        props: &[],
    },
    BlockInfo {
        name: "reinforced_deepslate",
        first_state: 23716,
        state_count: 1,
        default_state: 23716,
        props: &[],
    },
    BlockInfo {
        name: "decorated_pot",
        first_state: 23717,
        state_count: 8,
        default_state: 23718,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
];
