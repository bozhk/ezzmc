// @generated `cargo xtask gen-blockstates 1.19.3` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.19.3";
pub const STATE_COUNT: u32 = 23232;

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
        name: "dark_oak_planks",
        first_state: 20,
        state_count: 1,
        default_state: 20,
        props: &[],
    },
    BlockInfo {
        name: "mangrove_planks",
        first_state: 21,
        state_count: 1,
        default_state: 21,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_planks",
        first_state: 22,
        state_count: 1,
        default_state: 22,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_mosaic",
        first_state: 23,
        state_count: 1,
        default_state: 23,
        props: &[],
    },
    BlockInfo {
        name: "oak_sapling",
        first_state: 24,
        state_count: 2,
        default_state: 24,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "spruce_sapling",
        first_state: 26,
        state_count: 2,
        default_state: 26,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "birch_sapling",
        first_state: 28,
        state_count: 2,
        default_state: 28,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "jungle_sapling",
        first_state: 30,
        state_count: 2,
        default_state: 30,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "acacia_sapling",
        first_state: 32,
        state_count: 2,
        default_state: 32,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "dark_oak_sapling",
        first_state: 34,
        state_count: 2,
        default_state: 34,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "mangrove_propagule",
        first_state: 36,
        state_count: 40,
        default_state: 41,
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
        first_state: 76,
        state_count: 1,
        default_state: 76,
        props: &[],
    },
    BlockInfo {
        name: "water",
        first_state: 77,
        state_count: 16,
        default_state: 77,
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
        first_state: 93,
        state_count: 16,
        default_state: 93,
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
        first_state: 109,
        state_count: 1,
        default_state: 109,
        props: &[],
    },
    BlockInfo {
        name: "red_sand",
        first_state: 110,
        state_count: 1,
        default_state: 110,
        props: &[],
    },
    BlockInfo {
        name: "gravel",
        first_state: 111,
        state_count: 1,
        default_state: 111,
        props: &[],
    },
    BlockInfo {
        name: "gold_ore",
        first_state: 112,
        state_count: 1,
        default_state: 112,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_gold_ore",
        first_state: 113,
        state_count: 1,
        default_state: 113,
        props: &[],
    },
    BlockInfo {
        name: "iron_ore",
        first_state: 114,
        state_count: 1,
        default_state: 114,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_iron_ore",
        first_state: 115,
        state_count: 1,
        default_state: 115,
        props: &[],
    },
    BlockInfo {
        name: "coal_ore",
        first_state: 116,
        state_count: 1,
        default_state: 116,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_coal_ore",
        first_state: 117,
        state_count: 1,
        default_state: 117,
        props: &[],
    },
    BlockInfo {
        name: "nether_gold_ore",
        first_state: 118,
        state_count: 1,
        default_state: 118,
        props: &[],
    },
    BlockInfo {
        name: "oak_log",
        first_state: 119,
        state_count: 3,
        default_state: 120,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 122,
        state_count: 3,
        default_state: 123,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 125,
        state_count: 3,
        default_state: 126,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 128,
        state_count: 3,
        default_state: 129,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 131,
        state_count: 3,
        default_state: 132,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 134,
        state_count: 3,
        default_state: 135,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_log",
        first_state: 137,
        state_count: 3,
        default_state: 138,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_roots",
        first_state: 140,
        state_count: 2,
        default_state: 141,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "muddy_mangrove_roots",
        first_state: 142,
        state_count: 3,
        default_state: 143,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "bamboo_block",
        first_state: 145,
        state_count: 3,
        default_state: 146,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 148,
        state_count: 3,
        default_state: 149,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 151,
        state_count: 3,
        default_state: 152,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 154,
        state_count: 3,
        default_state: 155,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 157,
        state_count: 3,
        default_state: 158,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 160,
        state_count: 3,
        default_state: 161,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 163,
        state_count: 3,
        default_state: 164,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_log",
        first_state: 166,
        state_count: 3,
        default_state: 167,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_bamboo_block",
        first_state: 169,
        state_count: 3,
        default_state: 170,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 172,
        state_count: 3,
        default_state: 173,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 175,
        state_count: 3,
        default_state: 176,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 178,
        state_count: 3,
        default_state: 179,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 181,
        state_count: 3,
        default_state: 182,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 184,
        state_count: 3,
        default_state: 185,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 187,
        state_count: 3,
        default_state: 188,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_wood",
        first_state: 190,
        state_count: 3,
        default_state: 191,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 193,
        state_count: 3,
        default_state: 194,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 196,
        state_count: 3,
        default_state: 197,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 199,
        state_count: 3,
        default_state: 200,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 202,
        state_count: 3,
        default_state: 203,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 205,
        state_count: 3,
        default_state: 206,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 208,
        state_count: 3,
        default_state: 209,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_wood",
        first_state: 211,
        state_count: 3,
        default_state: 212,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 214,
        state_count: 28,
        default_state: 241,
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
        first_state: 242,
        state_count: 28,
        default_state: 269,
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
        first_state: 270,
        state_count: 28,
        default_state: 297,
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
        first_state: 298,
        state_count: 28,
        default_state: 325,
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
        first_state: 326,
        state_count: 28,
        default_state: 353,
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
        first_state: 354,
        state_count: 28,
        default_state: 381,
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
        first_state: 382,
        state_count: 28,
        default_state: 409,
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
        first_state: 410,
        state_count: 28,
        default_state: 437,
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
        first_state: 438,
        state_count: 28,
        default_state: 465,
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
        first_state: 466,
        state_count: 1,
        default_state: 466,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 467,
        state_count: 1,
        default_state: 467,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 468,
        state_count: 1,
        default_state: 468,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 469,
        state_count: 1,
        default_state: 469,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_lapis_ore",
        first_state: 470,
        state_count: 1,
        default_state: 470,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 471,
        state_count: 1,
        default_state: 471,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 472,
        state_count: 12,
        default_state: 473,
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
        first_state: 484,
        state_count: 1,
        default_state: 484,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 485,
        state_count: 1,
        default_state: 485,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 486,
        state_count: 1,
        default_state: 486,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 487,
        state_count: 1150,
        default_state: 488,
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
        first_state: 1637,
        state_count: 16,
        default_state: 1640,
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
        first_state: 1653,
        state_count: 16,
        default_state: 1656,
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
        first_state: 1669,
        state_count: 16,
        default_state: 1672,
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
        first_state: 1685,
        state_count: 16,
        default_state: 1688,
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
        first_state: 1701,
        state_count: 16,
        default_state: 1704,
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
        first_state: 1717,
        state_count: 16,
        default_state: 1720,
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
        first_state: 1733,
        state_count: 16,
        default_state: 1736,
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
        first_state: 1749,
        state_count: 16,
        default_state: 1752,
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
        first_state: 1765,
        state_count: 16,
        default_state: 1768,
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
        first_state: 1781,
        state_count: 16,
        default_state: 1784,
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
        first_state: 1797,
        state_count: 16,
        default_state: 1800,
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
        first_state: 1813,
        state_count: 16,
        default_state: 1816,
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
        first_state: 1829,
        state_count: 16,
        default_state: 1832,
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
        first_state: 1845,
        state_count: 16,
        default_state: 1848,
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
        first_state: 1861,
        state_count: 16,
        default_state: 1864,
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
        first_state: 1877,
        state_count: 16,
        default_state: 1880,
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
        first_state: 1893,
        state_count: 24,
        default_state: 1906,
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
        first_state: 1917,
        state_count: 24,
        default_state: 1930,
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
        first_state: 1941,
        state_count: 12,
        default_state: 1947,
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
        first_state: 1953,
        state_count: 1,
        default_state: 1953,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 1954,
        state_count: 1,
        default_state: 1954,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 1955,
        state_count: 1,
        default_state: 1955,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 1956,
        state_count: 1,
        default_state: 1956,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 1957,
        state_count: 1,
        default_state: 1957,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 1958,
        state_count: 2,
        default_state: 1959,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 1960,
        state_count: 12,
        default_state: 1966,
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
        first_state: 1972,
        state_count: 24,
        default_state: 1974,
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
        first_state: 1996,
        state_count: 1,
        default_state: 1996,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 1997,
        state_count: 1,
        default_state: 1997,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 1998,
        state_count: 1,
        default_state: 1998,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 1999,
        state_count: 1,
        default_state: 1999,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 2000,
        state_count: 1,
        default_state: 2000,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 2001,
        state_count: 1,
        default_state: 2001,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 2002,
        state_count: 1,
        default_state: 2002,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 2003,
        state_count: 1,
        default_state: 2003,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 2004,
        state_count: 1,
        default_state: 2004,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 2005,
        state_count: 1,
        default_state: 2005,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 2006,
        state_count: 1,
        default_state: 2006,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 2007,
        state_count: 1,
        default_state: 2007,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 2008,
        state_count: 1,
        default_state: 2008,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 2009,
        state_count: 1,
        default_state: 2009,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 2010,
        state_count: 1,
        default_state: 2010,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 2011,
        state_count: 1,
        default_state: 2011,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 2012,
        state_count: 12,
        default_state: 2012,
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
        first_state: 2024,
        state_count: 1,
        default_state: 2024,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 2025,
        state_count: 1,
        default_state: 2025,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 2026,
        state_count: 1,
        default_state: 2026,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 2027,
        state_count: 1,
        default_state: 2027,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 2028,
        state_count: 1,
        default_state: 2028,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 2029,
        state_count: 1,
        default_state: 2029,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 2030,
        state_count: 1,
        default_state: 2030,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 2031,
        state_count: 1,
        default_state: 2031,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 2032,
        state_count: 1,
        default_state: 2032,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 2033,
        state_count: 1,
        default_state: 2033,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 2034,
        state_count: 1,
        default_state: 2034,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 2035,
        state_count: 1,
        default_state: 2035,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 2036,
        state_count: 1,
        default_state: 2036,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 2037,
        state_count: 1,
        default_state: 2037,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 2038,
        state_count: 1,
        default_state: 2038,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 2039,
        state_count: 1,
        default_state: 2039,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 2040,
        state_count: 1,
        default_state: 2040,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 2041,
        state_count: 1,
        default_state: 2041,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 2042,
        state_count: 2,
        default_state: 2043,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 2044,
        state_count: 1,
        default_state: 2044,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_bookshelf",
        first_state: 2045,
        state_count: 256,
        default_state: 2108,
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
        first_state: 2301,
        state_count: 1,
        default_state: 2301,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 2302,
        state_count: 1,
        default_state: 2302,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 2303,
        state_count: 1,
        default_state: 2303,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 2304,
        state_count: 4,
        default_state: 2304,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 2308,
        state_count: 512,
        default_state: 2339,
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
        first_state: 2820,
        state_count: 1,
        default_state: 2820,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 2821,
        state_count: 1,
        default_state: 2821,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 2822,
        state_count: 80,
        default_state: 2833,
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
        first_state: 2902,
        state_count: 24,
        default_state: 2903,
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
        first_state: 2926,
        state_count: 1296,
        default_state: 4086,
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
        first_state: 4222,
        state_count: 1,
        default_state: 4222,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_diamond_ore",
        first_state: 4223,
        state_count: 1,
        default_state: 4223,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 4224,
        state_count: 1,
        default_state: 4224,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 4225,
        state_count: 1,
        default_state: 4225,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 4226,
        state_count: 8,
        default_state: 4226,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 4234,
        state_count: 8,
        default_state: 4234,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 4242,
        state_count: 8,
        default_state: 4243,
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
        first_state: 4250,
        state_count: 32,
        default_state: 4251,
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
        first_state: 4282,
        state_count: 32,
        default_state: 4283,
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
        first_state: 4314,
        state_count: 32,
        default_state: 4315,
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
        first_state: 4346,
        state_count: 32,
        default_state: 4347,
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
        first_state: 4378,
        state_count: 32,
        default_state: 4379,
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
        first_state: 4410,
        state_count: 32,
        default_state: 4411,
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
        first_state: 4442,
        state_count: 32,
        default_state: 4443,
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
        first_state: 4474,
        state_count: 32,
        default_state: 4475,
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
        first_state: 4506,
        state_count: 64,
        default_state: 4517,
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
        first_state: 4570,
        state_count: 8,
        default_state: 4571,
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
        first_state: 4578,
        state_count: 20,
        default_state: 4579,
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
        first_state: 4598,
        state_count: 80,
        default_state: 4609,
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
        first_state: 4678,
        state_count: 8,
        default_state: 4679,
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
        first_state: 4686,
        state_count: 8,
        default_state: 4687,
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
        first_state: 4694,
        state_count: 8,
        default_state: 4695,
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
        first_state: 4702,
        state_count: 8,
        default_state: 4703,
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
        first_state: 4710,
        state_count: 8,
        default_state: 4711,
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
        first_state: 4718,
        state_count: 8,
        default_state: 4719,
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
        first_state: 4726,
        state_count: 8,
        default_state: 4727,
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
        first_state: 4734,
        state_count: 8,
        default_state: 4735,
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
        first_state: 4742,
        state_count: 64,
        default_state: 4775,
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
        first_state: 4806,
        state_count: 64,
        default_state: 4839,
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
        first_state: 4870,
        state_count: 64,
        default_state: 4903,
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
        first_state: 4934,
        state_count: 64,
        default_state: 4967,
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
        first_state: 4998,
        state_count: 64,
        default_state: 5031,
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
        first_state: 5062,
        state_count: 64,
        default_state: 5095,
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
        first_state: 5126,
        state_count: 64,
        default_state: 5159,
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
        first_state: 5190,
        state_count: 64,
        default_state: 5223,
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
        first_state: 5254,
        state_count: 64,
        default_state: 5287,
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
        first_state: 5318,
        state_count: 64,
        default_state: 5351,
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
        first_state: 5382,
        state_count: 8,
        default_state: 5383,
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
        first_state: 5390,
        state_count: 8,
        default_state: 5391,
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
        first_state: 5398,
        state_count: 8,
        default_state: 5399,
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
        first_state: 5406,
        state_count: 8,
        default_state: 5407,
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
        first_state: 5414,
        state_count: 8,
        default_state: 5415,
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
        first_state: 5422,
        state_count: 8,
        default_state: 5423,
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
        first_state: 5430,
        state_count: 8,
        default_state: 5431,
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
        first_state: 5438,
        state_count: 8,
        default_state: 5439,
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
        first_state: 5446,
        state_count: 8,
        default_state: 5447,
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
        first_state: 5454,
        state_count: 8,
        default_state: 5455,
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
        first_state: 5462,
        state_count: 24,
        default_state: 5471,
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
        first_state: 5486,
        state_count: 2,
        default_state: 5487,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 5488,
        state_count: 64,
        default_state: 5499,
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
        first_state: 5552,
        state_count: 2,
        default_state: 5553,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 5554,
        state_count: 2,
        default_state: 5555,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 5556,
        state_count: 2,
        default_state: 5557,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 5558,
        state_count: 2,
        default_state: 5559,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 5560,
        state_count: 2,
        default_state: 5561,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 5562,
        state_count: 2,
        default_state: 5563,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "mangrove_pressure_plate",
        first_state: 5564,
        state_count: 2,
        default_state: 5565,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_pressure_plate",
        first_state: 5566,
        state_count: 2,
        default_state: 5567,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 5568,
        state_count: 2,
        default_state: 5569,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "deepslate_redstone_ore",
        first_state: 5570,
        state_count: 2,
        default_state: 5571,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 5572,
        state_count: 2,
        default_state: 5572,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 5574,
        state_count: 8,
        default_state: 5574,
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
        first_state: 5582,
        state_count: 24,
        default_state: 5591,
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
        first_state: 5606,
        state_count: 8,
        default_state: 5606,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 5614,
        state_count: 1,
        default_state: 5614,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 5615,
        state_count: 1,
        default_state: 5615,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 5616,
        state_count: 16,
        default_state: 5616,
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
        first_state: 5632,
        state_count: 1,
        default_state: 5632,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 5633,
        state_count: 16,
        default_state: 5633,
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
        first_state: 5649,
        state_count: 2,
        default_state: 5650,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 5651,
        state_count: 32,
        default_state: 5682,
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
        first_state: 5683,
        state_count: 1,
        default_state: 5683,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 5684,
        state_count: 1,
        default_state: 5684,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 5685,
        state_count: 1,
        default_state: 5685,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 5686,
        state_count: 1,
        default_state: 5686,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 5687,
        state_count: 3,
        default_state: 5688,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 5690,
        state_count: 3,
        default_state: 5691,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 5693,
        state_count: 1,
        default_state: 5693,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 5694,
        state_count: 4,
        default_state: 5694,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 5698,
        state_count: 1,
        default_state: 5698,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 5699,
        state_count: 2,
        default_state: 5699,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 5701,
        state_count: 4,
        default_state: 5701,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 5705,
        state_count: 4,
        default_state: 5705,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 5709,
        state_count: 7,
        default_state: 5709,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 5716,
        state_count: 64,
        default_state: 5719,
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
        first_state: 5780,
        state_count: 1,
        default_state: 5780,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 5781,
        state_count: 1,
        default_state: 5781,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 5782,
        state_count: 1,
        default_state: 5782,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 5783,
        state_count: 1,
        default_state: 5783,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 5784,
        state_count: 1,
        default_state: 5784,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 5785,
        state_count: 1,
        default_state: 5785,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 5786,
        state_count: 1,
        default_state: 5786,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 5787,
        state_count: 1,
        default_state: 5787,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 5788,
        state_count: 1,
        default_state: 5788,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 5789,
        state_count: 1,
        default_state: 5789,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 5790,
        state_count: 1,
        default_state: 5790,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 5791,
        state_count: 1,
        default_state: 5791,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 5792,
        state_count: 1,
        default_state: 5792,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 5793,
        state_count: 1,
        default_state: 5793,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 5794,
        state_count: 1,
        default_state: 5794,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 5795,
        state_count: 1,
        default_state: 5795,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 5796,
        state_count: 64,
        default_state: 5811,
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
        first_state: 5860,
        state_count: 64,
        default_state: 5875,
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
        first_state: 5924,
        state_count: 64,
        default_state: 5939,
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
        first_state: 5988,
        state_count: 64,
        default_state: 6003,
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
        first_state: 6052,
        state_count: 64,
        default_state: 6067,
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
        first_state: 6116,
        state_count: 64,
        default_state: 6131,
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
        first_state: 6180,
        state_count: 64,
        default_state: 6195,
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
        first_state: 6244,
        state_count: 64,
        default_state: 6259,
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
        first_state: 6308,
        state_count: 1,
        default_state: 6308,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 6309,
        state_count: 1,
        default_state: 6309,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 6310,
        state_count: 1,
        default_state: 6310,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 6311,
        state_count: 1,
        default_state: 6311,
        props: &[],
    },
    BlockInfo {
        name: "packed_mud",
        first_state: 6312,
        state_count: 1,
        default_state: 6312,
        props: &[],
    },
    BlockInfo {
        name: "mud_bricks",
        first_state: 6313,
        state_count: 1,
        default_state: 6313,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 6314,
        state_count: 1,
        default_state: 6314,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 6315,
        state_count: 1,
        default_state: 6315,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 6316,
        state_count: 1,
        default_state: 6316,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 6317,
        state_count: 1,
        default_state: 6317,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 6318,
        state_count: 1,
        default_state: 6318,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 6319,
        state_count: 1,
        default_state: 6319,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 6320,
        state_count: 64,
        default_state: 6320,
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
        first_state: 6384,
        state_count: 64,
        default_state: 6384,
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
        first_state: 6448,
        state_count: 64,
        default_state: 6448,
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
        first_state: 6512,
        state_count: 32,
        default_state: 6543,
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
        first_state: 6544,
        state_count: 6,
        default_state: 6547,
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
        first_state: 6550,
        state_count: 32,
        default_state: 6581,
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
        first_state: 6582,
        state_count: 1,
        default_state: 6582,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 6583,
        state_count: 4,
        default_state: 6583,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 6587,
        state_count: 4,
        default_state: 6587,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 6591,
        state_count: 8,
        default_state: 6591,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 6599,
        state_count: 8,
        default_state: 6599,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 6607,
        state_count: 32,
        default_state: 6638,
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
        first_state: 6639,
        state_count: 128,
        default_state: 6766,
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
        first_state: 6767,
        state_count: 32,
        default_state: 6774,
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
        first_state: 6799,
        state_count: 80,
        default_state: 6810,
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
        first_state: 6879,
        state_count: 80,
        default_state: 6890,
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
        first_state: 6959,
        state_count: 80,
        default_state: 6970,
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
        first_state: 7039,
        state_count: 2,
        default_state: 7040,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 7041,
        state_count: 1,
        default_state: 7041,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 7042,
        state_count: 1,
        default_state: 7042,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 7043,
        state_count: 32,
        default_state: 7074,
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
        first_state: 7075,
        state_count: 80,
        default_state: 7086,
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
        first_state: 7155,
        state_count: 4,
        default_state: 7155,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 7159,
        state_count: 1,
        default_state: 7159,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 7160,
        state_count: 8,
        default_state: 7167,
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
        first_state: 7168,
        state_count: 1,
        default_state: 7168,
        props: &[],
    },
    BlockInfo {
        name: "water_cauldron",
        first_state: 7169,
        state_count: 3,
        default_state: 7169,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "lava_cauldron",
        first_state: 7172,
        state_count: 1,
        default_state: 7172,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow_cauldron",
        first_state: 7173,
        state_count: 3,
        default_state: 7173,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 7176,
        state_count: 1,
        default_state: 7176,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 7177,
        state_count: 8,
        default_state: 7181,
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
        first_state: 7185,
        state_count: 1,
        default_state: 7185,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 7186,
        state_count: 1,
        default_state: 7186,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 7187,
        state_count: 2,
        default_state: 7188,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 7189,
        state_count: 12,
        default_state: 7189,
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
        first_state: 7201,
        state_count: 80,
        default_state: 7212,
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
        first_state: 7281,
        state_count: 1,
        default_state: 7281,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_emerald_ore",
        first_state: 7282,
        state_count: 1,
        default_state: 7282,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 7283,
        state_count: 8,
        default_state: 7284,
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
        first_state: 7291,
        state_count: 16,
        default_state: 7300,
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
        first_state: 7307,
        state_count: 128,
        default_state: 7434,
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
        first_state: 7435,
        state_count: 1,
        default_state: 7435,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 7436,
        state_count: 80,
        default_state: 7447,
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
        first_state: 7516,
        state_count: 80,
        default_state: 7527,
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
        first_state: 7596,
        state_count: 80,
        default_state: 7607,
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
        first_state: 7676,
        state_count: 12,
        default_state: 7682,
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
        first_state: 7688,
        state_count: 1,
        default_state: 7688,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 7689,
        state_count: 324,
        default_state: 7692,
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
        first_state: 8013,
        state_count: 324,
        default_state: 8016,
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
        first_state: 8337,
        state_count: 1,
        default_state: 8337,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 8338,
        state_count: 1,
        default_state: 8338,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 8339,
        state_count: 1,
        default_state: 8339,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 8340,
        state_count: 1,
        default_state: 8340,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 8341,
        state_count: 1,
        default_state: 8341,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 8342,
        state_count: 1,
        default_state: 8342,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 8343,
        state_count: 1,
        default_state: 8343,
        props: &[],
    },
    BlockInfo {
        name: "potted_mangrove_propagule",
        first_state: 8344,
        state_count: 1,
        default_state: 8344,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 8345,
        state_count: 1,
        default_state: 8345,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 8346,
        state_count: 1,
        default_state: 8346,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 8347,
        state_count: 1,
        default_state: 8347,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 8348,
        state_count: 1,
        default_state: 8348,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 8349,
        state_count: 1,
        default_state: 8349,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 8350,
        state_count: 1,
        default_state: 8350,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 8351,
        state_count: 1,
        default_state: 8351,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 8352,
        state_count: 1,
        default_state: 8352,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 8353,
        state_count: 1,
        default_state: 8353,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 8354,
        state_count: 1,
        default_state: 8354,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 8355,
        state_count: 1,
        default_state: 8355,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 8356,
        state_count: 1,
        default_state: 8356,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 8357,
        state_count: 1,
        default_state: 8357,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 8358,
        state_count: 1,
        default_state: 8358,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 8359,
        state_count: 1,
        default_state: 8359,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 8360,
        state_count: 1,
        default_state: 8360,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 8361,
        state_count: 1,
        default_state: 8361,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 8362,
        state_count: 1,
        default_state: 8362,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 8363,
        state_count: 8,
        default_state: 8363,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 8371,
        state_count: 8,
        default_state: 8371,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 8379,
        state_count: 24,
        default_state: 8388,
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
        first_state: 8403,
        state_count: 24,
        default_state: 8412,
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
        first_state: 8427,
        state_count: 24,
        default_state: 8436,
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
        first_state: 8451,
        state_count: 24,
        default_state: 8460,
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
        first_state: 8475,
        state_count: 24,
        default_state: 8484,
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
        first_state: 8499,
        state_count: 24,
        default_state: 8508,
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
        first_state: 8523,
        state_count: 24,
        default_state: 8532,
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
        first_state: 8547,
        state_count: 24,
        default_state: 8556,
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
        first_state: 8571,
        state_count: 16,
        default_state: 8571,
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
        first_state: 8587,
        state_count: 4,
        default_state: 8587,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 8591,
        state_count: 16,
        default_state: 8591,
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
        first_state: 8607,
        state_count: 4,
        default_state: 8607,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 8611,
        state_count: 16,
        default_state: 8611,
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
        first_state: 8627,
        state_count: 4,
        default_state: 8627,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 8631,
        state_count: 16,
        default_state: 8631,
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
        first_state: 8647,
        state_count: 4,
        default_state: 8647,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 8651,
        state_count: 16,
        default_state: 8651,
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
        first_state: 8667,
        state_count: 4,
        default_state: 8667,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 8671,
        state_count: 16,
        default_state: 8671,
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
        first_state: 8687,
        state_count: 4,
        default_state: 8687,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "piglin_head",
        first_state: 8691,
        state_count: 16,
        default_state: 8691,
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
        first_state: 8707,
        state_count: 4,
        default_state: 8707,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 8711,
        state_count: 4,
        default_state: 8711,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 8715,
        state_count: 4,
        default_state: 8715,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 8719,
        state_count: 4,
        default_state: 8719,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 8723,
        state_count: 24,
        default_state: 8724,
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
        first_state: 8747,
        state_count: 16,
        default_state: 8747,
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
        first_state: 8763,
        state_count: 16,
        default_state: 8763,
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
        first_state: 8779,
        state_count: 16,
        default_state: 8780,
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
        first_state: 8795,
        state_count: 32,
        default_state: 8811,
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
        first_state: 8827,
        state_count: 1,
        default_state: 8827,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 8828,
        state_count: 1,
        default_state: 8828,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 8829,
        state_count: 10,
        default_state: 8829,
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
        first_state: 8839,
        state_count: 1,
        default_state: 8839,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 8840,
        state_count: 1,
        default_state: 8840,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 8841,
        state_count: 3,
        default_state: 8842,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 8844,
        state_count: 80,
        default_state: 8855,
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
        first_state: 8924,
        state_count: 24,
        default_state: 8937,
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
        first_state: 8948,
        state_count: 12,
        default_state: 8949,
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
        first_state: 8960,
        state_count: 1,
        default_state: 8960,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 8961,
        state_count: 1,
        default_state: 8961,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 8962,
        state_count: 1,
        default_state: 8962,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 8963,
        state_count: 1,
        default_state: 8963,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 8964,
        state_count: 1,
        default_state: 8964,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 8965,
        state_count: 1,
        default_state: 8965,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 8966,
        state_count: 1,
        default_state: 8966,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 8967,
        state_count: 1,
        default_state: 8967,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 8968,
        state_count: 1,
        default_state: 8968,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 8969,
        state_count: 1,
        default_state: 8969,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 8970,
        state_count: 1,
        default_state: 8970,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 8971,
        state_count: 1,
        default_state: 8971,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 8972,
        state_count: 1,
        default_state: 8972,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 8973,
        state_count: 1,
        default_state: 8973,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 8974,
        state_count: 1,
        default_state: 8974,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 8975,
        state_count: 1,
        default_state: 8975,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 8976,
        state_count: 32,
        default_state: 9007,
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
        first_state: 9008,
        state_count: 32,
        default_state: 9039,
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
        first_state: 9040,
        state_count: 32,
        default_state: 9071,
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
        first_state: 9072,
        state_count: 32,
        default_state: 9103,
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
        first_state: 9104,
        state_count: 32,
        default_state: 9135,
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
        first_state: 9136,
        state_count: 32,
        default_state: 9167,
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
        first_state: 9168,
        state_count: 32,
        default_state: 9199,
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
        first_state: 9200,
        state_count: 32,
        default_state: 9231,
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
        first_state: 9232,
        state_count: 32,
        default_state: 9263,
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
        first_state: 9264,
        state_count: 32,
        default_state: 9295,
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
        first_state: 9296,
        state_count: 32,
        default_state: 9327,
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
        first_state: 9328,
        state_count: 32,
        default_state: 9359,
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
        first_state: 9360,
        state_count: 32,
        default_state: 9391,
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
        first_state: 9392,
        state_count: 32,
        default_state: 9423,
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
        first_state: 9424,
        state_count: 32,
        default_state: 9455,
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
        first_state: 9456,
        state_count: 32,
        default_state: 9487,
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
        first_state: 9488,
        state_count: 80,
        default_state: 9499,
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
        first_state: 9568,
        state_count: 80,
        default_state: 9579,
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
        first_state: 9648,
        state_count: 80,
        default_state: 9659,
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
        first_state: 9728,
        state_count: 80,
        default_state: 9739,
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
        first_state: 9808,
        state_count: 80,
        default_state: 9819,
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
        first_state: 9888,
        state_count: 1,
        default_state: 9888,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 9889,
        state_count: 1,
        default_state: 9889,
        props: &[],
    },
    BlockInfo {
        name: "light",
        first_state: 9890,
        state_count: 32,
        default_state: 9921,
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
        first_state: 9922,
        state_count: 64,
        default_state: 9937,
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
        first_state: 9986,
        state_count: 1,
        default_state: 9986,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 9987,
        state_count: 1,
        default_state: 9987,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 9988,
        state_count: 1,
        default_state: 9988,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 9989,
        state_count: 80,
        default_state: 10000,
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
        first_state: 10069,
        state_count: 80,
        default_state: 10080,
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
        first_state: 10149,
        state_count: 80,
        default_state: 10160,
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
        first_state: 10229,
        state_count: 6,
        default_state: 10232,
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
        first_state: 10235,
        state_count: 6,
        default_state: 10238,
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
        first_state: 10241,
        state_count: 6,
        default_state: 10244,
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
        first_state: 10247,
        state_count: 1,
        default_state: 10247,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 10248,
        state_count: 3,
        default_state: 10249,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 10251,
        state_count: 1,
        default_state: 10251,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 10252,
        state_count: 1,
        default_state: 10252,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 10253,
        state_count: 1,
        default_state: 10253,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 10254,
        state_count: 1,
        default_state: 10254,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 10255,
        state_count: 1,
        default_state: 10255,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 10256,
        state_count: 1,
        default_state: 10256,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 10257,
        state_count: 1,
        default_state: 10257,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 10258,
        state_count: 1,
        default_state: 10258,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 10259,
        state_count: 1,
        default_state: 10259,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 10260,
        state_count: 1,
        default_state: 10260,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 10261,
        state_count: 1,
        default_state: 10261,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 10262,
        state_count: 1,
        default_state: 10262,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 10263,
        state_count: 1,
        default_state: 10263,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 10264,
        state_count: 1,
        default_state: 10264,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 10265,
        state_count: 1,
        default_state: 10265,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 10266,
        state_count: 1,
        default_state: 10266,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 10267,
        state_count: 1,
        default_state: 10267,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 10268,
        state_count: 1,
        default_state: 10268,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 10269,
        state_count: 1,
        default_state: 10269,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 10270,
        state_count: 2,
        default_state: 10271,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 10272,
        state_count: 2,
        default_state: 10273,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 10274,
        state_count: 2,
        default_state: 10275,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 10276,
        state_count: 2,
        default_state: 10277,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 10278,
        state_count: 2,
        default_state: 10279,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 10280,
        state_count: 2,
        default_state: 10281,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 10282,
        state_count: 16,
        default_state: 10282,
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
        first_state: 10298,
        state_count: 16,
        default_state: 10298,
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
        first_state: 10314,
        state_count: 16,
        default_state: 10314,
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
        first_state: 10330,
        state_count: 16,
        default_state: 10330,
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
        first_state: 10346,
        state_count: 16,
        default_state: 10346,
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
        first_state: 10362,
        state_count: 16,
        default_state: 10362,
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
        first_state: 10378,
        state_count: 16,
        default_state: 10378,
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
        first_state: 10394,
        state_count: 16,
        default_state: 10394,
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
        first_state: 10410,
        state_count: 16,
        default_state: 10410,
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
        first_state: 10426,
        state_count: 16,
        default_state: 10426,
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
        first_state: 10442,
        state_count: 16,
        default_state: 10442,
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
        first_state: 10458,
        state_count: 16,
        default_state: 10458,
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
        first_state: 10474,
        state_count: 16,
        default_state: 10474,
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
        first_state: 10490,
        state_count: 16,
        default_state: 10490,
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
        first_state: 10506,
        state_count: 16,
        default_state: 10506,
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
        first_state: 10522,
        state_count: 16,
        default_state: 10522,
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
        first_state: 10538,
        state_count: 4,
        default_state: 10538,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 10542,
        state_count: 4,
        default_state: 10542,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 10546,
        state_count: 4,
        default_state: 10546,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 10550,
        state_count: 4,
        default_state: 10550,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 10554,
        state_count: 4,
        default_state: 10554,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 10558,
        state_count: 4,
        default_state: 10558,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 10562,
        state_count: 4,
        default_state: 10562,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 10566,
        state_count: 4,
        default_state: 10566,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 10570,
        state_count: 4,
        default_state: 10570,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 10574,
        state_count: 4,
        default_state: 10574,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 10578,
        state_count: 4,
        default_state: 10578,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 10582,
        state_count: 4,
        default_state: 10582,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 10586,
        state_count: 4,
        default_state: 10586,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 10590,
        state_count: 4,
        default_state: 10590,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 10594,
        state_count: 4,
        default_state: 10594,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 10598,
        state_count: 4,
        default_state: 10598,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 10602,
        state_count: 1,
        default_state: 10602,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 10603,
        state_count: 1,
        default_state: 10603,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 10604,
        state_count: 1,
        default_state: 10604,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 10605,
        state_count: 80,
        default_state: 10616,
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
        first_state: 10685,
        state_count: 6,
        default_state: 10688,
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
        first_state: 10691,
        state_count: 6,
        default_state: 10694,
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
        first_state: 10697,
        state_count: 6,
        default_state: 10700,
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
        first_state: 10703,
        state_count: 6,
        default_state: 10706,
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
        first_state: 10709,
        state_count: 6,
        default_state: 10712,
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
        first_state: 10715,
        state_count: 6,
        default_state: 10718,
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
        first_state: 10721,
        state_count: 6,
        default_state: 10724,
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
        first_state: 10727,
        state_count: 6,
        default_state: 10730,
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
        first_state: 10733,
        state_count: 6,
        default_state: 10736,
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
        first_state: 10739,
        state_count: 6,
        default_state: 10742,
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
        first_state: 10745,
        state_count: 6,
        default_state: 10748,
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
        first_state: 10751,
        state_count: 6,
        default_state: 10754,
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
        first_state: 10757,
        state_count: 6,
        default_state: 10760,
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
        first_state: 10763,
        state_count: 6,
        default_state: 10766,
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
        first_state: 10769,
        state_count: 6,
        default_state: 10772,
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
        first_state: 10775,
        state_count: 6,
        default_state: 10778,
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
        first_state: 10781,
        state_count: 6,
        default_state: 10784,
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
        first_state: 10787,
        state_count: 6,
        default_state: 10790,
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
        first_state: 10793,
        state_count: 6,
        default_state: 10796,
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
        first_state: 10799,
        state_count: 6,
        default_state: 10802,
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
        first_state: 10805,
        state_count: 6,
        default_state: 10808,
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
        first_state: 10811,
        state_count: 6,
        default_state: 10814,
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
        first_state: 10817,
        state_count: 6,
        default_state: 10820,
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
        first_state: 10823,
        state_count: 1,
        default_state: 10823,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 10824,
        state_count: 1,
        default_state: 10824,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 10825,
        state_count: 1,
        default_state: 10825,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 10826,
        state_count: 1,
        default_state: 10826,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 10827,
        state_count: 32,
        default_state: 10834,
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
        first_state: 10859,
        state_count: 32,
        default_state: 10866,
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
        first_state: 10891,
        state_count: 32,
        default_state: 10898,
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
        first_state: 10923,
        state_count: 32,
        default_state: 10930,
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
        first_state: 10955,
        state_count: 32,
        default_state: 10962,
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
        first_state: 10987,
        state_count: 32,
        default_state: 10994,
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
        first_state: 11019,
        state_count: 32,
        default_state: 11026,
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
        first_state: 11051,
        state_count: 32,
        default_state: 11082,
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
        first_state: 11083,
        state_count: 32,
        default_state: 11114,
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
        first_state: 11115,
        state_count: 32,
        default_state: 11146,
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
        first_state: 11147,
        state_count: 32,
        default_state: 11178,
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
        first_state: 11179,
        state_count: 32,
        default_state: 11210,
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
        first_state: 11211,
        state_count: 32,
        default_state: 11242,
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
        first_state: 11243,
        state_count: 32,
        default_state: 11274,
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
        first_state: 11275,
        state_count: 64,
        default_state: 11286,
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
        first_state: 11339,
        state_count: 64,
        default_state: 11350,
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
        first_state: 11403,
        state_count: 64,
        default_state: 11414,
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
        first_state: 11467,
        state_count: 64,
        default_state: 11478,
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
        first_state: 11531,
        state_count: 64,
        default_state: 11542,
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
        first_state: 11595,
        state_count: 64,
        default_state: 11606,
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
        first_state: 11659,
        state_count: 64,
        default_state: 11670,
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
        first_state: 11723,
        state_count: 6,
        default_state: 11727,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 11729,
        state_count: 64,
        default_state: 11792,
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
        first_state: 11793,
        state_count: 6,
        default_state: 11793,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 11799,
        state_count: 1,
        default_state: 11799,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 11800,
        state_count: 3,
        default_state: 11801,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 11803,
        state_count: 80,
        default_state: 11814,
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
        first_state: 11883,
        state_count: 1,
        default_state: 11883,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 11884,
        state_count: 4,
        default_state: 11884,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "dirt_path",
        first_state: 11888,
        state_count: 1,
        default_state: 11888,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 11889,
        state_count: 1,
        default_state: 11889,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 11890,
        state_count: 12,
        default_state: 11896,
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
        first_state: 11902,
        state_count: 12,
        default_state: 11908,
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
        first_state: 11914,
        state_count: 4,
        default_state: 11914,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 11918,
        state_count: 1,
        default_state: 11918,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 11919,
        state_count: 1,
        default_state: 11919,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 11920,
        state_count: 1,
        default_state: 11920,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 11921,
        state_count: 3,
        default_state: 11922,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 11924,
        state_count: 1,
        default_state: 11924,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 11925,
        state_count: 12,
        default_state: 11930,
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
        first_state: 11937,
        state_count: 6,
        default_state: 11941,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 11943,
        state_count: 6,
        default_state: 11947,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 11949,
        state_count: 6,
        default_state: 11953,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 11955,
        state_count: 6,
        default_state: 11959,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 11961,
        state_count: 6,
        default_state: 11965,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 11967,
        state_count: 6,
        default_state: 11971,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 11973,
        state_count: 6,
        default_state: 11977,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 11979,
        state_count: 6,
        default_state: 11983,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 11985,
        state_count: 6,
        default_state: 11989,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 11991,
        state_count: 6,
        default_state: 11995,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 11997,
        state_count: 6,
        default_state: 12001,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 12003,
        state_count: 6,
        default_state: 12007,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 12009,
        state_count: 6,
        default_state: 12013,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 12015,
        state_count: 6,
        default_state: 12019,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 12021,
        state_count: 6,
        default_state: 12025,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 12027,
        state_count: 6,
        default_state: 12031,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 12033,
        state_count: 6,
        default_state: 12037,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 12039,
        state_count: 4,
        default_state: 12039,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 12043,
        state_count: 4,
        default_state: 12043,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 12047,
        state_count: 4,
        default_state: 12047,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 12051,
        state_count: 4,
        default_state: 12051,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 12055,
        state_count: 4,
        default_state: 12055,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 12059,
        state_count: 4,
        default_state: 12059,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 12063,
        state_count: 4,
        default_state: 12063,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 12067,
        state_count: 4,
        default_state: 12067,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 12071,
        state_count: 4,
        default_state: 12071,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 12075,
        state_count: 4,
        default_state: 12075,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 12079,
        state_count: 4,
        default_state: 12079,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 12083,
        state_count: 4,
        default_state: 12083,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 12087,
        state_count: 4,
        default_state: 12087,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 12091,
        state_count: 4,
        default_state: 12091,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 12095,
        state_count: 4,
        default_state: 12095,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 12099,
        state_count: 4,
        default_state: 12099,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 12103,
        state_count: 1,
        default_state: 12103,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 12104,
        state_count: 1,
        default_state: 12104,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 12105,
        state_count: 1,
        default_state: 12105,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 12106,
        state_count: 1,
        default_state: 12106,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 12107,
        state_count: 1,
        default_state: 12107,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 12108,
        state_count: 1,
        default_state: 12108,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 12109,
        state_count: 1,
        default_state: 12109,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 12110,
        state_count: 1,
        default_state: 12110,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 12111,
        state_count: 1,
        default_state: 12111,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 12112,
        state_count: 1,
        default_state: 12112,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 12113,
        state_count: 1,
        default_state: 12113,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 12114,
        state_count: 1,
        default_state: 12114,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 12115,
        state_count: 1,
        default_state: 12115,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 12116,
        state_count: 1,
        default_state: 12116,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 12117,
        state_count: 1,
        default_state: 12117,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 12118,
        state_count: 1,
        default_state: 12118,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 12119,
        state_count: 1,
        default_state: 12119,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 12120,
        state_count: 1,
        default_state: 12120,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 12121,
        state_count: 1,
        default_state: 12121,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 12122,
        state_count: 1,
        default_state: 12122,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 12123,
        state_count: 1,
        default_state: 12123,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 12124,
        state_count: 1,
        default_state: 12124,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 12125,
        state_count: 1,
        default_state: 12125,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 12126,
        state_count: 1,
        default_state: 12126,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 12127,
        state_count: 1,
        default_state: 12127,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 12128,
        state_count: 1,
        default_state: 12128,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 12129,
        state_count: 1,
        default_state: 12129,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 12130,
        state_count: 1,
        default_state: 12130,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 12131,
        state_count: 1,
        default_state: 12131,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 12132,
        state_count: 1,
        default_state: 12132,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 12133,
        state_count: 1,
        default_state: 12133,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 12134,
        state_count: 1,
        default_state: 12134,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 12135,
        state_count: 26,
        default_state: 12135,
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
        first_state: 12161,
        state_count: 1,
        default_state: 12161,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 12162,
        state_count: 1,
        default_state: 12162,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 12163,
        state_count: 12,
        default_state: 12163,
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
        first_state: 12175,
        state_count: 1,
        default_state: 12175,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 12176,
        state_count: 1,
        default_state: 12176,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 12177,
        state_count: 1,
        default_state: 12177,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 12178,
        state_count: 1,
        default_state: 12178,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 12179,
        state_count: 1,
        default_state: 12179,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 12180,
        state_count: 1,
        default_state: 12180,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 12181,
        state_count: 1,
        default_state: 12181,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 12182,
        state_count: 1,
        default_state: 12182,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 12183,
        state_count: 1,
        default_state: 12183,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 12184,
        state_count: 1,
        default_state: 12184,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 12185,
        state_count: 2,
        default_state: 12185,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 12187,
        state_count: 2,
        default_state: 12187,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 12189,
        state_count: 2,
        default_state: 12189,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 12191,
        state_count: 2,
        default_state: 12191,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 12193,
        state_count: 2,
        default_state: 12193,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 12195,
        state_count: 2,
        default_state: 12195,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 12197,
        state_count: 2,
        default_state: 12197,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 12199,
        state_count: 2,
        default_state: 12199,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 12201,
        state_count: 2,
        default_state: 12201,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 12203,
        state_count: 2,
        default_state: 12203,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 12205,
        state_count: 2,
        default_state: 12205,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 12207,
        state_count: 2,
        default_state: 12207,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 12209,
        state_count: 2,
        default_state: 12209,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 12211,
        state_count: 2,
        default_state: 12211,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 12213,
        state_count: 2,
        default_state: 12213,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 12215,
        state_count: 2,
        default_state: 12215,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 12217,
        state_count: 2,
        default_state: 12217,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 12219,
        state_count: 2,
        default_state: 12219,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 12221,
        state_count: 2,
        default_state: 12221,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 12223,
        state_count: 2,
        default_state: 12223,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 12225,
        state_count: 8,
        default_state: 12225,
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
        first_state: 12233,
        state_count: 8,
        default_state: 12233,
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
        first_state: 12241,
        state_count: 8,
        default_state: 12241,
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
        first_state: 12249,
        state_count: 8,
        default_state: 12249,
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
        first_state: 12257,
        state_count: 8,
        default_state: 12257,
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
        first_state: 12265,
        state_count: 8,
        default_state: 12265,
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
        first_state: 12273,
        state_count: 8,
        default_state: 12273,
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
        first_state: 12281,
        state_count: 8,
        default_state: 12281,
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
        first_state: 12289,
        state_count: 8,
        default_state: 12289,
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
        first_state: 12297,
        state_count: 8,
        default_state: 12297,
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
        first_state: 12305,
        state_count: 8,
        default_state: 12305,
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
        first_state: 12313,
        state_count: 1,
        default_state: 12313,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 12314,
        state_count: 2,
        default_state: 12314,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 12316,
        state_count: 1,
        default_state: 12316,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 12317,
        state_count: 12,
        default_state: 12317,
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
        first_state: 12329,
        state_count: 1,
        default_state: 12329,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 12330,
        state_count: 1,
        default_state: 12330,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 12331,
        state_count: 1,
        default_state: 12331,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 12332,
        state_count: 2,
        default_state: 12332,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 12334,
        state_count: 80,
        default_state: 12345,
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
        first_state: 12414,
        state_count: 80,
        default_state: 12425,
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
        first_state: 12494,
        state_count: 80,
        default_state: 12505,
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
        first_state: 12574,
        state_count: 80,
        default_state: 12585,
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
        first_state: 12654,
        state_count: 80,
        default_state: 12665,
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
        first_state: 12734,
        state_count: 80,
        default_state: 12745,
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
        first_state: 12814,
        state_count: 80,
        default_state: 12825,
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
        first_state: 12894,
        state_count: 80,
        default_state: 12905,
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
        first_state: 12974,
        state_count: 80,
        default_state: 12985,
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
        first_state: 13054,
        state_count: 80,
        default_state: 13065,
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
        first_state: 13134,
        state_count: 80,
        default_state: 13145,
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
        first_state: 13214,
        state_count: 80,
        default_state: 13225,
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
        first_state: 13294,
        state_count: 80,
        default_state: 13305,
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
        first_state: 13374,
        state_count: 80,
        default_state: 13385,
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
        first_state: 13454,
        state_count: 6,
        default_state: 13457,
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
        first_state: 13460,
        state_count: 6,
        default_state: 13463,
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
        first_state: 13466,
        state_count: 6,
        default_state: 13469,
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
        first_state: 13472,
        state_count: 6,
        default_state: 13475,
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
        first_state: 13478,
        state_count: 6,
        default_state: 13481,
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
        first_state: 13484,
        state_count: 6,
        default_state: 13487,
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
        first_state: 13490,
        state_count: 6,
        default_state: 13493,
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
        first_state: 13496,
        state_count: 6,
        default_state: 13499,
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
        first_state: 13502,
        state_count: 6,
        default_state: 13505,
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
        first_state: 13508,
        state_count: 6,
        default_state: 13511,
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
        first_state: 13514,
        state_count: 6,
        default_state: 13517,
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
        first_state: 13520,
        state_count: 6,
        default_state: 13523,
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
        first_state: 13526,
        state_count: 6,
        default_state: 13529,
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
        first_state: 13532,
        state_count: 324,
        default_state: 13535,
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
        first_state: 13856,
        state_count: 324,
        default_state: 13859,
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
        first_state: 14180,
        state_count: 324,
        default_state: 14183,
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
        first_state: 14504,
        state_count: 324,
        default_state: 14507,
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
        first_state: 14828,
        state_count: 324,
        default_state: 14831,
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
        first_state: 15152,
        state_count: 324,
        default_state: 15155,
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
        first_state: 15476,
        state_count: 324,
        default_state: 15479,
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
        first_state: 15800,
        state_count: 324,
        default_state: 15803,
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
        first_state: 16124,
        state_count: 324,
        default_state: 16127,
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
        first_state: 16448,
        state_count: 324,
        default_state: 16451,
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
        first_state: 16772,
        state_count: 324,
        default_state: 16775,
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
        first_state: 17096,
        state_count: 324,
        default_state: 17099,
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
        first_state: 17420,
        state_count: 324,
        default_state: 17423,
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
        first_state: 17744,
        state_count: 32,
        default_state: 17775,
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
        first_state: 17776,
        state_count: 4,
        default_state: 17776,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 17780,
        state_count: 12,
        default_state: 17781,
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
        first_state: 17792,
        state_count: 8,
        default_state: 17793,
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
        first_state: 17800,
        state_count: 8,
        default_state: 17801,
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
        first_state: 17808,
        state_count: 1,
        default_state: 17808,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 17809,
        state_count: 1,
        default_state: 17809,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 17810,
        state_count: 12,
        default_state: 17814,
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
        first_state: 17822,
        state_count: 16,
        default_state: 17825,
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
        first_state: 17838,
        state_count: 1,
        default_state: 17838,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 17839,
        state_count: 4,
        default_state: 17839,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 17843,
        state_count: 32,
        default_state: 17844,
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
        first_state: 17875,
        state_count: 4,
        default_state: 17878,
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
        first_state: 17879,
        state_count: 4,
        default_state: 17882,
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
        first_state: 17883,
        state_count: 32,
        default_state: 17886,
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
        first_state: 17915,
        state_count: 32,
        default_state: 17918,
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
        first_state: 17947,
        state_count: 4,
        default_state: 17947,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 17951,
        state_count: 3,
        default_state: 17952,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 17954,
        state_count: 3,
        default_state: 17955,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 17957,
        state_count: 3,
        default_state: 17958,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 17960,
        state_count: 3,
        default_state: 17961,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 17963,
        state_count: 1,
        default_state: 17963,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 17964,
        state_count: 1,
        default_state: 17964,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 17965,
        state_count: 1,
        default_state: 17965,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 17966,
        state_count: 1,
        default_state: 17966,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 17967,
        state_count: 1,
        default_state: 17967,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 17968,
        state_count: 3,
        default_state: 17969,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 17971,
        state_count: 3,
        default_state: 17972,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 17974,
        state_count: 3,
        default_state: 17975,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 17977,
        state_count: 3,
        default_state: 17978,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 17980,
        state_count: 1,
        default_state: 17980,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 17981,
        state_count: 1,
        default_state: 17981,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 17982,
        state_count: 1,
        default_state: 17982,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 17983,
        state_count: 26,
        default_state: 17983,
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
        first_state: 18009,
        state_count: 1,
        default_state: 18009,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 18010,
        state_count: 26,
        default_state: 18010,
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
        first_state: 18036,
        state_count: 1,
        default_state: 18036,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 18037,
        state_count: 1,
        default_state: 18037,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 18038,
        state_count: 1,
        default_state: 18038,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 18039,
        state_count: 1,
        default_state: 18039,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 18040,
        state_count: 6,
        default_state: 18043,
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
        first_state: 18046,
        state_count: 6,
        default_state: 18049,
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
        first_state: 18052,
        state_count: 2,
        default_state: 18053,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 18054,
        state_count: 2,
        default_state: 18055,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 18056,
        state_count: 32,
        default_state: 18087,
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
        first_state: 18088,
        state_count: 32,
        default_state: 18119,
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
        first_state: 18120,
        state_count: 64,
        default_state: 18135,
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
        first_state: 18184,
        state_count: 64,
        default_state: 18199,
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
        first_state: 18248,
        state_count: 32,
        default_state: 18255,
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
        first_state: 18280,
        state_count: 32,
        default_state: 18287,
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
        first_state: 18312,
        state_count: 80,
        default_state: 18323,
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
        first_state: 18392,
        state_count: 80,
        default_state: 18403,
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
        first_state: 18472,
        state_count: 24,
        default_state: 18481,
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
        first_state: 18496,
        state_count: 24,
        default_state: 18505,
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
        first_state: 18520,
        state_count: 64,
        default_state: 18531,
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
        first_state: 18584,
        state_count: 64,
        default_state: 18595,
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
        first_state: 18648,
        state_count: 32,
        default_state: 18649,
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
        first_state: 18680,
        state_count: 32,
        default_state: 18681,
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
        first_state: 18712,
        state_count: 8,
        default_state: 18713,
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
        first_state: 18720,
        state_count: 8,
        default_state: 18721,
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
        first_state: 18728,
        state_count: 4,
        default_state: 18729,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 18732,
        state_count: 12,
        default_state: 18742,
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
        first_state: 18744,
        state_count: 9,
        default_state: 18744,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 18753,
        state_count: 16,
        default_state: 18753,
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
        first_state: 18769,
        state_count: 24,
        default_state: 18769,
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
        first_state: 18793,
        state_count: 24,
        default_state: 18793,
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
        first_state: 18817,
        state_count: 1,
        default_state: 18817,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 18818,
        state_count: 1,
        default_state: 18818,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 18819,
        state_count: 1,
        default_state: 18819,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 18820,
        state_count: 1,
        default_state: 18820,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 18821,
        state_count: 1,
        default_state: 18821,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 18822,
        state_count: 5,
        default_state: 18822,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 18827,
        state_count: 1,
        default_state: 18827,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 18828,
        state_count: 1,
        default_state: 18828,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 18829,
        state_count: 1,
        default_state: 18829,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 18830,
        state_count: 1,
        default_state: 18830,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 18831,
        state_count: 1,
        default_state: 18831,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 18832,
        state_count: 1,
        default_state: 18832,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 18833,
        state_count: 80,
        default_state: 18844,
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
        first_state: 18913,
        state_count: 324,
        default_state: 18916,
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
        first_state: 19237,
        state_count: 6,
        default_state: 19240,
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
        first_state: 19243,
        state_count: 1,
        default_state: 19243,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 19244,
        state_count: 1,
        default_state: 19244,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 19245,
        state_count: 1,
        default_state: 19245,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 19246,
        state_count: 1,
        default_state: 19246,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 19247,
        state_count: 6,
        default_state: 19250,
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
        first_state: 19253,
        state_count: 80,
        default_state: 19264,
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
        first_state: 19333,
        state_count: 324,
        default_state: 19336,
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
        first_state: 19657,
        state_count: 1,
        default_state: 19657,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 19658,
        state_count: 80,
        default_state: 19669,
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
        first_state: 19738,
        state_count: 6,
        default_state: 19741,
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
        first_state: 19744,
        state_count: 2,
        default_state: 19745,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 19746,
        state_count: 24,
        default_state: 19755,
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
        first_state: 19770,
        state_count: 324,
        default_state: 19773,
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
        first_state: 20094,
        state_count: 1,
        default_state: 20094,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 20095,
        state_count: 1,
        default_state: 20095,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 20096,
        state_count: 1,
        default_state: 20096,
        props: &[],
    },
    BlockInfo {
        name: "candle",
        first_state: 20097,
        state_count: 16,
        default_state: 20100,
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
        first_state: 20113,
        state_count: 16,
        default_state: 20116,
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
        first_state: 20129,
        state_count: 16,
        default_state: 20132,
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
        first_state: 20145,
        state_count: 16,
        default_state: 20148,
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
        first_state: 20161,
        state_count: 16,
        default_state: 20164,
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
        first_state: 20177,
        state_count: 16,
        default_state: 20180,
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
        first_state: 20193,
        state_count: 16,
        default_state: 20196,
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
        first_state: 20209,
        state_count: 16,
        default_state: 20212,
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
        first_state: 20225,
        state_count: 16,
        default_state: 20228,
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
        first_state: 20241,
        state_count: 16,
        default_state: 20244,
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
        first_state: 20257,
        state_count: 16,
        default_state: 20260,
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
        first_state: 20273,
        state_count: 16,
        default_state: 20276,
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
        first_state: 20289,
        state_count: 16,
        default_state: 20292,
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
        first_state: 20305,
        state_count: 16,
        default_state: 20308,
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
        first_state: 20321,
        state_count: 16,
        default_state: 20324,
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
        first_state: 20337,
        state_count: 16,
        default_state: 20340,
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
        first_state: 20353,
        state_count: 16,
        default_state: 20356,
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
        first_state: 20369,
        state_count: 2,
        default_state: 20370,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "white_candle_cake",
        first_state: 20371,
        state_count: 2,
        default_state: 20372,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "orange_candle_cake",
        first_state: 20373,
        state_count: 2,
        default_state: 20374,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "magenta_candle_cake",
        first_state: 20375,
        state_count: 2,
        default_state: 20376,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_blue_candle_cake",
        first_state: 20377,
        state_count: 2,
        default_state: 20378,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "yellow_candle_cake",
        first_state: 20379,
        state_count: 2,
        default_state: 20380,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lime_candle_cake",
        first_state: 20381,
        state_count: 2,
        default_state: 20382,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pink_candle_cake",
        first_state: 20383,
        state_count: 2,
        default_state: 20384,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "gray_candle_cake",
        first_state: 20385,
        state_count: 2,
        default_state: 20386,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_gray_candle_cake",
        first_state: 20387,
        state_count: 2,
        default_state: 20388,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cyan_candle_cake",
        first_state: 20389,
        state_count: 2,
        default_state: 20390,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "purple_candle_cake",
        first_state: 20391,
        state_count: 2,
        default_state: 20392,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "blue_candle_cake",
        first_state: 20393,
        state_count: 2,
        default_state: 20394,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brown_candle_cake",
        first_state: 20395,
        state_count: 2,
        default_state: 20396,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "green_candle_cake",
        first_state: 20397,
        state_count: 2,
        default_state: 20398,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "red_candle_cake",
        first_state: 20399,
        state_count: 2,
        default_state: 20400,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "black_candle_cake",
        first_state: 20401,
        state_count: 2,
        default_state: 20402,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "amethyst_block",
        first_state: 20403,
        state_count: 1,
        default_state: 20403,
        props: &[],
    },
    BlockInfo {
        name: "budding_amethyst",
        first_state: 20404,
        state_count: 1,
        default_state: 20404,
        props: &[],
    },
    BlockInfo {
        name: "amethyst_cluster",
        first_state: 20405,
        state_count: 12,
        default_state: 20414,
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
        first_state: 20417,
        state_count: 12,
        default_state: 20426,
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
        first_state: 20429,
        state_count: 12,
        default_state: 20438,
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
        first_state: 20441,
        state_count: 12,
        default_state: 20450,
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
        first_state: 20453,
        state_count: 1,
        default_state: 20453,
        props: &[],
    },
    BlockInfo {
        name: "calcite",
        first_state: 20454,
        state_count: 1,
        default_state: 20454,
        props: &[],
    },
    BlockInfo {
        name: "tinted_glass",
        first_state: 20455,
        state_count: 1,
        default_state: 20455,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow",
        first_state: 20456,
        state_count: 1,
        default_state: 20456,
        props: &[],
    },
    BlockInfo {
        name: "sculk_sensor",
        first_state: 20457,
        state_count: 96,
        default_state: 20458,
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
        first_state: 20553,
        state_count: 1,
        default_state: 20553,
        props: &[],
    },
    BlockInfo {
        name: "sculk_vein",
        first_state: 20554,
        state_count: 128,
        default_state: 20681,
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
        first_state: 20682,
        state_count: 2,
        default_state: 20683,
        props: &[PropInfo {
            name: "bloom",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sculk_shrieker",
        first_state: 20684,
        state_count: 8,
        default_state: 20691,
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
        first_state: 20692,
        state_count: 1,
        default_state: 20692,
        props: &[],
    },
    BlockInfo {
        name: "weathered_copper",
        first_state: 20693,
        state_count: 1,
        default_state: 20693,
        props: &[],
    },
    BlockInfo {
        name: "exposed_copper",
        first_state: 20694,
        state_count: 1,
        default_state: 20694,
        props: &[],
    },
    BlockInfo {
        name: "copper_block",
        first_state: 20695,
        state_count: 1,
        default_state: 20695,
        props: &[],
    },
    BlockInfo {
        name: "copper_ore",
        first_state: 20696,
        state_count: 1,
        default_state: 20696,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_copper_ore",
        first_state: 20697,
        state_count: 1,
        default_state: 20697,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper",
        first_state: 20698,
        state_count: 1,
        default_state: 20698,
        props: &[],
    },
    BlockInfo {
        name: "weathered_cut_copper",
        first_state: 20699,
        state_count: 1,
        default_state: 20699,
        props: &[],
    },
    BlockInfo {
        name: "exposed_cut_copper",
        first_state: 20700,
        state_count: 1,
        default_state: 20700,
        props: &[],
    },
    BlockInfo {
        name: "cut_copper",
        first_state: 20701,
        state_count: 1,
        default_state: 20701,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper_stairs",
        first_state: 20702,
        state_count: 80,
        default_state: 20713,
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
        first_state: 20782,
        state_count: 80,
        default_state: 20793,
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
        first_state: 20862,
        state_count: 80,
        default_state: 20873,
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
        first_state: 20942,
        state_count: 80,
        default_state: 20953,
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
        first_state: 21022,
        state_count: 6,
        default_state: 21025,
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
        first_state: 21028,
        state_count: 6,
        default_state: 21031,
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
        first_state: 21034,
        state_count: 6,
        default_state: 21037,
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
        first_state: 21040,
        state_count: 6,
        default_state: 21043,
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
        first_state: 21046,
        state_count: 1,
        default_state: 21046,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_copper",
        first_state: 21047,
        state_count: 1,
        default_state: 21047,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_copper",
        first_state: 21048,
        state_count: 1,
        default_state: 21048,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_copper",
        first_state: 21049,
        state_count: 1,
        default_state: 21049,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper",
        first_state: 21050,
        state_count: 1,
        default_state: 21050,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper",
        first_state: 21051,
        state_count: 1,
        default_state: 21051,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper",
        first_state: 21052,
        state_count: 1,
        default_state: 21052,
        props: &[],
    },
    BlockInfo {
        name: "waxed_cut_copper",
        first_state: 21053,
        state_count: 1,
        default_state: 21053,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_stairs",
        first_state: 21054,
        state_count: 80,
        default_state: 21065,
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
        first_state: 21134,
        state_count: 80,
        default_state: 21145,
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
        first_state: 21214,
        state_count: 80,
        default_state: 21225,
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
        first_state: 21294,
        state_count: 80,
        default_state: 21305,
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
        first_state: 21374,
        state_count: 6,
        default_state: 21377,
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
        first_state: 21380,
        state_count: 6,
        default_state: 21383,
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
        first_state: 21386,
        state_count: 6,
        default_state: 21389,
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
        first_state: 21392,
        state_count: 6,
        default_state: 21395,
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
        first_state: 21398,
        state_count: 24,
        default_state: 21417,
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
        first_state: 21422,
        state_count: 20,
        default_state: 21427,
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
        first_state: 21442,
        state_count: 1,
        default_state: 21442,
        props: &[],
    },
    BlockInfo {
        name: "cave_vines",
        first_state: 21443,
        state_count: 52,
        default_state: 21444,
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
        first_state: 21495,
        state_count: 2,
        default_state: 21496,
        props: &[PropInfo {
            name: "berries",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spore_blossom",
        first_state: 21497,
        state_count: 1,
        default_state: 21497,
        props: &[],
    },
    BlockInfo {
        name: "azalea",
        first_state: 21498,
        state_count: 1,
        default_state: 21498,
        props: &[],
    },
    BlockInfo {
        name: "flowering_azalea",
        first_state: 21499,
        state_count: 1,
        default_state: 21499,
        props: &[],
    },
    BlockInfo {
        name: "moss_carpet",
        first_state: 21500,
        state_count: 1,
        default_state: 21500,
        props: &[],
    },
    BlockInfo {
        name: "moss_block",
        first_state: 21501,
        state_count: 1,
        default_state: 21501,
        props: &[],
    },
    BlockInfo {
        name: "big_dripleaf",
        first_state: 21502,
        state_count: 32,
        default_state: 21503,
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
        first_state: 21534,
        state_count: 8,
        default_state: 21535,
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
        first_state: 21542,
        state_count: 16,
        default_state: 21545,
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
        first_state: 21558,
        state_count: 2,
        default_state: 21559,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "rooted_dirt",
        first_state: 21560,
        state_count: 1,
        default_state: 21560,
        props: &[],
    },
    BlockInfo {
        name: "mud",
        first_state: 21561,
        state_count: 1,
        default_state: 21561,
        props: &[],
    },
    BlockInfo {
        name: "deepslate",
        first_state: 21562,
        state_count: 3,
        default_state: 21563,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cobbled_deepslate",
        first_state: 21565,
        state_count: 1,
        default_state: 21565,
        props: &[],
    },
    BlockInfo {
        name: "cobbled_deepslate_stairs",
        first_state: 21566,
        state_count: 80,
        default_state: 21577,
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
        first_state: 21646,
        state_count: 6,
        default_state: 21649,
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
        first_state: 21652,
        state_count: 324,
        default_state: 21655,
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
        first_state: 21976,
        state_count: 1,
        default_state: 21976,
        props: &[],
    },
    BlockInfo {
        name: "polished_deepslate_stairs",
        first_state: 21977,
        state_count: 80,
        default_state: 21988,
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
        first_state: 22057,
        state_count: 6,
        default_state: 22060,
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
        first_state: 22063,
        state_count: 324,
        default_state: 22066,
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
        first_state: 22387,
        state_count: 1,
        default_state: 22387,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_tile_stairs",
        first_state: 22388,
        state_count: 80,
        default_state: 22399,
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
        first_state: 22468,
        state_count: 6,
        default_state: 22471,
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
        first_state: 22474,
        state_count: 324,
        default_state: 22477,
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
        first_state: 22798,
        state_count: 1,
        default_state: 22798,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_brick_stairs",
        first_state: 22799,
        state_count: 80,
        default_state: 22810,
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
        first_state: 22879,
        state_count: 6,
        default_state: 22882,
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
        first_state: 22885,
        state_count: 324,
        default_state: 22888,
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
        first_state: 23209,
        state_count: 1,
        default_state: 23209,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_bricks",
        first_state: 23210,
        state_count: 1,
        default_state: 23210,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_tiles",
        first_state: 23211,
        state_count: 1,
        default_state: 23211,
        props: &[],
    },
    BlockInfo {
        name: "infested_deepslate",
        first_state: 23212,
        state_count: 3,
        default_state: 23213,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "smooth_basalt",
        first_state: 23215,
        state_count: 1,
        default_state: 23215,
        props: &[],
    },
    BlockInfo {
        name: "raw_iron_block",
        first_state: 23216,
        state_count: 1,
        default_state: 23216,
        props: &[],
    },
    BlockInfo {
        name: "raw_copper_block",
        first_state: 23217,
        state_count: 1,
        default_state: 23217,
        props: &[],
    },
    BlockInfo {
        name: "raw_gold_block",
        first_state: 23218,
        state_count: 1,
        default_state: 23218,
        props: &[],
    },
    BlockInfo {
        name: "potted_azalea_bush",
        first_state: 23219,
        state_count: 1,
        default_state: 23219,
        props: &[],
    },
    BlockInfo {
        name: "potted_flowering_azalea_bush",
        first_state: 23220,
        state_count: 1,
        default_state: 23220,
        props: &[],
    },
    BlockInfo {
        name: "ochre_froglight",
        first_state: 23221,
        state_count: 3,
        default_state: 23222,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "verdant_froglight",
        first_state: 23224,
        state_count: 3,
        default_state: 23225,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pearlescent_froglight",
        first_state: 23227,
        state_count: 3,
        default_state: 23228,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "frogspawn",
        first_state: 23230,
        state_count: 1,
        default_state: 23230,
        props: &[],
    },
    BlockInfo {
        name: "reinforced_deepslate",
        first_state: 23231,
        state_count: 1,
        default_state: 23231,
        props: &[],
    },
];
