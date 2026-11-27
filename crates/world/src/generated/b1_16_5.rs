// @generated `cargo xtask gen-blockstates 1.16.5` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.16.5";
pub const STATE_COUNT: u32 = 17112;

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
        name: "oak_sapling",
        first_state: 21,
        state_count: 2,
        default_state: 21,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "spruce_sapling",
        first_state: 23,
        state_count: 2,
        default_state: 23,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "birch_sapling",
        first_state: 25,
        state_count: 2,
        default_state: 25,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "jungle_sapling",
        first_state: 27,
        state_count: 2,
        default_state: 27,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "acacia_sapling",
        first_state: 29,
        state_count: 2,
        default_state: 29,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "dark_oak_sapling",
        first_state: 31,
        state_count: 2,
        default_state: 31,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "bedrock",
        first_state: 33,
        state_count: 1,
        default_state: 33,
        props: &[],
    },
    BlockInfo {
        name: "water",
        first_state: 34,
        state_count: 16,
        default_state: 34,
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
        first_state: 50,
        state_count: 16,
        default_state: 50,
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
        first_state: 66,
        state_count: 1,
        default_state: 66,
        props: &[],
    },
    BlockInfo {
        name: "red_sand",
        first_state: 67,
        state_count: 1,
        default_state: 67,
        props: &[],
    },
    BlockInfo {
        name: "gravel",
        first_state: 68,
        state_count: 1,
        default_state: 68,
        props: &[],
    },
    BlockInfo {
        name: "gold_ore",
        first_state: 69,
        state_count: 1,
        default_state: 69,
        props: &[],
    },
    BlockInfo {
        name: "iron_ore",
        first_state: 70,
        state_count: 1,
        default_state: 70,
        props: &[],
    },
    BlockInfo {
        name: "coal_ore",
        first_state: 71,
        state_count: 1,
        default_state: 71,
        props: &[],
    },
    BlockInfo {
        name: "nether_gold_ore",
        first_state: 72,
        state_count: 1,
        default_state: 72,
        props: &[],
    },
    BlockInfo {
        name: "oak_log",
        first_state: 73,
        state_count: 3,
        default_state: 74,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 76,
        state_count: 3,
        default_state: 77,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 79,
        state_count: 3,
        default_state: 80,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 82,
        state_count: 3,
        default_state: 83,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 85,
        state_count: 3,
        default_state: 86,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 88,
        state_count: 3,
        default_state: 89,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 91,
        state_count: 3,
        default_state: 92,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 94,
        state_count: 3,
        default_state: 95,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 97,
        state_count: 3,
        default_state: 98,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 100,
        state_count: 3,
        default_state: 101,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 103,
        state_count: 3,
        default_state: 104,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 106,
        state_count: 3,
        default_state: 107,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 109,
        state_count: 3,
        default_state: 110,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 112,
        state_count: 3,
        default_state: 113,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 115,
        state_count: 3,
        default_state: 116,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 118,
        state_count: 3,
        default_state: 119,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 121,
        state_count: 3,
        default_state: 122,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 124,
        state_count: 3,
        default_state: 125,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 127,
        state_count: 3,
        default_state: 128,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 130,
        state_count: 3,
        default_state: 131,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 133,
        state_count: 3,
        default_state: 134,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 136,
        state_count: 3,
        default_state: 137,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 139,
        state_count: 3,
        default_state: 140,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 142,
        state_count: 3,
        default_state: 143,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 145,
        state_count: 14,
        default_state: 158,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "spruce_leaves",
        first_state: 159,
        state_count: 14,
        default_state: 172,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "birch_leaves",
        first_state: 173,
        state_count: 14,
        default_state: 186,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "jungle_leaves",
        first_state: 187,
        state_count: 14,
        default_state: 200,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "acacia_leaves",
        first_state: 201,
        state_count: 14,
        default_state: 214,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dark_oak_leaves",
        first_state: 215,
        state_count: 14,
        default_state: 228,
        props: &[
            PropInfo {
                name: "distance",
                values: &["1", "2", "3", "4", "5", "6", "7"],
            },
            PropInfo {
                name: "persistent",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "sponge",
        first_state: 229,
        state_count: 1,
        default_state: 229,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 230,
        state_count: 1,
        default_state: 230,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 231,
        state_count: 1,
        default_state: 231,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 232,
        state_count: 1,
        default_state: 232,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 233,
        state_count: 1,
        default_state: 233,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 234,
        state_count: 12,
        default_state: 235,
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
        first_state: 246,
        state_count: 1,
        default_state: 246,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 247,
        state_count: 1,
        default_state: 247,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 248,
        state_count: 1,
        default_state: 248,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 249,
        state_count: 800,
        default_state: 250,
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
        first_state: 1049,
        state_count: 16,
        default_state: 1052,
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
        first_state: 1065,
        state_count: 16,
        default_state: 1068,
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
        first_state: 1081,
        state_count: 16,
        default_state: 1084,
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
        first_state: 1097,
        state_count: 16,
        default_state: 1100,
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
        first_state: 1113,
        state_count: 16,
        default_state: 1116,
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
        first_state: 1129,
        state_count: 16,
        default_state: 1132,
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
        first_state: 1145,
        state_count: 16,
        default_state: 1148,
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
        first_state: 1161,
        state_count: 16,
        default_state: 1164,
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
        first_state: 1177,
        state_count: 16,
        default_state: 1180,
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
        first_state: 1193,
        state_count: 16,
        default_state: 1196,
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
        first_state: 1209,
        state_count: 16,
        default_state: 1212,
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
        first_state: 1225,
        state_count: 16,
        default_state: 1228,
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
        first_state: 1241,
        state_count: 16,
        default_state: 1244,
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
        first_state: 1257,
        state_count: 16,
        default_state: 1260,
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
        first_state: 1273,
        state_count: 16,
        default_state: 1276,
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
        first_state: 1289,
        state_count: 16,
        default_state: 1292,
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
        first_state: 1305,
        state_count: 12,
        default_state: 1311,
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
        ],
    },
    BlockInfo {
        name: "detector_rail",
        first_state: 1317,
        state_count: 12,
        default_state: 1323,
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
        ],
    },
    BlockInfo {
        name: "sticky_piston",
        first_state: 1329,
        state_count: 12,
        default_state: 1335,
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
        first_state: 1341,
        state_count: 1,
        default_state: 1341,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 1342,
        state_count: 1,
        default_state: 1342,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 1343,
        state_count: 1,
        default_state: 1343,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 1344,
        state_count: 1,
        default_state: 1344,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 1345,
        state_count: 1,
        default_state: 1345,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 1346,
        state_count: 2,
        default_state: 1347,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 1348,
        state_count: 12,
        default_state: 1354,
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
        first_state: 1360,
        state_count: 24,
        default_state: 1362,
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
        first_state: 1384,
        state_count: 1,
        default_state: 1384,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 1385,
        state_count: 1,
        default_state: 1385,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 1386,
        state_count: 1,
        default_state: 1386,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 1387,
        state_count: 1,
        default_state: 1387,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 1388,
        state_count: 1,
        default_state: 1388,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 1389,
        state_count: 1,
        default_state: 1389,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 1390,
        state_count: 1,
        default_state: 1390,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 1391,
        state_count: 1,
        default_state: 1391,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 1392,
        state_count: 1,
        default_state: 1392,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 1393,
        state_count: 1,
        default_state: 1393,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 1394,
        state_count: 1,
        default_state: 1394,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 1395,
        state_count: 1,
        default_state: 1395,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 1396,
        state_count: 1,
        default_state: 1396,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 1397,
        state_count: 1,
        default_state: 1397,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 1398,
        state_count: 1,
        default_state: 1398,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 1399,
        state_count: 1,
        default_state: 1399,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 1400,
        state_count: 12,
        default_state: 1400,
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
        first_state: 1412,
        state_count: 1,
        default_state: 1412,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 1413,
        state_count: 1,
        default_state: 1413,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 1414,
        state_count: 1,
        default_state: 1414,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 1415,
        state_count: 1,
        default_state: 1415,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 1416,
        state_count: 1,
        default_state: 1416,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 1417,
        state_count: 1,
        default_state: 1417,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 1418,
        state_count: 1,
        default_state: 1418,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 1419,
        state_count: 1,
        default_state: 1419,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 1420,
        state_count: 1,
        default_state: 1420,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 1421,
        state_count: 1,
        default_state: 1421,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 1422,
        state_count: 1,
        default_state: 1422,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 1423,
        state_count: 1,
        default_state: 1423,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 1424,
        state_count: 1,
        default_state: 1424,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 1425,
        state_count: 1,
        default_state: 1425,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 1426,
        state_count: 1,
        default_state: 1426,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 1427,
        state_count: 1,
        default_state: 1427,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 1428,
        state_count: 1,
        default_state: 1428,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 1429,
        state_count: 1,
        default_state: 1429,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 1430,
        state_count: 2,
        default_state: 1431,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 1432,
        state_count: 1,
        default_state: 1432,
        props: &[],
    },
    BlockInfo {
        name: "mossy_cobblestone",
        first_state: 1433,
        state_count: 1,
        default_state: 1433,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 1434,
        state_count: 1,
        default_state: 1434,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 1435,
        state_count: 1,
        default_state: 1435,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 1436,
        state_count: 4,
        default_state: 1436,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 1440,
        state_count: 512,
        default_state: 1471,
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
        first_state: 1952,
        state_count: 1,
        default_state: 1952,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 1953,
        state_count: 1,
        default_state: 1953,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 1954,
        state_count: 80,
        default_state: 1965,
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
        first_state: 2034,
        state_count: 24,
        default_state: 2035,
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
        first_state: 2058,
        state_count: 1296,
        default_state: 3218,
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
        first_state: 3354,
        state_count: 1,
        default_state: 3354,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 3355,
        state_count: 1,
        default_state: 3355,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 3356,
        state_count: 1,
        default_state: 3356,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 3357,
        state_count: 8,
        default_state: 3357,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 3365,
        state_count: 8,
        default_state: 3365,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 3373,
        state_count: 8,
        default_state: 3374,
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
        first_state: 3381,
        state_count: 32,
        default_state: 3382,
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
        first_state: 3413,
        state_count: 32,
        default_state: 3414,
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
        first_state: 3445,
        state_count: 32,
        default_state: 3446,
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
        first_state: 3477,
        state_count: 32,
        default_state: 3478,
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
        first_state: 3509,
        state_count: 32,
        default_state: 3510,
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
        first_state: 3541,
        state_count: 32,
        default_state: 3542,
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
        first_state: 3573,
        state_count: 64,
        default_state: 3584,
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
        first_state: 3637,
        state_count: 8,
        default_state: 3638,
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
        first_state: 3645,
        state_count: 10,
        default_state: 3645,
        props: &[PropInfo {
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
        }],
    },
    BlockInfo {
        name: "cobblestone_stairs",
        first_state: 3655,
        state_count: 80,
        default_state: 3666,
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
        first_state: 3735,
        state_count: 8,
        default_state: 3736,
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
        first_state: 3743,
        state_count: 8,
        default_state: 3744,
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
        first_state: 3751,
        state_count: 8,
        default_state: 3752,
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
        first_state: 3759,
        state_count: 8,
        default_state: 3760,
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
        first_state: 3767,
        state_count: 8,
        default_state: 3768,
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
        first_state: 3775,
        state_count: 8,
        default_state: 3776,
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
        first_state: 3783,
        state_count: 24,
        default_state: 3792,
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
        first_state: 3807,
        state_count: 2,
        default_state: 3808,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 3809,
        state_count: 64,
        default_state: 3820,
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
        first_state: 3873,
        state_count: 2,
        default_state: 3874,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 3875,
        state_count: 2,
        default_state: 3876,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 3877,
        state_count: 2,
        default_state: 3878,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 3879,
        state_count: 2,
        default_state: 3880,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 3881,
        state_count: 2,
        default_state: 3882,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 3883,
        state_count: 2,
        default_state: 3884,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 3885,
        state_count: 2,
        default_state: 3886,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 3887,
        state_count: 2,
        default_state: 3887,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 3889,
        state_count: 8,
        default_state: 3889,
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
        first_state: 3897,
        state_count: 24,
        default_state: 3906,
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
        first_state: 3921,
        state_count: 8,
        default_state: 3921,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 3929,
        state_count: 1,
        default_state: 3929,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 3930,
        state_count: 1,
        default_state: 3930,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 3931,
        state_count: 16,
        default_state: 3931,
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
        first_state: 3947,
        state_count: 1,
        default_state: 3947,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 3948,
        state_count: 16,
        default_state: 3948,
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
        first_state: 3964,
        state_count: 2,
        default_state: 3965,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 3966,
        state_count: 32,
        default_state: 3997,
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
        first_state: 3998,
        state_count: 1,
        default_state: 3998,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 3999,
        state_count: 1,
        default_state: 3999,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 4000,
        state_count: 1,
        default_state: 4000,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 4001,
        state_count: 1,
        default_state: 4001,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 4002,
        state_count: 3,
        default_state: 4003,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 4005,
        state_count: 3,
        default_state: 4006,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 4008,
        state_count: 1,
        default_state: 4008,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 4009,
        state_count: 4,
        default_state: 4009,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 4013,
        state_count: 1,
        default_state: 4013,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 4014,
        state_count: 2,
        default_state: 4014,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 4016,
        state_count: 4,
        default_state: 4016,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 4020,
        state_count: 4,
        default_state: 4020,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 4024,
        state_count: 7,
        default_state: 4024,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 4031,
        state_count: 64,
        default_state: 4034,
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
        first_state: 4095,
        state_count: 1,
        default_state: 4095,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 4096,
        state_count: 1,
        default_state: 4096,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 4097,
        state_count: 1,
        default_state: 4097,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 4098,
        state_count: 1,
        default_state: 4098,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 4099,
        state_count: 1,
        default_state: 4099,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 4100,
        state_count: 1,
        default_state: 4100,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 4101,
        state_count: 1,
        default_state: 4101,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 4102,
        state_count: 1,
        default_state: 4102,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 4103,
        state_count: 1,
        default_state: 4103,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 4104,
        state_count: 1,
        default_state: 4104,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 4105,
        state_count: 1,
        default_state: 4105,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 4106,
        state_count: 1,
        default_state: 4106,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 4107,
        state_count: 1,
        default_state: 4107,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 4108,
        state_count: 1,
        default_state: 4108,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 4109,
        state_count: 1,
        default_state: 4109,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 4110,
        state_count: 1,
        default_state: 4110,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 4111,
        state_count: 64,
        default_state: 4126,
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
        first_state: 4175,
        state_count: 64,
        default_state: 4190,
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
        first_state: 4239,
        state_count: 64,
        default_state: 4254,
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
        first_state: 4303,
        state_count: 64,
        default_state: 4318,
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
        first_state: 4367,
        state_count: 64,
        default_state: 4382,
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
        first_state: 4431,
        state_count: 64,
        default_state: 4446,
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
        first_state: 4495,
        state_count: 1,
        default_state: 4495,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 4496,
        state_count: 1,
        default_state: 4496,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 4497,
        state_count: 1,
        default_state: 4497,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 4498,
        state_count: 1,
        default_state: 4498,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 4499,
        state_count: 1,
        default_state: 4499,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 4500,
        state_count: 1,
        default_state: 4500,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 4501,
        state_count: 1,
        default_state: 4501,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 4502,
        state_count: 1,
        default_state: 4502,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 4503,
        state_count: 1,
        default_state: 4503,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 4504,
        state_count: 1,
        default_state: 4504,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 4505,
        state_count: 64,
        default_state: 4505,
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
        first_state: 4569,
        state_count: 64,
        default_state: 4569,
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
        first_state: 4633,
        state_count: 64,
        default_state: 4633,
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
        first_state: 4697,
        state_count: 32,
        default_state: 4728,
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
        first_state: 4729,
        state_count: 6,
        default_state: 4732,
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
        first_state: 4735,
        state_count: 32,
        default_state: 4766,
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
        first_state: 4767,
        state_count: 1,
        default_state: 4767,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 4768,
        state_count: 4,
        default_state: 4768,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 4772,
        state_count: 4,
        default_state: 4772,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 4776,
        state_count: 8,
        default_state: 4776,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 4784,
        state_count: 8,
        default_state: 4784,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 4792,
        state_count: 32,
        default_state: 4823,
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
        name: "oak_fence_gate",
        first_state: 4824,
        state_count: 32,
        default_state: 4831,
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
        first_state: 4856,
        state_count: 80,
        default_state: 4867,
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
        first_state: 4936,
        state_count: 80,
        default_state: 4947,
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
        first_state: 5016,
        state_count: 2,
        default_state: 5017,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 5018,
        state_count: 1,
        default_state: 5018,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 5019,
        state_count: 1,
        default_state: 5019,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 5020,
        state_count: 32,
        default_state: 5051,
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
        first_state: 5052,
        state_count: 80,
        default_state: 5063,
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
        first_state: 5132,
        state_count: 4,
        default_state: 5132,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 5136,
        state_count: 1,
        default_state: 5136,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 5137,
        state_count: 8,
        default_state: 5144,
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
        first_state: 5145,
        state_count: 4,
        default_state: 5145,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 5149,
        state_count: 1,
        default_state: 5149,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 5150,
        state_count: 8,
        default_state: 5154,
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
        first_state: 5158,
        state_count: 1,
        default_state: 5158,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 5159,
        state_count: 1,
        default_state: 5159,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 5160,
        state_count: 2,
        default_state: 5161,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 5162,
        state_count: 12,
        default_state: 5162,
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
        first_state: 5174,
        state_count: 80,
        default_state: 5185,
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
        first_state: 5254,
        state_count: 1,
        default_state: 5254,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 5255,
        state_count: 8,
        default_state: 5256,
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
        first_state: 5263,
        state_count: 16,
        default_state: 5272,
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
        first_state: 5279,
        state_count: 128,
        default_state: 5406,
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
        first_state: 5407,
        state_count: 1,
        default_state: 5407,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 5408,
        state_count: 80,
        default_state: 5419,
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
        first_state: 5488,
        state_count: 80,
        default_state: 5499,
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
        first_state: 5568,
        state_count: 80,
        default_state: 5579,
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
        first_state: 5648,
        state_count: 12,
        default_state: 5654,
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
        first_state: 5660,
        state_count: 1,
        default_state: 5660,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 5661,
        state_count: 324,
        default_state: 5664,
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
        first_state: 5985,
        state_count: 324,
        default_state: 5988,
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
        first_state: 6309,
        state_count: 1,
        default_state: 6309,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 6310,
        state_count: 1,
        default_state: 6310,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 6311,
        state_count: 1,
        default_state: 6311,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 6312,
        state_count: 1,
        default_state: 6312,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 6313,
        state_count: 1,
        default_state: 6313,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 6314,
        state_count: 1,
        default_state: 6314,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 6315,
        state_count: 1,
        default_state: 6315,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 6316,
        state_count: 1,
        default_state: 6316,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 6317,
        state_count: 1,
        default_state: 6317,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 6318,
        state_count: 1,
        default_state: 6318,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 6319,
        state_count: 1,
        default_state: 6319,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 6320,
        state_count: 1,
        default_state: 6320,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 6321,
        state_count: 1,
        default_state: 6321,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 6322,
        state_count: 1,
        default_state: 6322,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 6323,
        state_count: 1,
        default_state: 6323,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 6324,
        state_count: 1,
        default_state: 6324,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 6325,
        state_count: 1,
        default_state: 6325,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 6326,
        state_count: 1,
        default_state: 6326,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 6327,
        state_count: 1,
        default_state: 6327,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 6328,
        state_count: 1,
        default_state: 6328,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 6329,
        state_count: 1,
        default_state: 6329,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 6330,
        state_count: 1,
        default_state: 6330,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 6331,
        state_count: 1,
        default_state: 6331,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 6332,
        state_count: 1,
        default_state: 6332,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 6333,
        state_count: 1,
        default_state: 6333,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 6334,
        state_count: 8,
        default_state: 6334,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 6342,
        state_count: 8,
        default_state: 6342,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 6350,
        state_count: 24,
        default_state: 6359,
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
        first_state: 6374,
        state_count: 24,
        default_state: 6383,
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
        first_state: 6398,
        state_count: 24,
        default_state: 6407,
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
        first_state: 6422,
        state_count: 24,
        default_state: 6431,
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
        first_state: 6446,
        state_count: 24,
        default_state: 6455,
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
        first_state: 6470,
        state_count: 24,
        default_state: 6479,
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
        first_state: 6494,
        state_count: 16,
        default_state: 6494,
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
        first_state: 6510,
        state_count: 4,
        default_state: 6510,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 6514,
        state_count: 16,
        default_state: 6514,
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
        first_state: 6530,
        state_count: 4,
        default_state: 6530,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 6534,
        state_count: 16,
        default_state: 6534,
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
        first_state: 6550,
        state_count: 4,
        default_state: 6550,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 6554,
        state_count: 16,
        default_state: 6554,
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
        first_state: 6570,
        state_count: 4,
        default_state: 6570,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 6574,
        state_count: 16,
        default_state: 6574,
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
        first_state: 6590,
        state_count: 4,
        default_state: 6590,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 6594,
        state_count: 16,
        default_state: 6594,
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
        first_state: 6610,
        state_count: 4,
        default_state: 6610,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 6614,
        state_count: 4,
        default_state: 6614,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 6618,
        state_count: 4,
        default_state: 6618,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 6622,
        state_count: 4,
        default_state: 6622,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 6626,
        state_count: 24,
        default_state: 6627,
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
        first_state: 6650,
        state_count: 16,
        default_state: 6650,
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
        first_state: 6666,
        state_count: 16,
        default_state: 6666,
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
        first_state: 6682,
        state_count: 16,
        default_state: 6683,
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
        first_state: 6698,
        state_count: 32,
        default_state: 6714,
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
        first_state: 6730,
        state_count: 1,
        default_state: 6730,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 6731,
        state_count: 1,
        default_state: 6731,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 6732,
        state_count: 10,
        default_state: 6732,
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
        first_state: 6742,
        state_count: 1,
        default_state: 6742,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 6743,
        state_count: 1,
        default_state: 6743,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 6744,
        state_count: 3,
        default_state: 6745,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 6747,
        state_count: 80,
        default_state: 6758,
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
        first_state: 6827,
        state_count: 12,
        default_state: 6833,
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
        ],
    },
    BlockInfo {
        name: "dropper",
        first_state: 6839,
        state_count: 12,
        default_state: 6840,
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
        first_state: 6851,
        state_count: 1,
        default_state: 6851,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 6852,
        state_count: 1,
        default_state: 6852,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 6853,
        state_count: 1,
        default_state: 6853,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 6854,
        state_count: 1,
        default_state: 6854,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 6855,
        state_count: 1,
        default_state: 6855,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 6856,
        state_count: 1,
        default_state: 6856,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 6857,
        state_count: 1,
        default_state: 6857,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 6858,
        state_count: 1,
        default_state: 6858,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 6859,
        state_count: 1,
        default_state: 6859,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 6860,
        state_count: 1,
        default_state: 6860,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 6861,
        state_count: 1,
        default_state: 6861,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 6862,
        state_count: 1,
        default_state: 6862,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 6863,
        state_count: 1,
        default_state: 6863,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 6864,
        state_count: 1,
        default_state: 6864,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 6865,
        state_count: 1,
        default_state: 6865,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 6866,
        state_count: 1,
        default_state: 6866,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 6867,
        state_count: 32,
        default_state: 6898,
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
        first_state: 6899,
        state_count: 32,
        default_state: 6930,
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
        first_state: 6931,
        state_count: 32,
        default_state: 6962,
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
        first_state: 6963,
        state_count: 32,
        default_state: 6994,
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
        first_state: 6995,
        state_count: 32,
        default_state: 7026,
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
        first_state: 7027,
        state_count: 32,
        default_state: 7058,
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
        first_state: 7059,
        state_count: 32,
        default_state: 7090,
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
        first_state: 7091,
        state_count: 32,
        default_state: 7122,
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
        first_state: 7123,
        state_count: 32,
        default_state: 7154,
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
        first_state: 7155,
        state_count: 32,
        default_state: 7186,
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
        first_state: 7187,
        state_count: 32,
        default_state: 7218,
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
        first_state: 7219,
        state_count: 32,
        default_state: 7250,
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
        first_state: 7251,
        state_count: 32,
        default_state: 7282,
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
        first_state: 7283,
        state_count: 32,
        default_state: 7314,
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
        first_state: 7315,
        state_count: 32,
        default_state: 7346,
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
        first_state: 7347,
        state_count: 32,
        default_state: 7378,
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
        first_state: 7379,
        state_count: 80,
        default_state: 7390,
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
        first_state: 7459,
        state_count: 80,
        default_state: 7470,
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
        first_state: 7539,
        state_count: 1,
        default_state: 7539,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 7540,
        state_count: 1,
        default_state: 7540,
        props: &[],
    },
    BlockInfo {
        name: "iron_trapdoor",
        first_state: 7541,
        state_count: 64,
        default_state: 7556,
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
        first_state: 7605,
        state_count: 1,
        default_state: 7605,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 7606,
        state_count: 1,
        default_state: 7606,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 7607,
        state_count: 1,
        default_state: 7607,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 7608,
        state_count: 80,
        default_state: 7619,
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
        first_state: 7688,
        state_count: 80,
        default_state: 7699,
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
        first_state: 7768,
        state_count: 80,
        default_state: 7779,
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
        first_state: 7848,
        state_count: 6,
        default_state: 7851,
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
        first_state: 7854,
        state_count: 6,
        default_state: 7857,
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
        first_state: 7860,
        state_count: 6,
        default_state: 7863,
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
        first_state: 7866,
        state_count: 1,
        default_state: 7866,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 7867,
        state_count: 3,
        default_state: 7868,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 7870,
        state_count: 1,
        default_state: 7870,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 7871,
        state_count: 1,
        default_state: 7871,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 7872,
        state_count: 1,
        default_state: 7872,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 7873,
        state_count: 1,
        default_state: 7873,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 7874,
        state_count: 1,
        default_state: 7874,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 7875,
        state_count: 1,
        default_state: 7875,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 7876,
        state_count: 1,
        default_state: 7876,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 7877,
        state_count: 1,
        default_state: 7877,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 7878,
        state_count: 1,
        default_state: 7878,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 7879,
        state_count: 1,
        default_state: 7879,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 7880,
        state_count: 1,
        default_state: 7880,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 7881,
        state_count: 1,
        default_state: 7881,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 7882,
        state_count: 1,
        default_state: 7882,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 7883,
        state_count: 1,
        default_state: 7883,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 7884,
        state_count: 1,
        default_state: 7884,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 7885,
        state_count: 1,
        default_state: 7885,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 7886,
        state_count: 1,
        default_state: 7886,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 7887,
        state_count: 1,
        default_state: 7887,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 7888,
        state_count: 1,
        default_state: 7888,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 7889,
        state_count: 2,
        default_state: 7890,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 7891,
        state_count: 2,
        default_state: 7892,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 7893,
        state_count: 2,
        default_state: 7894,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 7895,
        state_count: 2,
        default_state: 7896,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 7897,
        state_count: 2,
        default_state: 7898,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 7899,
        state_count: 2,
        default_state: 7900,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 7901,
        state_count: 16,
        default_state: 7901,
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
        first_state: 7917,
        state_count: 16,
        default_state: 7917,
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
        first_state: 7933,
        state_count: 16,
        default_state: 7933,
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
        first_state: 7949,
        state_count: 16,
        default_state: 7949,
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
        first_state: 7965,
        state_count: 16,
        default_state: 7965,
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
        first_state: 7981,
        state_count: 16,
        default_state: 7981,
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
        first_state: 7997,
        state_count: 16,
        default_state: 7997,
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
        first_state: 8013,
        state_count: 16,
        default_state: 8013,
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
        first_state: 8029,
        state_count: 16,
        default_state: 8029,
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
        first_state: 8045,
        state_count: 16,
        default_state: 8045,
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
        first_state: 8061,
        state_count: 16,
        default_state: 8061,
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
        first_state: 8077,
        state_count: 16,
        default_state: 8077,
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
        first_state: 8093,
        state_count: 16,
        default_state: 8093,
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
        first_state: 8109,
        state_count: 16,
        default_state: 8109,
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
        first_state: 8125,
        state_count: 16,
        default_state: 8125,
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
        first_state: 8141,
        state_count: 16,
        default_state: 8141,
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
        first_state: 8157,
        state_count: 4,
        default_state: 8157,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 8161,
        state_count: 4,
        default_state: 8161,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 8165,
        state_count: 4,
        default_state: 8165,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 8169,
        state_count: 4,
        default_state: 8169,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 8173,
        state_count: 4,
        default_state: 8173,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 8177,
        state_count: 4,
        default_state: 8177,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 8181,
        state_count: 4,
        default_state: 8181,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 8185,
        state_count: 4,
        default_state: 8185,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 8189,
        state_count: 4,
        default_state: 8189,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 8193,
        state_count: 4,
        default_state: 8193,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 8197,
        state_count: 4,
        default_state: 8197,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 8201,
        state_count: 4,
        default_state: 8201,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 8205,
        state_count: 4,
        default_state: 8205,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 8209,
        state_count: 4,
        default_state: 8209,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 8213,
        state_count: 4,
        default_state: 8213,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 8217,
        state_count: 4,
        default_state: 8217,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 8221,
        state_count: 1,
        default_state: 8221,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 8222,
        state_count: 1,
        default_state: 8222,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 8223,
        state_count: 1,
        default_state: 8223,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 8224,
        state_count: 80,
        default_state: 8235,
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
        first_state: 8304,
        state_count: 6,
        default_state: 8307,
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
        first_state: 8310,
        state_count: 6,
        default_state: 8313,
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
        first_state: 8316,
        state_count: 6,
        default_state: 8319,
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
        first_state: 8322,
        state_count: 6,
        default_state: 8325,
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
        first_state: 8328,
        state_count: 6,
        default_state: 8331,
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
        first_state: 8334,
        state_count: 6,
        default_state: 8337,
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
        first_state: 8340,
        state_count: 6,
        default_state: 8343,
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
        first_state: 8346,
        state_count: 6,
        default_state: 8349,
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
        first_state: 8352,
        state_count: 6,
        default_state: 8355,
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
        first_state: 8358,
        state_count: 6,
        default_state: 8361,
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
        first_state: 8364,
        state_count: 6,
        default_state: 8367,
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
        first_state: 8370,
        state_count: 6,
        default_state: 8373,
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
        first_state: 8376,
        state_count: 6,
        default_state: 8379,
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
        first_state: 8382,
        state_count: 6,
        default_state: 8385,
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
        first_state: 8388,
        state_count: 6,
        default_state: 8391,
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
        first_state: 8394,
        state_count: 6,
        default_state: 8397,
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
        first_state: 8400,
        state_count: 6,
        default_state: 8403,
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
        first_state: 8406,
        state_count: 6,
        default_state: 8409,
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
        first_state: 8412,
        state_count: 6,
        default_state: 8415,
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
        first_state: 8418,
        state_count: 1,
        default_state: 8418,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 8419,
        state_count: 1,
        default_state: 8419,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 8420,
        state_count: 1,
        default_state: 8420,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 8421,
        state_count: 1,
        default_state: 8421,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 8422,
        state_count: 32,
        default_state: 8429,
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
        first_state: 8454,
        state_count: 32,
        default_state: 8461,
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
        first_state: 8486,
        state_count: 32,
        default_state: 8493,
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
        first_state: 8518,
        state_count: 32,
        default_state: 8525,
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
        first_state: 8550,
        state_count: 32,
        default_state: 8557,
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
        first_state: 8582,
        state_count: 32,
        default_state: 8613,
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
        first_state: 8614,
        state_count: 32,
        default_state: 8645,
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
        first_state: 8646,
        state_count: 32,
        default_state: 8677,
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
        first_state: 8678,
        state_count: 32,
        default_state: 8709,
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
        first_state: 8710,
        state_count: 32,
        default_state: 8741,
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
        first_state: 8742,
        state_count: 64,
        default_state: 8753,
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
        first_state: 8806,
        state_count: 64,
        default_state: 8817,
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
        first_state: 8870,
        state_count: 64,
        default_state: 8881,
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
        first_state: 8934,
        state_count: 64,
        default_state: 8945,
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
        first_state: 8998,
        state_count: 64,
        default_state: 9009,
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
        first_state: 9062,
        state_count: 6,
        default_state: 9066,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 9068,
        state_count: 64,
        default_state: 9131,
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
        first_state: 9132,
        state_count: 6,
        default_state: 9132,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 9138,
        state_count: 1,
        default_state: 9138,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 9139,
        state_count: 3,
        default_state: 9140,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 9142,
        state_count: 80,
        default_state: 9153,
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
        first_state: 9222,
        state_count: 1,
        default_state: 9222,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 9223,
        state_count: 4,
        default_state: 9223,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "grass_path",
        first_state: 9227,
        state_count: 1,
        default_state: 9227,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 9228,
        state_count: 1,
        default_state: 9228,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 9229,
        state_count: 12,
        default_state: 9235,
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
        first_state: 9241,
        state_count: 12,
        default_state: 9247,
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
        first_state: 9253,
        state_count: 4,
        default_state: 9253,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 9257,
        state_count: 1,
        default_state: 9257,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 9258,
        state_count: 1,
        default_state: 9258,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 9259,
        state_count: 1,
        default_state: 9259,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 9260,
        state_count: 3,
        default_state: 9261,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 9263,
        state_count: 1,
        default_state: 9263,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 9264,
        state_count: 12,
        default_state: 9269,
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
        first_state: 9276,
        state_count: 6,
        default_state: 9280,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 9282,
        state_count: 6,
        default_state: 9286,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 9288,
        state_count: 6,
        default_state: 9292,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 9294,
        state_count: 6,
        default_state: 9298,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 9300,
        state_count: 6,
        default_state: 9304,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 9306,
        state_count: 6,
        default_state: 9310,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 9312,
        state_count: 6,
        default_state: 9316,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 9318,
        state_count: 6,
        default_state: 9322,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 9324,
        state_count: 6,
        default_state: 9328,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 9330,
        state_count: 6,
        default_state: 9334,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 9336,
        state_count: 6,
        default_state: 9340,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 9342,
        state_count: 6,
        default_state: 9346,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 9348,
        state_count: 6,
        default_state: 9352,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 9354,
        state_count: 6,
        default_state: 9358,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 9360,
        state_count: 6,
        default_state: 9364,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 9366,
        state_count: 6,
        default_state: 9370,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 9372,
        state_count: 6,
        default_state: 9376,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 9378,
        state_count: 4,
        default_state: 9378,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 9382,
        state_count: 4,
        default_state: 9382,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 9386,
        state_count: 4,
        default_state: 9386,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 9390,
        state_count: 4,
        default_state: 9390,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 9394,
        state_count: 4,
        default_state: 9394,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 9398,
        state_count: 4,
        default_state: 9398,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 9402,
        state_count: 4,
        default_state: 9402,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 9406,
        state_count: 4,
        default_state: 9406,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 9410,
        state_count: 4,
        default_state: 9410,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 9414,
        state_count: 4,
        default_state: 9414,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 9418,
        state_count: 4,
        default_state: 9418,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 9422,
        state_count: 4,
        default_state: 9422,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 9426,
        state_count: 4,
        default_state: 9426,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 9430,
        state_count: 4,
        default_state: 9430,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 9434,
        state_count: 4,
        default_state: 9434,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 9438,
        state_count: 4,
        default_state: 9438,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 9442,
        state_count: 1,
        default_state: 9442,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 9443,
        state_count: 1,
        default_state: 9443,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 9444,
        state_count: 1,
        default_state: 9444,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 9445,
        state_count: 1,
        default_state: 9445,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 9446,
        state_count: 1,
        default_state: 9446,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 9447,
        state_count: 1,
        default_state: 9447,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 9448,
        state_count: 1,
        default_state: 9448,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 9449,
        state_count: 1,
        default_state: 9449,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 9450,
        state_count: 1,
        default_state: 9450,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 9451,
        state_count: 1,
        default_state: 9451,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 9452,
        state_count: 1,
        default_state: 9452,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 9453,
        state_count: 1,
        default_state: 9453,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 9454,
        state_count: 1,
        default_state: 9454,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 9455,
        state_count: 1,
        default_state: 9455,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 9456,
        state_count: 1,
        default_state: 9456,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 9457,
        state_count: 1,
        default_state: 9457,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 9458,
        state_count: 1,
        default_state: 9458,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 9459,
        state_count: 1,
        default_state: 9459,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 9460,
        state_count: 1,
        default_state: 9460,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 9461,
        state_count: 1,
        default_state: 9461,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 9462,
        state_count: 1,
        default_state: 9462,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 9463,
        state_count: 1,
        default_state: 9463,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 9464,
        state_count: 1,
        default_state: 9464,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 9465,
        state_count: 1,
        default_state: 9465,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 9466,
        state_count: 1,
        default_state: 9466,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 9467,
        state_count: 1,
        default_state: 9467,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 9468,
        state_count: 1,
        default_state: 9468,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 9469,
        state_count: 1,
        default_state: 9469,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 9470,
        state_count: 1,
        default_state: 9470,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 9471,
        state_count: 1,
        default_state: 9471,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 9472,
        state_count: 1,
        default_state: 9472,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 9473,
        state_count: 1,
        default_state: 9473,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 9474,
        state_count: 26,
        default_state: 9474,
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
        first_state: 9500,
        state_count: 1,
        default_state: 9500,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 9501,
        state_count: 1,
        default_state: 9501,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 9502,
        state_count: 12,
        default_state: 9502,
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
        first_state: 9514,
        state_count: 1,
        default_state: 9514,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 9515,
        state_count: 1,
        default_state: 9515,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 9516,
        state_count: 1,
        default_state: 9516,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 9517,
        state_count: 1,
        default_state: 9517,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 9518,
        state_count: 1,
        default_state: 9518,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 9519,
        state_count: 1,
        default_state: 9519,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 9520,
        state_count: 1,
        default_state: 9520,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 9521,
        state_count: 1,
        default_state: 9521,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 9522,
        state_count: 1,
        default_state: 9522,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 9523,
        state_count: 1,
        default_state: 9523,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 9524,
        state_count: 2,
        default_state: 9524,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 9526,
        state_count: 2,
        default_state: 9526,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 9528,
        state_count: 2,
        default_state: 9528,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 9530,
        state_count: 2,
        default_state: 9530,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 9532,
        state_count: 2,
        default_state: 9532,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 9534,
        state_count: 2,
        default_state: 9534,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 9536,
        state_count: 2,
        default_state: 9536,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 9538,
        state_count: 2,
        default_state: 9538,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 9540,
        state_count: 2,
        default_state: 9540,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 9542,
        state_count: 2,
        default_state: 9542,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 9544,
        state_count: 2,
        default_state: 9544,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 9546,
        state_count: 2,
        default_state: 9546,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 9548,
        state_count: 2,
        default_state: 9548,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 9550,
        state_count: 2,
        default_state: 9550,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 9552,
        state_count: 2,
        default_state: 9552,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 9554,
        state_count: 2,
        default_state: 9554,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 9556,
        state_count: 2,
        default_state: 9556,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 9558,
        state_count: 2,
        default_state: 9558,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 9560,
        state_count: 2,
        default_state: 9560,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 9562,
        state_count: 2,
        default_state: 9562,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 9564,
        state_count: 8,
        default_state: 9564,
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
        first_state: 9572,
        state_count: 8,
        default_state: 9572,
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
        first_state: 9580,
        state_count: 8,
        default_state: 9580,
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
        first_state: 9588,
        state_count: 8,
        default_state: 9588,
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
        first_state: 9596,
        state_count: 8,
        default_state: 9596,
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
        first_state: 9604,
        state_count: 8,
        default_state: 9604,
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
        first_state: 9612,
        state_count: 8,
        default_state: 9612,
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
        first_state: 9620,
        state_count: 8,
        default_state: 9620,
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
        first_state: 9628,
        state_count: 8,
        default_state: 9628,
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
        first_state: 9636,
        state_count: 8,
        default_state: 9636,
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
        first_state: 9644,
        state_count: 8,
        default_state: 9644,
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
        first_state: 9652,
        state_count: 1,
        default_state: 9652,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 9653,
        state_count: 2,
        default_state: 9653,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 9655,
        state_count: 1,
        default_state: 9655,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 9656,
        state_count: 12,
        default_state: 9656,
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
        first_state: 9668,
        state_count: 1,
        default_state: 9668,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 9669,
        state_count: 1,
        default_state: 9669,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 9670,
        state_count: 1,
        default_state: 9670,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 9671,
        state_count: 2,
        default_state: 9671,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 9673,
        state_count: 80,
        default_state: 9684,
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
        first_state: 9753,
        state_count: 80,
        default_state: 9764,
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
        first_state: 9833,
        state_count: 80,
        default_state: 9844,
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
        first_state: 9913,
        state_count: 80,
        default_state: 9924,
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
        first_state: 9993,
        state_count: 80,
        default_state: 10004,
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
        first_state: 10073,
        state_count: 80,
        default_state: 10084,
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
        first_state: 10153,
        state_count: 80,
        default_state: 10164,
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
        first_state: 10233,
        state_count: 80,
        default_state: 10244,
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
        first_state: 10313,
        state_count: 80,
        default_state: 10324,
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
        first_state: 10393,
        state_count: 80,
        default_state: 10404,
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
        first_state: 10473,
        state_count: 80,
        default_state: 10484,
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
        first_state: 10553,
        state_count: 80,
        default_state: 10564,
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
        first_state: 10633,
        state_count: 80,
        default_state: 10644,
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
        first_state: 10713,
        state_count: 80,
        default_state: 10724,
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
        name: "smooth_red_sandstone_slab",
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
        name: "mossy_stone_brick_slab",
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
        name: "polished_diorite_slab",
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
        name: "mossy_cobblestone_slab",
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
        name: "end_stone_brick_slab",
        first_state: 10823,
        state_count: 6,
        default_state: 10826,
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
        first_state: 10829,
        state_count: 6,
        default_state: 10832,
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
        first_state: 10835,
        state_count: 6,
        default_state: 10838,
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
        first_state: 10841,
        state_count: 6,
        default_state: 10844,
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
        first_state: 10847,
        state_count: 6,
        default_state: 10850,
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
        first_state: 10853,
        state_count: 6,
        default_state: 10856,
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
        first_state: 10859,
        state_count: 6,
        default_state: 10862,
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
        first_state: 10865,
        state_count: 6,
        default_state: 10868,
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
        first_state: 10871,
        state_count: 324,
        default_state: 10874,
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
        first_state: 11195,
        state_count: 324,
        default_state: 11198,
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
        first_state: 11519,
        state_count: 324,
        default_state: 11522,
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
        first_state: 11843,
        state_count: 324,
        default_state: 11846,
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
        first_state: 12167,
        state_count: 324,
        default_state: 12170,
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
        first_state: 12491,
        state_count: 324,
        default_state: 12494,
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
        first_state: 12815,
        state_count: 324,
        default_state: 12818,
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
        first_state: 13139,
        state_count: 324,
        default_state: 13142,
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
        first_state: 13463,
        state_count: 324,
        default_state: 13466,
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
        first_state: 13787,
        state_count: 324,
        default_state: 13790,
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
        first_state: 14111,
        state_count: 324,
        default_state: 14114,
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
        first_state: 14435,
        state_count: 324,
        default_state: 14438,
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
        first_state: 14759,
        state_count: 32,
        default_state: 14790,
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
        first_state: 14791,
        state_count: 4,
        default_state: 14791,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 14795,
        state_count: 12,
        default_state: 14796,
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
        first_state: 14807,
        state_count: 8,
        default_state: 14808,
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
        first_state: 14815,
        state_count: 8,
        default_state: 14816,
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
        first_state: 14823,
        state_count: 1,
        default_state: 14823,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 14824,
        state_count: 1,
        default_state: 14824,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 14825,
        state_count: 12,
        default_state: 14829,
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
        first_state: 14837,
        state_count: 16,
        default_state: 14840,
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
        first_state: 14853,
        state_count: 1,
        default_state: 14853,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 14854,
        state_count: 4,
        default_state: 14854,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 14858,
        state_count: 32,
        default_state: 14859,
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
        first_state: 14890,
        state_count: 4,
        default_state: 14893,
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
        first_state: 14894,
        state_count: 4,
        default_state: 14897,
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
        first_state: 14898,
        state_count: 32,
        default_state: 14901,
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
        first_state: 14930,
        state_count: 32,
        default_state: 14933,
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
        first_state: 14962,
        state_count: 4,
        default_state: 14962,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 14966,
        state_count: 3,
        default_state: 14967,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 14969,
        state_count: 3,
        default_state: 14970,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 14972,
        state_count: 3,
        default_state: 14973,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 14975,
        state_count: 3,
        default_state: 14976,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 14978,
        state_count: 1,
        default_state: 14978,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 14979,
        state_count: 1,
        default_state: 14979,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 14980,
        state_count: 1,
        default_state: 14980,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 14981,
        state_count: 1,
        default_state: 14981,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 14982,
        state_count: 1,
        default_state: 14982,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 14983,
        state_count: 3,
        default_state: 14984,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 14986,
        state_count: 3,
        default_state: 14987,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 14989,
        state_count: 3,
        default_state: 14990,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 14992,
        state_count: 3,
        default_state: 14993,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 14995,
        state_count: 1,
        default_state: 14995,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 14996,
        state_count: 1,
        default_state: 14996,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 14997,
        state_count: 1,
        default_state: 14997,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 14998,
        state_count: 26,
        default_state: 14998,
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
        first_state: 15024,
        state_count: 1,
        default_state: 15024,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 15025,
        state_count: 26,
        default_state: 15025,
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
        first_state: 15051,
        state_count: 1,
        default_state: 15051,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 15052,
        state_count: 1,
        default_state: 15052,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 15053,
        state_count: 1,
        default_state: 15053,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 15054,
        state_count: 1,
        default_state: 15054,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 15055,
        state_count: 6,
        default_state: 15058,
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
        first_state: 15061,
        state_count: 6,
        default_state: 15064,
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
        first_state: 15067,
        state_count: 2,
        default_state: 15068,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 15069,
        state_count: 2,
        default_state: 15070,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 15071,
        state_count: 32,
        default_state: 15102,
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
        first_state: 15103,
        state_count: 32,
        default_state: 15134,
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
        first_state: 15135,
        state_count: 64,
        default_state: 15150,
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
        first_state: 15199,
        state_count: 64,
        default_state: 15214,
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
        first_state: 15263,
        state_count: 32,
        default_state: 15270,
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
        first_state: 15295,
        state_count: 32,
        default_state: 15302,
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
        first_state: 15327,
        state_count: 80,
        default_state: 15338,
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
        first_state: 15407,
        state_count: 80,
        default_state: 15418,
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
        first_state: 15487,
        state_count: 24,
        default_state: 15496,
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
        first_state: 15511,
        state_count: 24,
        default_state: 15520,
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
        first_state: 15535,
        state_count: 64,
        default_state: 15546,
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
        first_state: 15599,
        state_count: 64,
        default_state: 15610,
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
        first_state: 15663,
        state_count: 32,
        default_state: 15664,
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
        first_state: 15695,
        state_count: 32,
        default_state: 15696,
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
        first_state: 15727,
        state_count: 8,
        default_state: 15728,
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
        first_state: 15735,
        state_count: 8,
        default_state: 15736,
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
        first_state: 15743,
        state_count: 4,
        default_state: 15743,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 15747,
        state_count: 12,
        default_state: 15757,
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
        first_state: 15759,
        state_count: 9,
        default_state: 15759,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 15768,
        state_count: 16,
        default_state: 15768,
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
        first_state: 15784,
        state_count: 24,
        default_state: 15784,
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
        first_state: 15808,
        state_count: 24,
        default_state: 15808,
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
        first_state: 15832,
        state_count: 1,
        default_state: 15832,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 15833,
        state_count: 1,
        default_state: 15833,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 15834,
        state_count: 1,
        default_state: 15834,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 15835,
        state_count: 1,
        default_state: 15835,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 15836,
        state_count: 1,
        default_state: 15836,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 15837,
        state_count: 5,
        default_state: 15837,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 15842,
        state_count: 1,
        default_state: 15842,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 15843,
        state_count: 1,
        default_state: 15843,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 15844,
        state_count: 1,
        default_state: 15844,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 15845,
        state_count: 1,
        default_state: 15845,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 15846,
        state_count: 1,
        default_state: 15846,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 15847,
        state_count: 1,
        default_state: 15847,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 15848,
        state_count: 80,
        default_state: 15859,
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
        first_state: 15928,
        state_count: 324,
        default_state: 15931,
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
        first_state: 16252,
        state_count: 6,
        default_state: 16255,
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
        first_state: 16258,
        state_count: 1,
        default_state: 16258,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 16259,
        state_count: 1,
        default_state: 16259,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 16260,
        state_count: 1,
        default_state: 16260,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 16261,
        state_count: 1,
        default_state: 16261,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
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
        name: "polished_blackstone_brick_stairs",
        first_state: 16268,
        state_count: 80,
        default_state: 16279,
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
        first_state: 16348,
        state_count: 324,
        default_state: 16351,
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
        first_state: 16672,
        state_count: 1,
        default_state: 16672,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 16673,
        state_count: 80,
        default_state: 16684,
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
        first_state: 16753,
        state_count: 6,
        default_state: 16756,
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
        first_state: 16759,
        state_count: 2,
        default_state: 16760,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 16761,
        state_count: 24,
        default_state: 16770,
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
        first_state: 16785,
        state_count: 324,
        default_state: 16788,
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
        first_state: 17109,
        state_count: 1,
        default_state: 17109,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 17110,
        state_count: 1,
        default_state: 17110,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 17111,
        state_count: 1,
        default_state: 17111,
        props: &[],
    },
];
