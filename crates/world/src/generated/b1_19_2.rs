// @generated `cargo xtask gen-blockstates 1.19.2` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.19.2";
pub const STATE_COUNT: u32 = 21448;

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
        name: "oak_sapling",
        first_state: 22,
        state_count: 2,
        default_state: 22,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "spruce_sapling",
        first_state: 24,
        state_count: 2,
        default_state: 24,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "birch_sapling",
        first_state: 26,
        state_count: 2,
        default_state: 26,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "jungle_sapling",
        first_state: 28,
        state_count: 2,
        default_state: 28,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "acacia_sapling",
        first_state: 30,
        state_count: 2,
        default_state: 30,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "dark_oak_sapling",
        first_state: 32,
        state_count: 2,
        default_state: 32,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "mangrove_propagule",
        first_state: 34,
        state_count: 40,
        default_state: 39,
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
        first_state: 74,
        state_count: 1,
        default_state: 74,
        props: &[],
    },
    BlockInfo {
        name: "water",
        first_state: 75,
        state_count: 16,
        default_state: 75,
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
        first_state: 91,
        state_count: 16,
        default_state: 91,
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
        first_state: 107,
        state_count: 1,
        default_state: 107,
        props: &[],
    },
    BlockInfo {
        name: "red_sand",
        first_state: 108,
        state_count: 1,
        default_state: 108,
        props: &[],
    },
    BlockInfo {
        name: "gravel",
        first_state: 109,
        state_count: 1,
        default_state: 109,
        props: &[],
    },
    BlockInfo {
        name: "gold_ore",
        first_state: 110,
        state_count: 1,
        default_state: 110,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_gold_ore",
        first_state: 111,
        state_count: 1,
        default_state: 111,
        props: &[],
    },
    BlockInfo {
        name: "iron_ore",
        first_state: 112,
        state_count: 1,
        default_state: 112,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_iron_ore",
        first_state: 113,
        state_count: 1,
        default_state: 113,
        props: &[],
    },
    BlockInfo {
        name: "coal_ore",
        first_state: 114,
        state_count: 1,
        default_state: 114,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_coal_ore",
        first_state: 115,
        state_count: 1,
        default_state: 115,
        props: &[],
    },
    BlockInfo {
        name: "nether_gold_ore",
        first_state: 116,
        state_count: 1,
        default_state: 116,
        props: &[],
    },
    BlockInfo {
        name: "oak_log",
        first_state: 117,
        state_count: 3,
        default_state: 118,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 120,
        state_count: 3,
        default_state: 121,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 123,
        state_count: 3,
        default_state: 124,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 126,
        state_count: 3,
        default_state: 127,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 129,
        state_count: 3,
        default_state: 130,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 132,
        state_count: 3,
        default_state: 133,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_log",
        first_state: 135,
        state_count: 3,
        default_state: 136,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_roots",
        first_state: 138,
        state_count: 2,
        default_state: 139,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "muddy_mangrove_roots",
        first_state: 140,
        state_count: 3,
        default_state: 141,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 143,
        state_count: 3,
        default_state: 144,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 146,
        state_count: 3,
        default_state: 147,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 149,
        state_count: 3,
        default_state: 150,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 152,
        state_count: 3,
        default_state: 153,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 155,
        state_count: 3,
        default_state: 156,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 158,
        state_count: 3,
        default_state: 159,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_log",
        first_state: 161,
        state_count: 3,
        default_state: 162,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 164,
        state_count: 3,
        default_state: 165,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 167,
        state_count: 3,
        default_state: 168,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 170,
        state_count: 3,
        default_state: 171,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 173,
        state_count: 3,
        default_state: 174,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 176,
        state_count: 3,
        default_state: 177,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 179,
        state_count: 3,
        default_state: 180,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_wood",
        first_state: 182,
        state_count: 3,
        default_state: 183,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 185,
        state_count: 3,
        default_state: 186,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 188,
        state_count: 3,
        default_state: 189,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 191,
        state_count: 3,
        default_state: 192,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 194,
        state_count: 3,
        default_state: 195,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 197,
        state_count: 3,
        default_state: 198,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 200,
        state_count: 3,
        default_state: 201,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_wood",
        first_state: 203,
        state_count: 3,
        default_state: 204,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 206,
        state_count: 28,
        default_state: 233,
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
        first_state: 234,
        state_count: 28,
        default_state: 261,
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
        first_state: 262,
        state_count: 28,
        default_state: 289,
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
        first_state: 290,
        state_count: 28,
        default_state: 317,
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
        first_state: 318,
        state_count: 28,
        default_state: 345,
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
        first_state: 346,
        state_count: 28,
        default_state: 373,
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
        first_state: 374,
        state_count: 28,
        default_state: 401,
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
        first_state: 402,
        state_count: 28,
        default_state: 429,
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
        first_state: 430,
        state_count: 28,
        default_state: 457,
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
        first_state: 458,
        state_count: 1,
        default_state: 458,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 459,
        state_count: 1,
        default_state: 459,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 460,
        state_count: 1,
        default_state: 460,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 461,
        state_count: 1,
        default_state: 461,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_lapis_ore",
        first_state: 462,
        state_count: 1,
        default_state: 462,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 463,
        state_count: 1,
        default_state: 463,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 464,
        state_count: 12,
        default_state: 465,
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
        first_state: 476,
        state_count: 1,
        default_state: 476,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 477,
        state_count: 1,
        default_state: 477,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 478,
        state_count: 1,
        default_state: 478,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 479,
        state_count: 800,
        default_state: 480,
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
        first_state: 1279,
        state_count: 16,
        default_state: 1282,
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
        first_state: 1295,
        state_count: 16,
        default_state: 1298,
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
        first_state: 1311,
        state_count: 16,
        default_state: 1314,
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
        first_state: 1327,
        state_count: 16,
        default_state: 1330,
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
        first_state: 1343,
        state_count: 16,
        default_state: 1346,
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
        first_state: 1359,
        state_count: 16,
        default_state: 1362,
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
        first_state: 1375,
        state_count: 16,
        default_state: 1378,
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
        first_state: 1391,
        state_count: 16,
        default_state: 1394,
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
        first_state: 1407,
        state_count: 16,
        default_state: 1410,
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
        first_state: 1423,
        state_count: 16,
        default_state: 1426,
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
        first_state: 1439,
        state_count: 16,
        default_state: 1442,
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
        first_state: 1455,
        state_count: 16,
        default_state: 1458,
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
        first_state: 1471,
        state_count: 16,
        default_state: 1474,
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
        first_state: 1487,
        state_count: 16,
        default_state: 1490,
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
        first_state: 1503,
        state_count: 16,
        default_state: 1506,
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
        first_state: 1519,
        state_count: 16,
        default_state: 1522,
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
        first_state: 1535,
        state_count: 24,
        default_state: 1548,
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
        first_state: 1559,
        state_count: 24,
        default_state: 1572,
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
        first_state: 1583,
        state_count: 12,
        default_state: 1589,
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
        first_state: 1595,
        state_count: 1,
        default_state: 1595,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 1596,
        state_count: 1,
        default_state: 1596,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 1597,
        state_count: 1,
        default_state: 1597,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 1598,
        state_count: 1,
        default_state: 1598,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 1599,
        state_count: 1,
        default_state: 1599,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 1600,
        state_count: 2,
        default_state: 1601,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 1602,
        state_count: 12,
        default_state: 1608,
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
        first_state: 1614,
        state_count: 24,
        default_state: 1616,
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
        first_state: 1638,
        state_count: 1,
        default_state: 1638,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 1639,
        state_count: 1,
        default_state: 1639,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 1640,
        state_count: 1,
        default_state: 1640,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 1641,
        state_count: 1,
        default_state: 1641,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 1642,
        state_count: 1,
        default_state: 1642,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 1643,
        state_count: 1,
        default_state: 1643,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 1644,
        state_count: 1,
        default_state: 1644,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 1645,
        state_count: 1,
        default_state: 1645,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 1646,
        state_count: 1,
        default_state: 1646,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 1647,
        state_count: 1,
        default_state: 1647,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 1648,
        state_count: 1,
        default_state: 1648,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 1649,
        state_count: 1,
        default_state: 1649,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 1650,
        state_count: 1,
        default_state: 1650,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 1651,
        state_count: 1,
        default_state: 1651,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 1652,
        state_count: 1,
        default_state: 1652,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 1653,
        state_count: 1,
        default_state: 1653,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 1654,
        state_count: 12,
        default_state: 1654,
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
        first_state: 1666,
        state_count: 1,
        default_state: 1666,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 1667,
        state_count: 1,
        default_state: 1667,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 1668,
        state_count: 1,
        default_state: 1668,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 1669,
        state_count: 1,
        default_state: 1669,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 1670,
        state_count: 1,
        default_state: 1670,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 1671,
        state_count: 1,
        default_state: 1671,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 1672,
        state_count: 1,
        default_state: 1672,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 1673,
        state_count: 1,
        default_state: 1673,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 1674,
        state_count: 1,
        default_state: 1674,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 1675,
        state_count: 1,
        default_state: 1675,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 1676,
        state_count: 1,
        default_state: 1676,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 1677,
        state_count: 1,
        default_state: 1677,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 1678,
        state_count: 1,
        default_state: 1678,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 1679,
        state_count: 1,
        default_state: 1679,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 1680,
        state_count: 1,
        default_state: 1680,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 1681,
        state_count: 1,
        default_state: 1681,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 1682,
        state_count: 1,
        default_state: 1682,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 1683,
        state_count: 1,
        default_state: 1683,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 1684,
        state_count: 2,
        default_state: 1685,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 1686,
        state_count: 1,
        default_state: 1686,
        props: &[],
    },
    BlockInfo {
        name: "mossy_cobblestone",
        first_state: 1687,
        state_count: 1,
        default_state: 1687,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 1688,
        state_count: 1,
        default_state: 1688,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 1689,
        state_count: 1,
        default_state: 1689,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 1690,
        state_count: 4,
        default_state: 1690,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 1694,
        state_count: 512,
        default_state: 1725,
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
        first_state: 2206,
        state_count: 1,
        default_state: 2206,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 2207,
        state_count: 1,
        default_state: 2207,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 2208,
        state_count: 80,
        default_state: 2219,
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
        first_state: 2288,
        state_count: 24,
        default_state: 2289,
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
        first_state: 2312,
        state_count: 1296,
        default_state: 3472,
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
        first_state: 3608,
        state_count: 1,
        default_state: 3608,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_diamond_ore",
        first_state: 3609,
        state_count: 1,
        default_state: 3609,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 3610,
        state_count: 1,
        default_state: 3610,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 3611,
        state_count: 1,
        default_state: 3611,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 3612,
        state_count: 8,
        default_state: 3612,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 3620,
        state_count: 8,
        default_state: 3620,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 3628,
        state_count: 8,
        default_state: 3629,
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
        first_state: 3636,
        state_count: 32,
        default_state: 3637,
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
        first_state: 3668,
        state_count: 32,
        default_state: 3669,
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
        first_state: 3700,
        state_count: 32,
        default_state: 3701,
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
        first_state: 3732,
        state_count: 32,
        default_state: 3733,
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
        first_state: 3764,
        state_count: 32,
        default_state: 3765,
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
        first_state: 3796,
        state_count: 32,
        default_state: 3797,
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
        first_state: 3828,
        state_count: 32,
        default_state: 3829,
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
        first_state: 3860,
        state_count: 64,
        default_state: 3871,
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
        first_state: 3924,
        state_count: 8,
        default_state: 3925,
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
        first_state: 3932,
        state_count: 20,
        default_state: 3933,
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
        first_state: 3952,
        state_count: 80,
        default_state: 3963,
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
        first_state: 4032,
        state_count: 8,
        default_state: 4033,
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
        first_state: 4040,
        state_count: 8,
        default_state: 4041,
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
        first_state: 4048,
        state_count: 8,
        default_state: 4049,
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
        first_state: 4056,
        state_count: 8,
        default_state: 4057,
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
        first_state: 4064,
        state_count: 8,
        default_state: 4065,
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
        first_state: 4072,
        state_count: 8,
        default_state: 4073,
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
        first_state: 4080,
        state_count: 8,
        default_state: 4081,
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
        first_state: 4088,
        state_count: 24,
        default_state: 4097,
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
        first_state: 4112,
        state_count: 2,
        default_state: 4113,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 4114,
        state_count: 64,
        default_state: 4125,
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
        first_state: 4178,
        state_count: 2,
        default_state: 4179,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 4180,
        state_count: 2,
        default_state: 4181,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 4182,
        state_count: 2,
        default_state: 4183,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 4184,
        state_count: 2,
        default_state: 4185,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 4186,
        state_count: 2,
        default_state: 4187,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 4188,
        state_count: 2,
        default_state: 4189,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "mangrove_pressure_plate",
        first_state: 4190,
        state_count: 2,
        default_state: 4191,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 4192,
        state_count: 2,
        default_state: 4193,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "deepslate_redstone_ore",
        first_state: 4194,
        state_count: 2,
        default_state: 4195,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 4196,
        state_count: 2,
        default_state: 4196,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 4198,
        state_count: 8,
        default_state: 4198,
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
        first_state: 4206,
        state_count: 24,
        default_state: 4215,
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
        first_state: 4230,
        state_count: 8,
        default_state: 4230,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 4238,
        state_count: 1,
        default_state: 4238,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 4239,
        state_count: 1,
        default_state: 4239,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 4240,
        state_count: 16,
        default_state: 4240,
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
        first_state: 4256,
        state_count: 1,
        default_state: 4256,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 4257,
        state_count: 16,
        default_state: 4257,
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
        first_state: 4273,
        state_count: 2,
        default_state: 4274,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 4275,
        state_count: 32,
        default_state: 4306,
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
        first_state: 4307,
        state_count: 1,
        default_state: 4307,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 4308,
        state_count: 1,
        default_state: 4308,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 4309,
        state_count: 1,
        default_state: 4309,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 4310,
        state_count: 1,
        default_state: 4310,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 4311,
        state_count: 3,
        default_state: 4312,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 4314,
        state_count: 3,
        default_state: 4315,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 4317,
        state_count: 1,
        default_state: 4317,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 4318,
        state_count: 4,
        default_state: 4318,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 4322,
        state_count: 1,
        default_state: 4322,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 4323,
        state_count: 2,
        default_state: 4323,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 4325,
        state_count: 4,
        default_state: 4325,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 4329,
        state_count: 4,
        default_state: 4329,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 4333,
        state_count: 7,
        default_state: 4333,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 4340,
        state_count: 64,
        default_state: 4343,
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
        first_state: 4404,
        state_count: 1,
        default_state: 4404,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 4405,
        state_count: 1,
        default_state: 4405,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 4406,
        state_count: 1,
        default_state: 4406,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 4407,
        state_count: 1,
        default_state: 4407,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 4408,
        state_count: 1,
        default_state: 4408,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 4409,
        state_count: 1,
        default_state: 4409,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 4410,
        state_count: 1,
        default_state: 4410,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 4411,
        state_count: 1,
        default_state: 4411,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 4412,
        state_count: 1,
        default_state: 4412,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 4413,
        state_count: 1,
        default_state: 4413,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 4414,
        state_count: 1,
        default_state: 4414,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 4415,
        state_count: 1,
        default_state: 4415,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 4416,
        state_count: 1,
        default_state: 4416,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 4417,
        state_count: 1,
        default_state: 4417,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 4418,
        state_count: 1,
        default_state: 4418,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 4419,
        state_count: 1,
        default_state: 4419,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 4420,
        state_count: 64,
        default_state: 4435,
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
        first_state: 4484,
        state_count: 64,
        default_state: 4499,
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
        first_state: 4548,
        state_count: 64,
        default_state: 4563,
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
        first_state: 4612,
        state_count: 64,
        default_state: 4627,
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
        first_state: 4676,
        state_count: 64,
        default_state: 4691,
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
        first_state: 4740,
        state_count: 64,
        default_state: 4755,
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
        first_state: 4804,
        state_count: 64,
        default_state: 4819,
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
        first_state: 4868,
        state_count: 1,
        default_state: 4868,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 4869,
        state_count: 1,
        default_state: 4869,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 4870,
        state_count: 1,
        default_state: 4870,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 4871,
        state_count: 1,
        default_state: 4871,
        props: &[],
    },
    BlockInfo {
        name: "packed_mud",
        first_state: 4872,
        state_count: 1,
        default_state: 4872,
        props: &[],
    },
    BlockInfo {
        name: "mud_bricks",
        first_state: 4873,
        state_count: 1,
        default_state: 4873,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 4874,
        state_count: 1,
        default_state: 4874,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 4875,
        state_count: 1,
        default_state: 4875,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 4876,
        state_count: 1,
        default_state: 4876,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 4877,
        state_count: 1,
        default_state: 4877,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 4878,
        state_count: 1,
        default_state: 4878,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 4879,
        state_count: 1,
        default_state: 4879,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 4880,
        state_count: 64,
        default_state: 4880,
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
        first_state: 4944,
        state_count: 64,
        default_state: 4944,
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
        first_state: 5008,
        state_count: 64,
        default_state: 5008,
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
        first_state: 5072,
        state_count: 32,
        default_state: 5103,
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
        first_state: 5104,
        state_count: 6,
        default_state: 5107,
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
        first_state: 5110,
        state_count: 32,
        default_state: 5141,
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
        first_state: 5142,
        state_count: 1,
        default_state: 5142,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 5143,
        state_count: 4,
        default_state: 5143,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 5147,
        state_count: 4,
        default_state: 5147,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 5151,
        state_count: 8,
        default_state: 5151,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 5159,
        state_count: 8,
        default_state: 5159,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 5167,
        state_count: 32,
        default_state: 5198,
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
        first_state: 5199,
        state_count: 128,
        default_state: 5326,
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
        first_state: 5327,
        state_count: 32,
        default_state: 5334,
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
        first_state: 5359,
        state_count: 80,
        default_state: 5370,
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
        first_state: 5439,
        state_count: 80,
        default_state: 5450,
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
        first_state: 5519,
        state_count: 80,
        default_state: 5530,
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
        first_state: 5599,
        state_count: 2,
        default_state: 5600,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 5601,
        state_count: 1,
        default_state: 5601,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 5602,
        state_count: 1,
        default_state: 5602,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 5603,
        state_count: 32,
        default_state: 5634,
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
        first_state: 5635,
        state_count: 80,
        default_state: 5646,
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
        first_state: 5715,
        state_count: 4,
        default_state: 5715,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 5719,
        state_count: 1,
        default_state: 5719,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 5720,
        state_count: 8,
        default_state: 5727,
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
        first_state: 5728,
        state_count: 1,
        default_state: 5728,
        props: &[],
    },
    BlockInfo {
        name: "water_cauldron",
        first_state: 5729,
        state_count: 3,
        default_state: 5729,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "lava_cauldron",
        first_state: 5732,
        state_count: 1,
        default_state: 5732,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow_cauldron",
        first_state: 5733,
        state_count: 3,
        default_state: 5733,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 5736,
        state_count: 1,
        default_state: 5736,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 5737,
        state_count: 8,
        default_state: 5741,
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
        first_state: 5745,
        state_count: 1,
        default_state: 5745,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 5746,
        state_count: 1,
        default_state: 5746,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 5747,
        state_count: 2,
        default_state: 5748,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 5749,
        state_count: 12,
        default_state: 5749,
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
        first_state: 5761,
        state_count: 80,
        default_state: 5772,
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
        first_state: 5841,
        state_count: 1,
        default_state: 5841,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_emerald_ore",
        first_state: 5842,
        state_count: 1,
        default_state: 5842,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 5843,
        state_count: 8,
        default_state: 5844,
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
        first_state: 5851,
        state_count: 16,
        default_state: 5860,
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
        first_state: 5867,
        state_count: 128,
        default_state: 5994,
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
        first_state: 5995,
        state_count: 1,
        default_state: 5995,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 5996,
        state_count: 80,
        default_state: 6007,
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
        first_state: 6076,
        state_count: 80,
        default_state: 6087,
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
        first_state: 6156,
        state_count: 80,
        default_state: 6167,
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
        first_state: 6236,
        state_count: 12,
        default_state: 6242,
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
        first_state: 6248,
        state_count: 1,
        default_state: 6248,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 6249,
        state_count: 324,
        default_state: 6252,
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
        first_state: 6573,
        state_count: 324,
        default_state: 6576,
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
        first_state: 6897,
        state_count: 1,
        default_state: 6897,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 6898,
        state_count: 1,
        default_state: 6898,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 6899,
        state_count: 1,
        default_state: 6899,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 6900,
        state_count: 1,
        default_state: 6900,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 6901,
        state_count: 1,
        default_state: 6901,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 6902,
        state_count: 1,
        default_state: 6902,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 6903,
        state_count: 1,
        default_state: 6903,
        props: &[],
    },
    BlockInfo {
        name: "potted_mangrove_propagule",
        first_state: 6904,
        state_count: 1,
        default_state: 6904,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 6905,
        state_count: 1,
        default_state: 6905,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 6906,
        state_count: 1,
        default_state: 6906,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 6907,
        state_count: 1,
        default_state: 6907,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 6908,
        state_count: 1,
        default_state: 6908,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 6909,
        state_count: 1,
        default_state: 6909,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 6910,
        state_count: 1,
        default_state: 6910,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 6911,
        state_count: 1,
        default_state: 6911,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 6912,
        state_count: 1,
        default_state: 6912,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 6913,
        state_count: 1,
        default_state: 6913,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 6914,
        state_count: 1,
        default_state: 6914,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 6915,
        state_count: 1,
        default_state: 6915,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 6916,
        state_count: 1,
        default_state: 6916,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 6917,
        state_count: 1,
        default_state: 6917,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 6918,
        state_count: 1,
        default_state: 6918,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 6919,
        state_count: 1,
        default_state: 6919,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 6920,
        state_count: 1,
        default_state: 6920,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 6921,
        state_count: 1,
        default_state: 6921,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 6922,
        state_count: 1,
        default_state: 6922,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 6923,
        state_count: 8,
        default_state: 6923,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 6931,
        state_count: 8,
        default_state: 6931,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 6939,
        state_count: 24,
        default_state: 6948,
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
        first_state: 6963,
        state_count: 24,
        default_state: 6972,
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
        first_state: 6987,
        state_count: 24,
        default_state: 6996,
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
        first_state: 7011,
        state_count: 24,
        default_state: 7020,
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
        first_state: 7035,
        state_count: 24,
        default_state: 7044,
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
        first_state: 7059,
        state_count: 24,
        default_state: 7068,
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
        first_state: 7083,
        state_count: 24,
        default_state: 7092,
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
        first_state: 7107,
        state_count: 16,
        default_state: 7107,
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
        first_state: 7123,
        state_count: 4,
        default_state: 7123,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 7127,
        state_count: 16,
        default_state: 7127,
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
        first_state: 7143,
        state_count: 4,
        default_state: 7143,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 7147,
        state_count: 16,
        default_state: 7147,
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
        first_state: 7163,
        state_count: 4,
        default_state: 7163,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 7167,
        state_count: 16,
        default_state: 7167,
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
        first_state: 7183,
        state_count: 4,
        default_state: 7183,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 7187,
        state_count: 16,
        default_state: 7187,
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
        first_state: 7203,
        state_count: 4,
        default_state: 7203,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 7207,
        state_count: 16,
        default_state: 7207,
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
        first_state: 7223,
        state_count: 4,
        default_state: 7223,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 7227,
        state_count: 4,
        default_state: 7227,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 7231,
        state_count: 4,
        default_state: 7231,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 7235,
        state_count: 4,
        default_state: 7235,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 7239,
        state_count: 24,
        default_state: 7240,
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
        first_state: 7263,
        state_count: 16,
        default_state: 7263,
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
        first_state: 7279,
        state_count: 16,
        default_state: 7279,
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
        first_state: 7295,
        state_count: 16,
        default_state: 7296,
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
        first_state: 7311,
        state_count: 32,
        default_state: 7327,
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
        first_state: 7343,
        state_count: 1,
        default_state: 7343,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 7344,
        state_count: 1,
        default_state: 7344,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 7345,
        state_count: 10,
        default_state: 7345,
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
        first_state: 7355,
        state_count: 1,
        default_state: 7355,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 7356,
        state_count: 1,
        default_state: 7356,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 7357,
        state_count: 3,
        default_state: 7358,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 7360,
        state_count: 80,
        default_state: 7371,
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
        first_state: 7440,
        state_count: 24,
        default_state: 7453,
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
        first_state: 7464,
        state_count: 12,
        default_state: 7465,
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
        first_state: 7476,
        state_count: 1,
        default_state: 7476,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 7477,
        state_count: 1,
        default_state: 7477,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 7478,
        state_count: 1,
        default_state: 7478,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 7479,
        state_count: 1,
        default_state: 7479,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 7480,
        state_count: 1,
        default_state: 7480,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 7481,
        state_count: 1,
        default_state: 7481,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 7482,
        state_count: 1,
        default_state: 7482,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 7483,
        state_count: 1,
        default_state: 7483,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 7484,
        state_count: 1,
        default_state: 7484,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 7485,
        state_count: 1,
        default_state: 7485,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 7486,
        state_count: 1,
        default_state: 7486,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 7487,
        state_count: 1,
        default_state: 7487,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 7488,
        state_count: 1,
        default_state: 7488,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 7489,
        state_count: 1,
        default_state: 7489,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 7490,
        state_count: 1,
        default_state: 7490,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 7491,
        state_count: 1,
        default_state: 7491,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 7492,
        state_count: 32,
        default_state: 7523,
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
        first_state: 7524,
        state_count: 32,
        default_state: 7555,
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
        first_state: 7556,
        state_count: 32,
        default_state: 7587,
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
        first_state: 7588,
        state_count: 32,
        default_state: 7619,
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
        first_state: 7620,
        state_count: 32,
        default_state: 7651,
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
        first_state: 7652,
        state_count: 32,
        default_state: 7683,
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
        first_state: 7684,
        state_count: 32,
        default_state: 7715,
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
        first_state: 7716,
        state_count: 32,
        default_state: 7747,
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
        first_state: 7748,
        state_count: 32,
        default_state: 7779,
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
        first_state: 7780,
        state_count: 32,
        default_state: 7811,
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
        first_state: 7812,
        state_count: 32,
        default_state: 7843,
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
        first_state: 7844,
        state_count: 32,
        default_state: 7875,
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
        first_state: 7876,
        state_count: 32,
        default_state: 7907,
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
        first_state: 7908,
        state_count: 32,
        default_state: 7939,
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
        first_state: 7940,
        state_count: 32,
        default_state: 7971,
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
        first_state: 7972,
        state_count: 32,
        default_state: 8003,
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
        first_state: 8004,
        state_count: 80,
        default_state: 8015,
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
        first_state: 8084,
        state_count: 80,
        default_state: 8095,
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
        first_state: 8164,
        state_count: 80,
        default_state: 8175,
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
        first_state: 8244,
        state_count: 1,
        default_state: 8244,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 8245,
        state_count: 1,
        default_state: 8245,
        props: &[],
    },
    BlockInfo {
        name: "light",
        first_state: 8246,
        state_count: 32,
        default_state: 8277,
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
        first_state: 8278,
        state_count: 64,
        default_state: 8293,
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
        first_state: 8342,
        state_count: 1,
        default_state: 8342,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 8343,
        state_count: 1,
        default_state: 8343,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 8344,
        state_count: 1,
        default_state: 8344,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 8345,
        state_count: 80,
        default_state: 8356,
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
        first_state: 8425,
        state_count: 80,
        default_state: 8436,
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
        first_state: 8505,
        state_count: 80,
        default_state: 8516,
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
        first_state: 8585,
        state_count: 6,
        default_state: 8588,
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
        first_state: 8591,
        state_count: 6,
        default_state: 8594,
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
        first_state: 8597,
        state_count: 6,
        default_state: 8600,
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
        first_state: 8603,
        state_count: 1,
        default_state: 8603,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 8604,
        state_count: 3,
        default_state: 8605,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 8607,
        state_count: 1,
        default_state: 8607,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 8608,
        state_count: 1,
        default_state: 8608,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 8609,
        state_count: 1,
        default_state: 8609,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 8610,
        state_count: 1,
        default_state: 8610,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 8611,
        state_count: 1,
        default_state: 8611,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 8612,
        state_count: 1,
        default_state: 8612,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 8613,
        state_count: 1,
        default_state: 8613,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 8614,
        state_count: 1,
        default_state: 8614,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 8615,
        state_count: 1,
        default_state: 8615,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 8616,
        state_count: 1,
        default_state: 8616,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 8617,
        state_count: 1,
        default_state: 8617,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 8618,
        state_count: 1,
        default_state: 8618,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 8619,
        state_count: 1,
        default_state: 8619,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 8620,
        state_count: 1,
        default_state: 8620,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 8621,
        state_count: 1,
        default_state: 8621,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 8622,
        state_count: 1,
        default_state: 8622,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 8623,
        state_count: 1,
        default_state: 8623,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 8624,
        state_count: 1,
        default_state: 8624,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 8625,
        state_count: 1,
        default_state: 8625,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 8626,
        state_count: 2,
        default_state: 8627,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 8628,
        state_count: 2,
        default_state: 8629,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 8630,
        state_count: 2,
        default_state: 8631,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 8632,
        state_count: 2,
        default_state: 8633,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 8634,
        state_count: 2,
        default_state: 8635,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 8636,
        state_count: 2,
        default_state: 8637,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 8638,
        state_count: 16,
        default_state: 8638,
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
        first_state: 8654,
        state_count: 16,
        default_state: 8654,
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
        first_state: 8670,
        state_count: 16,
        default_state: 8670,
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
        first_state: 8686,
        state_count: 16,
        default_state: 8686,
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
        first_state: 8702,
        state_count: 16,
        default_state: 8702,
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
        first_state: 8718,
        state_count: 16,
        default_state: 8718,
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
        first_state: 8734,
        state_count: 16,
        default_state: 8734,
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
        first_state: 8750,
        state_count: 16,
        default_state: 8750,
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
        first_state: 8766,
        state_count: 16,
        default_state: 8766,
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
        first_state: 8782,
        state_count: 16,
        default_state: 8782,
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
        first_state: 8798,
        state_count: 16,
        default_state: 8798,
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
        first_state: 8814,
        state_count: 16,
        default_state: 8814,
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
        first_state: 8830,
        state_count: 16,
        default_state: 8830,
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
        first_state: 8846,
        state_count: 16,
        default_state: 8846,
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
        first_state: 8862,
        state_count: 16,
        default_state: 8862,
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
        first_state: 8878,
        state_count: 16,
        default_state: 8878,
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
        first_state: 8894,
        state_count: 4,
        default_state: 8894,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 8898,
        state_count: 4,
        default_state: 8898,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 8902,
        state_count: 4,
        default_state: 8902,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 8906,
        state_count: 4,
        default_state: 8906,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 8910,
        state_count: 4,
        default_state: 8910,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 8914,
        state_count: 4,
        default_state: 8914,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 8918,
        state_count: 4,
        default_state: 8918,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 8922,
        state_count: 4,
        default_state: 8922,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 8926,
        state_count: 4,
        default_state: 8926,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 8930,
        state_count: 4,
        default_state: 8930,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 8934,
        state_count: 4,
        default_state: 8934,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 8938,
        state_count: 4,
        default_state: 8938,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 8942,
        state_count: 4,
        default_state: 8942,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 8946,
        state_count: 4,
        default_state: 8946,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 8950,
        state_count: 4,
        default_state: 8950,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 8954,
        state_count: 4,
        default_state: 8954,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 8958,
        state_count: 1,
        default_state: 8958,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 8959,
        state_count: 1,
        default_state: 8959,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 8960,
        state_count: 1,
        default_state: 8960,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 8961,
        state_count: 80,
        default_state: 8972,
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
        first_state: 9041,
        state_count: 6,
        default_state: 9044,
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
        first_state: 9047,
        state_count: 6,
        default_state: 9050,
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
        first_state: 9053,
        state_count: 6,
        default_state: 9056,
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
        first_state: 9059,
        state_count: 6,
        default_state: 9062,
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
        first_state: 9065,
        state_count: 6,
        default_state: 9068,
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
        first_state: 9071,
        state_count: 6,
        default_state: 9074,
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
        first_state: 9077,
        state_count: 6,
        default_state: 9080,
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
        first_state: 9083,
        state_count: 6,
        default_state: 9086,
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
        first_state: 9089,
        state_count: 6,
        default_state: 9092,
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
        first_state: 9095,
        state_count: 6,
        default_state: 9098,
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
        first_state: 9101,
        state_count: 6,
        default_state: 9104,
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
        first_state: 9107,
        state_count: 6,
        default_state: 9110,
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
        first_state: 9113,
        state_count: 6,
        default_state: 9116,
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
        first_state: 9119,
        state_count: 6,
        default_state: 9122,
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
        first_state: 9125,
        state_count: 6,
        default_state: 9128,
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
        first_state: 9131,
        state_count: 6,
        default_state: 9134,
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
        first_state: 9137,
        state_count: 6,
        default_state: 9140,
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
        first_state: 9143,
        state_count: 6,
        default_state: 9146,
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
        first_state: 9149,
        state_count: 6,
        default_state: 9152,
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
        first_state: 9155,
        state_count: 6,
        default_state: 9158,
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
        first_state: 9161,
        state_count: 6,
        default_state: 9164,
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
        first_state: 9167,
        state_count: 1,
        default_state: 9167,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 9168,
        state_count: 1,
        default_state: 9168,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 9169,
        state_count: 1,
        default_state: 9169,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 9170,
        state_count: 1,
        default_state: 9170,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 9171,
        state_count: 32,
        default_state: 9178,
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
        first_state: 9203,
        state_count: 32,
        default_state: 9210,
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
        first_state: 9235,
        state_count: 32,
        default_state: 9242,
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
        first_state: 9267,
        state_count: 32,
        default_state: 9274,
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
        first_state: 9299,
        state_count: 32,
        default_state: 9306,
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
        first_state: 9331,
        state_count: 32,
        default_state: 9338,
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
        first_state: 9363,
        state_count: 32,
        default_state: 9394,
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
        first_state: 9395,
        state_count: 32,
        default_state: 9426,
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
        first_state: 9427,
        state_count: 32,
        default_state: 9458,
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
        first_state: 9459,
        state_count: 32,
        default_state: 9490,
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
        first_state: 9491,
        state_count: 32,
        default_state: 9522,
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
        first_state: 9523,
        state_count: 32,
        default_state: 9554,
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
        first_state: 9555,
        state_count: 64,
        default_state: 9566,
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
        first_state: 9619,
        state_count: 64,
        default_state: 9630,
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
        first_state: 9683,
        state_count: 64,
        default_state: 9694,
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
        first_state: 9747,
        state_count: 64,
        default_state: 9758,
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
        first_state: 9811,
        state_count: 64,
        default_state: 9822,
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
        first_state: 9875,
        state_count: 64,
        default_state: 9886,
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
        first_state: 9939,
        state_count: 6,
        default_state: 9943,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 9945,
        state_count: 64,
        default_state: 10008,
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
        first_state: 10009,
        state_count: 6,
        default_state: 10009,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 10015,
        state_count: 1,
        default_state: 10015,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 10016,
        state_count: 3,
        default_state: 10017,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 10019,
        state_count: 80,
        default_state: 10030,
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
        first_state: 10099,
        state_count: 1,
        default_state: 10099,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 10100,
        state_count: 4,
        default_state: 10100,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "dirt_path",
        first_state: 10104,
        state_count: 1,
        default_state: 10104,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 10105,
        state_count: 1,
        default_state: 10105,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 10106,
        state_count: 12,
        default_state: 10112,
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
        first_state: 10118,
        state_count: 12,
        default_state: 10124,
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
        first_state: 10130,
        state_count: 4,
        default_state: 10130,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 10134,
        state_count: 1,
        default_state: 10134,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 10135,
        state_count: 1,
        default_state: 10135,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 10136,
        state_count: 1,
        default_state: 10136,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 10137,
        state_count: 3,
        default_state: 10138,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 10140,
        state_count: 1,
        default_state: 10140,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 10141,
        state_count: 12,
        default_state: 10146,
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
        first_state: 10153,
        state_count: 6,
        default_state: 10157,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 10159,
        state_count: 6,
        default_state: 10163,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 10165,
        state_count: 6,
        default_state: 10169,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 10171,
        state_count: 6,
        default_state: 10175,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 10177,
        state_count: 6,
        default_state: 10181,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 10183,
        state_count: 6,
        default_state: 10187,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 10189,
        state_count: 6,
        default_state: 10193,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 10195,
        state_count: 6,
        default_state: 10199,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 10201,
        state_count: 6,
        default_state: 10205,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 10207,
        state_count: 6,
        default_state: 10211,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 10213,
        state_count: 6,
        default_state: 10217,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 10219,
        state_count: 6,
        default_state: 10223,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 10225,
        state_count: 6,
        default_state: 10229,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 10231,
        state_count: 6,
        default_state: 10235,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 10237,
        state_count: 6,
        default_state: 10241,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 10243,
        state_count: 6,
        default_state: 10247,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 10249,
        state_count: 6,
        default_state: 10253,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 10255,
        state_count: 4,
        default_state: 10255,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 10259,
        state_count: 4,
        default_state: 10259,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 10263,
        state_count: 4,
        default_state: 10263,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 10267,
        state_count: 4,
        default_state: 10267,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 10271,
        state_count: 4,
        default_state: 10271,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 10275,
        state_count: 4,
        default_state: 10275,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 10279,
        state_count: 4,
        default_state: 10279,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 10283,
        state_count: 4,
        default_state: 10283,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 10287,
        state_count: 4,
        default_state: 10287,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 10291,
        state_count: 4,
        default_state: 10291,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 10295,
        state_count: 4,
        default_state: 10295,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 10299,
        state_count: 4,
        default_state: 10299,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 10303,
        state_count: 4,
        default_state: 10303,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 10307,
        state_count: 4,
        default_state: 10307,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 10311,
        state_count: 4,
        default_state: 10311,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 10315,
        state_count: 4,
        default_state: 10315,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 10319,
        state_count: 1,
        default_state: 10319,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 10320,
        state_count: 1,
        default_state: 10320,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 10321,
        state_count: 1,
        default_state: 10321,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 10322,
        state_count: 1,
        default_state: 10322,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 10323,
        state_count: 1,
        default_state: 10323,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 10324,
        state_count: 1,
        default_state: 10324,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 10325,
        state_count: 1,
        default_state: 10325,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 10326,
        state_count: 1,
        default_state: 10326,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 10327,
        state_count: 1,
        default_state: 10327,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 10328,
        state_count: 1,
        default_state: 10328,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 10329,
        state_count: 1,
        default_state: 10329,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 10330,
        state_count: 1,
        default_state: 10330,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 10331,
        state_count: 1,
        default_state: 10331,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 10332,
        state_count: 1,
        default_state: 10332,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 10333,
        state_count: 1,
        default_state: 10333,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 10334,
        state_count: 1,
        default_state: 10334,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 10335,
        state_count: 1,
        default_state: 10335,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 10336,
        state_count: 1,
        default_state: 10336,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 10337,
        state_count: 1,
        default_state: 10337,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 10338,
        state_count: 1,
        default_state: 10338,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 10339,
        state_count: 1,
        default_state: 10339,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 10340,
        state_count: 1,
        default_state: 10340,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 10341,
        state_count: 1,
        default_state: 10341,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 10342,
        state_count: 1,
        default_state: 10342,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 10343,
        state_count: 1,
        default_state: 10343,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 10344,
        state_count: 1,
        default_state: 10344,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 10345,
        state_count: 1,
        default_state: 10345,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 10346,
        state_count: 1,
        default_state: 10346,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 10347,
        state_count: 1,
        default_state: 10347,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 10348,
        state_count: 1,
        default_state: 10348,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 10349,
        state_count: 1,
        default_state: 10349,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 10350,
        state_count: 1,
        default_state: 10350,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 10351,
        state_count: 26,
        default_state: 10351,
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
        first_state: 10377,
        state_count: 1,
        default_state: 10377,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 10378,
        state_count: 1,
        default_state: 10378,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 10379,
        state_count: 12,
        default_state: 10379,
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
        first_state: 10391,
        state_count: 1,
        default_state: 10391,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 10392,
        state_count: 1,
        default_state: 10392,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 10393,
        state_count: 1,
        default_state: 10393,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 10394,
        state_count: 1,
        default_state: 10394,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 10395,
        state_count: 1,
        default_state: 10395,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 10396,
        state_count: 1,
        default_state: 10396,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 10397,
        state_count: 1,
        default_state: 10397,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 10398,
        state_count: 1,
        default_state: 10398,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 10399,
        state_count: 1,
        default_state: 10399,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 10400,
        state_count: 1,
        default_state: 10400,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 10401,
        state_count: 2,
        default_state: 10401,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 10403,
        state_count: 2,
        default_state: 10403,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 10405,
        state_count: 2,
        default_state: 10405,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 10407,
        state_count: 2,
        default_state: 10407,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 10409,
        state_count: 2,
        default_state: 10409,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 10411,
        state_count: 2,
        default_state: 10411,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 10413,
        state_count: 2,
        default_state: 10413,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 10415,
        state_count: 2,
        default_state: 10415,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 10417,
        state_count: 2,
        default_state: 10417,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 10419,
        state_count: 2,
        default_state: 10419,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 10421,
        state_count: 2,
        default_state: 10421,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 10423,
        state_count: 2,
        default_state: 10423,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 10425,
        state_count: 2,
        default_state: 10425,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 10427,
        state_count: 2,
        default_state: 10427,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 10429,
        state_count: 2,
        default_state: 10429,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 10431,
        state_count: 2,
        default_state: 10431,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 10433,
        state_count: 2,
        default_state: 10433,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 10435,
        state_count: 2,
        default_state: 10435,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 10437,
        state_count: 2,
        default_state: 10437,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 10439,
        state_count: 2,
        default_state: 10439,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 10441,
        state_count: 8,
        default_state: 10441,
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
        first_state: 10449,
        state_count: 8,
        default_state: 10449,
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
        first_state: 10457,
        state_count: 8,
        default_state: 10457,
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
        first_state: 10465,
        state_count: 8,
        default_state: 10465,
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
        first_state: 10473,
        state_count: 8,
        default_state: 10473,
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
        first_state: 10481,
        state_count: 8,
        default_state: 10481,
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
        first_state: 10489,
        state_count: 8,
        default_state: 10489,
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
        first_state: 10497,
        state_count: 8,
        default_state: 10497,
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
        first_state: 10505,
        state_count: 8,
        default_state: 10505,
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
        first_state: 10513,
        state_count: 8,
        default_state: 10513,
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
        first_state: 10521,
        state_count: 8,
        default_state: 10521,
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
        first_state: 10529,
        state_count: 1,
        default_state: 10529,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 10530,
        state_count: 2,
        default_state: 10530,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 10532,
        state_count: 1,
        default_state: 10532,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 10533,
        state_count: 12,
        default_state: 10533,
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
        first_state: 10545,
        state_count: 1,
        default_state: 10545,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 10546,
        state_count: 1,
        default_state: 10546,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 10547,
        state_count: 1,
        default_state: 10547,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 10548,
        state_count: 2,
        default_state: 10548,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 10550,
        state_count: 80,
        default_state: 10561,
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
        first_state: 10630,
        state_count: 80,
        default_state: 10641,
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
        first_state: 10710,
        state_count: 80,
        default_state: 10721,
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
        first_state: 10790,
        state_count: 80,
        default_state: 10801,
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
        first_state: 10870,
        state_count: 80,
        default_state: 10881,
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
        first_state: 10950,
        state_count: 80,
        default_state: 10961,
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
        first_state: 11030,
        state_count: 80,
        default_state: 11041,
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
        first_state: 11110,
        state_count: 80,
        default_state: 11121,
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
        first_state: 11190,
        state_count: 80,
        default_state: 11201,
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
        first_state: 11270,
        state_count: 80,
        default_state: 11281,
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
        first_state: 11350,
        state_count: 80,
        default_state: 11361,
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
        first_state: 11430,
        state_count: 80,
        default_state: 11441,
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
        first_state: 11510,
        state_count: 80,
        default_state: 11521,
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
        first_state: 11590,
        state_count: 80,
        default_state: 11601,
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
        first_state: 11670,
        state_count: 6,
        default_state: 11673,
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
        first_state: 11676,
        state_count: 6,
        default_state: 11679,
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
        first_state: 11682,
        state_count: 6,
        default_state: 11685,
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
        first_state: 11688,
        state_count: 6,
        default_state: 11691,
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
        first_state: 11694,
        state_count: 6,
        default_state: 11697,
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
        first_state: 11700,
        state_count: 6,
        default_state: 11703,
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
        first_state: 11706,
        state_count: 6,
        default_state: 11709,
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
        first_state: 11712,
        state_count: 6,
        default_state: 11715,
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
        first_state: 11718,
        state_count: 6,
        default_state: 11721,
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
        first_state: 11724,
        state_count: 6,
        default_state: 11727,
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
        first_state: 11730,
        state_count: 6,
        default_state: 11733,
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
        first_state: 11736,
        state_count: 6,
        default_state: 11739,
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
        first_state: 11742,
        state_count: 6,
        default_state: 11745,
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
        first_state: 11748,
        state_count: 324,
        default_state: 11751,
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
        first_state: 12072,
        state_count: 324,
        default_state: 12075,
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
        first_state: 12396,
        state_count: 324,
        default_state: 12399,
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
        first_state: 12720,
        state_count: 324,
        default_state: 12723,
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
        first_state: 13044,
        state_count: 324,
        default_state: 13047,
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
        first_state: 13368,
        state_count: 324,
        default_state: 13371,
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
        first_state: 13692,
        state_count: 324,
        default_state: 13695,
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
        first_state: 14016,
        state_count: 324,
        default_state: 14019,
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
        first_state: 14340,
        state_count: 324,
        default_state: 14343,
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
        first_state: 14664,
        state_count: 324,
        default_state: 14667,
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
        first_state: 14988,
        state_count: 324,
        default_state: 14991,
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
        first_state: 15312,
        state_count: 324,
        default_state: 15315,
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
        first_state: 15636,
        state_count: 324,
        default_state: 15639,
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
        first_state: 15960,
        state_count: 32,
        default_state: 15991,
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
        first_state: 15992,
        state_count: 4,
        default_state: 15992,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 15996,
        state_count: 12,
        default_state: 15997,
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
        first_state: 16008,
        state_count: 8,
        default_state: 16009,
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
        first_state: 16016,
        state_count: 8,
        default_state: 16017,
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
        first_state: 16024,
        state_count: 1,
        default_state: 16024,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 16025,
        state_count: 1,
        default_state: 16025,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 16026,
        state_count: 12,
        default_state: 16030,
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
        first_state: 16038,
        state_count: 16,
        default_state: 16041,
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
        first_state: 16054,
        state_count: 1,
        default_state: 16054,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 16055,
        state_count: 4,
        default_state: 16055,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 16059,
        state_count: 32,
        default_state: 16060,
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
        first_state: 16091,
        state_count: 4,
        default_state: 16094,
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
        first_state: 16095,
        state_count: 4,
        default_state: 16098,
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
        first_state: 16099,
        state_count: 32,
        default_state: 16102,
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
        first_state: 16131,
        state_count: 32,
        default_state: 16134,
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
        first_state: 16163,
        state_count: 4,
        default_state: 16163,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 16167,
        state_count: 3,
        default_state: 16168,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 16170,
        state_count: 3,
        default_state: 16171,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 16173,
        state_count: 3,
        default_state: 16174,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 16176,
        state_count: 3,
        default_state: 16177,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 16179,
        state_count: 1,
        default_state: 16179,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 16180,
        state_count: 1,
        default_state: 16180,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 16181,
        state_count: 1,
        default_state: 16181,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 16182,
        state_count: 1,
        default_state: 16182,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 16183,
        state_count: 1,
        default_state: 16183,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 16184,
        state_count: 3,
        default_state: 16185,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 16187,
        state_count: 3,
        default_state: 16188,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 16190,
        state_count: 3,
        default_state: 16191,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 16193,
        state_count: 3,
        default_state: 16194,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 16196,
        state_count: 1,
        default_state: 16196,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 16197,
        state_count: 1,
        default_state: 16197,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 16198,
        state_count: 1,
        default_state: 16198,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 16199,
        state_count: 26,
        default_state: 16199,
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
        first_state: 16225,
        state_count: 1,
        default_state: 16225,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 16226,
        state_count: 26,
        default_state: 16226,
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
        first_state: 16252,
        state_count: 1,
        default_state: 16252,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 16253,
        state_count: 1,
        default_state: 16253,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 16254,
        state_count: 1,
        default_state: 16254,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 16255,
        state_count: 1,
        default_state: 16255,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 16256,
        state_count: 6,
        default_state: 16259,
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
        first_state: 16262,
        state_count: 6,
        default_state: 16265,
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
        first_state: 16268,
        state_count: 2,
        default_state: 16269,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 16270,
        state_count: 2,
        default_state: 16271,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 16272,
        state_count: 32,
        default_state: 16303,
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
        first_state: 16304,
        state_count: 32,
        default_state: 16335,
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
        first_state: 16336,
        state_count: 64,
        default_state: 16351,
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
        first_state: 16400,
        state_count: 64,
        default_state: 16415,
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
        first_state: 16464,
        state_count: 32,
        default_state: 16471,
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
        first_state: 16496,
        state_count: 32,
        default_state: 16503,
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
        first_state: 16528,
        state_count: 80,
        default_state: 16539,
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
        first_state: 16608,
        state_count: 80,
        default_state: 16619,
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
        first_state: 16688,
        state_count: 24,
        default_state: 16697,
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
        first_state: 16712,
        state_count: 24,
        default_state: 16721,
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
        first_state: 16736,
        state_count: 64,
        default_state: 16747,
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
        first_state: 16800,
        state_count: 64,
        default_state: 16811,
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
        first_state: 16864,
        state_count: 32,
        default_state: 16865,
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
        first_state: 16896,
        state_count: 32,
        default_state: 16897,
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
        first_state: 16928,
        state_count: 8,
        default_state: 16929,
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
        first_state: 16936,
        state_count: 8,
        default_state: 16937,
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
        first_state: 16944,
        state_count: 4,
        default_state: 16945,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 16948,
        state_count: 12,
        default_state: 16958,
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
        first_state: 16960,
        state_count: 9,
        default_state: 16960,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 16969,
        state_count: 16,
        default_state: 16969,
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
        first_state: 16985,
        state_count: 24,
        default_state: 16985,
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
        first_state: 17009,
        state_count: 24,
        default_state: 17009,
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
        first_state: 17033,
        state_count: 1,
        default_state: 17033,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 17034,
        state_count: 1,
        default_state: 17034,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 17035,
        state_count: 1,
        default_state: 17035,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 17036,
        state_count: 1,
        default_state: 17036,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 17037,
        state_count: 1,
        default_state: 17037,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 17038,
        state_count: 5,
        default_state: 17038,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 17043,
        state_count: 1,
        default_state: 17043,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 17044,
        state_count: 1,
        default_state: 17044,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 17045,
        state_count: 1,
        default_state: 17045,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 17046,
        state_count: 1,
        default_state: 17046,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 17047,
        state_count: 1,
        default_state: 17047,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 17048,
        state_count: 1,
        default_state: 17048,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 17049,
        state_count: 80,
        default_state: 17060,
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
        first_state: 17129,
        state_count: 324,
        default_state: 17132,
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
        first_state: 17453,
        state_count: 6,
        default_state: 17456,
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
        first_state: 17459,
        state_count: 1,
        default_state: 17459,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 17460,
        state_count: 1,
        default_state: 17460,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 17461,
        state_count: 1,
        default_state: 17461,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 17462,
        state_count: 1,
        default_state: 17462,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 17463,
        state_count: 6,
        default_state: 17466,
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
        first_state: 17469,
        state_count: 80,
        default_state: 17480,
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
        first_state: 17549,
        state_count: 324,
        default_state: 17552,
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
        first_state: 17873,
        state_count: 1,
        default_state: 17873,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 17874,
        state_count: 80,
        default_state: 17885,
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
        first_state: 17954,
        state_count: 6,
        default_state: 17957,
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
        first_state: 17960,
        state_count: 2,
        default_state: 17961,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 17962,
        state_count: 24,
        default_state: 17971,
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
        first_state: 17986,
        state_count: 324,
        default_state: 17989,
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
        first_state: 18310,
        state_count: 1,
        default_state: 18310,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 18311,
        state_count: 1,
        default_state: 18311,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 18312,
        state_count: 1,
        default_state: 18312,
        props: &[],
    },
    BlockInfo {
        name: "candle",
        first_state: 18313,
        state_count: 16,
        default_state: 18316,
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
        first_state: 18329,
        state_count: 16,
        default_state: 18332,
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
        first_state: 18345,
        state_count: 16,
        default_state: 18348,
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
        first_state: 18361,
        state_count: 16,
        default_state: 18364,
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
        first_state: 18377,
        state_count: 16,
        default_state: 18380,
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
        first_state: 18393,
        state_count: 16,
        default_state: 18396,
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
        first_state: 18409,
        state_count: 16,
        default_state: 18412,
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
        first_state: 18425,
        state_count: 16,
        default_state: 18428,
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
        first_state: 18441,
        state_count: 16,
        default_state: 18444,
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
        first_state: 18457,
        state_count: 16,
        default_state: 18460,
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
        first_state: 18473,
        state_count: 16,
        default_state: 18476,
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
        first_state: 18489,
        state_count: 16,
        default_state: 18492,
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
        first_state: 18505,
        state_count: 16,
        default_state: 18508,
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
        first_state: 18521,
        state_count: 16,
        default_state: 18524,
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
        first_state: 18537,
        state_count: 16,
        default_state: 18540,
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
        first_state: 18553,
        state_count: 16,
        default_state: 18556,
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
        first_state: 18569,
        state_count: 16,
        default_state: 18572,
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
        first_state: 18585,
        state_count: 2,
        default_state: 18586,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "white_candle_cake",
        first_state: 18587,
        state_count: 2,
        default_state: 18588,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "orange_candle_cake",
        first_state: 18589,
        state_count: 2,
        default_state: 18590,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "magenta_candle_cake",
        first_state: 18591,
        state_count: 2,
        default_state: 18592,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_blue_candle_cake",
        first_state: 18593,
        state_count: 2,
        default_state: 18594,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "yellow_candle_cake",
        first_state: 18595,
        state_count: 2,
        default_state: 18596,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lime_candle_cake",
        first_state: 18597,
        state_count: 2,
        default_state: 18598,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pink_candle_cake",
        first_state: 18599,
        state_count: 2,
        default_state: 18600,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "gray_candle_cake",
        first_state: 18601,
        state_count: 2,
        default_state: 18602,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_gray_candle_cake",
        first_state: 18603,
        state_count: 2,
        default_state: 18604,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cyan_candle_cake",
        first_state: 18605,
        state_count: 2,
        default_state: 18606,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "purple_candle_cake",
        first_state: 18607,
        state_count: 2,
        default_state: 18608,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "blue_candle_cake",
        first_state: 18609,
        state_count: 2,
        default_state: 18610,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brown_candle_cake",
        first_state: 18611,
        state_count: 2,
        default_state: 18612,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "green_candle_cake",
        first_state: 18613,
        state_count: 2,
        default_state: 18614,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "red_candle_cake",
        first_state: 18615,
        state_count: 2,
        default_state: 18616,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "black_candle_cake",
        first_state: 18617,
        state_count: 2,
        default_state: 18618,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "amethyst_block",
        first_state: 18619,
        state_count: 1,
        default_state: 18619,
        props: &[],
    },
    BlockInfo {
        name: "budding_amethyst",
        first_state: 18620,
        state_count: 1,
        default_state: 18620,
        props: &[],
    },
    BlockInfo {
        name: "amethyst_cluster",
        first_state: 18621,
        state_count: 12,
        default_state: 18630,
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
        first_state: 18633,
        state_count: 12,
        default_state: 18642,
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
        first_state: 18645,
        state_count: 12,
        default_state: 18654,
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
        first_state: 18657,
        state_count: 12,
        default_state: 18666,
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
        first_state: 18669,
        state_count: 1,
        default_state: 18669,
        props: &[],
    },
    BlockInfo {
        name: "calcite",
        first_state: 18670,
        state_count: 1,
        default_state: 18670,
        props: &[],
    },
    BlockInfo {
        name: "tinted_glass",
        first_state: 18671,
        state_count: 1,
        default_state: 18671,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow",
        first_state: 18672,
        state_count: 1,
        default_state: 18672,
        props: &[],
    },
    BlockInfo {
        name: "sculk_sensor",
        first_state: 18673,
        state_count: 96,
        default_state: 18674,
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
        first_state: 18769,
        state_count: 1,
        default_state: 18769,
        props: &[],
    },
    BlockInfo {
        name: "sculk_vein",
        first_state: 18770,
        state_count: 128,
        default_state: 18897,
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
        first_state: 18898,
        state_count: 2,
        default_state: 18899,
        props: &[PropInfo {
            name: "bloom",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sculk_shrieker",
        first_state: 18900,
        state_count: 8,
        default_state: 18907,
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
        first_state: 18908,
        state_count: 1,
        default_state: 18908,
        props: &[],
    },
    BlockInfo {
        name: "weathered_copper",
        first_state: 18909,
        state_count: 1,
        default_state: 18909,
        props: &[],
    },
    BlockInfo {
        name: "exposed_copper",
        first_state: 18910,
        state_count: 1,
        default_state: 18910,
        props: &[],
    },
    BlockInfo {
        name: "copper_block",
        first_state: 18911,
        state_count: 1,
        default_state: 18911,
        props: &[],
    },
    BlockInfo {
        name: "copper_ore",
        first_state: 18912,
        state_count: 1,
        default_state: 18912,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_copper_ore",
        first_state: 18913,
        state_count: 1,
        default_state: 18913,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper",
        first_state: 18914,
        state_count: 1,
        default_state: 18914,
        props: &[],
    },
    BlockInfo {
        name: "weathered_cut_copper",
        first_state: 18915,
        state_count: 1,
        default_state: 18915,
        props: &[],
    },
    BlockInfo {
        name: "exposed_cut_copper",
        first_state: 18916,
        state_count: 1,
        default_state: 18916,
        props: &[],
    },
    BlockInfo {
        name: "cut_copper",
        first_state: 18917,
        state_count: 1,
        default_state: 18917,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper_stairs",
        first_state: 18918,
        state_count: 80,
        default_state: 18929,
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
        first_state: 18998,
        state_count: 80,
        default_state: 19009,
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
        first_state: 19078,
        state_count: 80,
        default_state: 19089,
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
        first_state: 19158,
        state_count: 80,
        default_state: 19169,
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
        first_state: 19238,
        state_count: 6,
        default_state: 19241,
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
        first_state: 19244,
        state_count: 6,
        default_state: 19247,
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
        first_state: 19250,
        state_count: 6,
        default_state: 19253,
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
        first_state: 19256,
        state_count: 6,
        default_state: 19259,
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
        first_state: 19262,
        state_count: 1,
        default_state: 19262,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_copper",
        first_state: 19263,
        state_count: 1,
        default_state: 19263,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_copper",
        first_state: 19264,
        state_count: 1,
        default_state: 19264,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_copper",
        first_state: 19265,
        state_count: 1,
        default_state: 19265,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper",
        first_state: 19266,
        state_count: 1,
        default_state: 19266,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper",
        first_state: 19267,
        state_count: 1,
        default_state: 19267,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper",
        first_state: 19268,
        state_count: 1,
        default_state: 19268,
        props: &[],
    },
    BlockInfo {
        name: "waxed_cut_copper",
        first_state: 19269,
        state_count: 1,
        default_state: 19269,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_stairs",
        first_state: 19270,
        state_count: 80,
        default_state: 19281,
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
        first_state: 19350,
        state_count: 80,
        default_state: 19361,
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
        first_state: 19430,
        state_count: 80,
        default_state: 19441,
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
        first_state: 19510,
        state_count: 80,
        default_state: 19521,
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
        first_state: 19590,
        state_count: 6,
        default_state: 19593,
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
        first_state: 19596,
        state_count: 6,
        default_state: 19599,
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
        first_state: 19602,
        state_count: 6,
        default_state: 19605,
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
        first_state: 19608,
        state_count: 6,
        default_state: 19611,
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
        first_state: 19614,
        state_count: 24,
        default_state: 19633,
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
        first_state: 19638,
        state_count: 20,
        default_state: 19643,
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
        first_state: 19658,
        state_count: 1,
        default_state: 19658,
        props: &[],
    },
    BlockInfo {
        name: "cave_vines",
        first_state: 19659,
        state_count: 52,
        default_state: 19660,
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
        first_state: 19711,
        state_count: 2,
        default_state: 19712,
        props: &[PropInfo {
            name: "berries",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spore_blossom",
        first_state: 19713,
        state_count: 1,
        default_state: 19713,
        props: &[],
    },
    BlockInfo {
        name: "azalea",
        first_state: 19714,
        state_count: 1,
        default_state: 19714,
        props: &[],
    },
    BlockInfo {
        name: "flowering_azalea",
        first_state: 19715,
        state_count: 1,
        default_state: 19715,
        props: &[],
    },
    BlockInfo {
        name: "moss_carpet",
        first_state: 19716,
        state_count: 1,
        default_state: 19716,
        props: &[],
    },
    BlockInfo {
        name: "moss_block",
        first_state: 19717,
        state_count: 1,
        default_state: 19717,
        props: &[],
    },
    BlockInfo {
        name: "big_dripleaf",
        first_state: 19718,
        state_count: 32,
        default_state: 19719,
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
        first_state: 19750,
        state_count: 8,
        default_state: 19751,
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
        first_state: 19758,
        state_count: 16,
        default_state: 19761,
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
        first_state: 19774,
        state_count: 2,
        default_state: 19775,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "rooted_dirt",
        first_state: 19776,
        state_count: 1,
        default_state: 19776,
        props: &[],
    },
    BlockInfo {
        name: "mud",
        first_state: 19777,
        state_count: 1,
        default_state: 19777,
        props: &[],
    },
    BlockInfo {
        name: "deepslate",
        first_state: 19778,
        state_count: 3,
        default_state: 19779,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cobbled_deepslate",
        first_state: 19781,
        state_count: 1,
        default_state: 19781,
        props: &[],
    },
    BlockInfo {
        name: "cobbled_deepslate_stairs",
        first_state: 19782,
        state_count: 80,
        default_state: 19793,
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
        first_state: 19862,
        state_count: 6,
        default_state: 19865,
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
        first_state: 19868,
        state_count: 324,
        default_state: 19871,
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
        first_state: 20192,
        state_count: 1,
        default_state: 20192,
        props: &[],
    },
    BlockInfo {
        name: "polished_deepslate_stairs",
        first_state: 20193,
        state_count: 80,
        default_state: 20204,
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
        first_state: 20273,
        state_count: 6,
        default_state: 20276,
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
        first_state: 20279,
        state_count: 324,
        default_state: 20282,
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
        first_state: 20603,
        state_count: 1,
        default_state: 20603,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_tile_stairs",
        first_state: 20604,
        state_count: 80,
        default_state: 20615,
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
        first_state: 20684,
        state_count: 6,
        default_state: 20687,
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
        first_state: 20690,
        state_count: 324,
        default_state: 20693,
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
        first_state: 21014,
        state_count: 1,
        default_state: 21014,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_brick_stairs",
        first_state: 21015,
        state_count: 80,
        default_state: 21026,
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
        first_state: 21095,
        state_count: 6,
        default_state: 21098,
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
        first_state: 21101,
        state_count: 324,
        default_state: 21104,
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
        first_state: 21425,
        state_count: 1,
        default_state: 21425,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_bricks",
        first_state: 21426,
        state_count: 1,
        default_state: 21426,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_tiles",
        first_state: 21427,
        state_count: 1,
        default_state: 21427,
        props: &[],
    },
    BlockInfo {
        name: "infested_deepslate",
        first_state: 21428,
        state_count: 3,
        default_state: 21429,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "smooth_basalt",
        first_state: 21431,
        state_count: 1,
        default_state: 21431,
        props: &[],
    },
    BlockInfo {
        name: "raw_iron_block",
        first_state: 21432,
        state_count: 1,
        default_state: 21432,
        props: &[],
    },
    BlockInfo {
        name: "raw_copper_block",
        first_state: 21433,
        state_count: 1,
        default_state: 21433,
        props: &[],
    },
    BlockInfo {
        name: "raw_gold_block",
        first_state: 21434,
        state_count: 1,
        default_state: 21434,
        props: &[],
    },
    BlockInfo {
        name: "potted_azalea_bush",
        first_state: 21435,
        state_count: 1,
        default_state: 21435,
        props: &[],
    },
    BlockInfo {
        name: "potted_flowering_azalea_bush",
        first_state: 21436,
        state_count: 1,
        default_state: 21436,
        props: &[],
    },
    BlockInfo {
        name: "ochre_froglight",
        first_state: 21437,
        state_count: 3,
        default_state: 21438,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "verdant_froglight",
        first_state: 21440,
        state_count: 3,
        default_state: 21441,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pearlescent_froglight",
        first_state: 21443,
        state_count: 3,
        default_state: 21444,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "frogspawn",
        first_state: 21446,
        state_count: 1,
        default_state: 21446,
        props: &[],
    },
    BlockInfo {
        name: "reinforced_deepslate",
        first_state: 21447,
        state_count: 1,
        default_state: 21447,
        props: &[],
    },
];
