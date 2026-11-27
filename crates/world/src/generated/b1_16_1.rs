// @generated `cargo xtask gen-blockstates 1.16.1` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.16.1";
pub const STATE_COUNT: u32 = 17104;

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
        state_count: 2,
        default_state: 4730,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "glass_pane",
        first_state: 4731,
        state_count: 32,
        default_state: 4762,
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
        first_state: 4763,
        state_count: 1,
        default_state: 4763,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 4764,
        state_count: 4,
        default_state: 4764,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 4768,
        state_count: 4,
        default_state: 4768,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 4772,
        state_count: 8,
        default_state: 4772,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 4780,
        state_count: 8,
        default_state: 4780,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 4788,
        state_count: 32,
        default_state: 4819,
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
        first_state: 4820,
        state_count: 32,
        default_state: 4827,
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
        first_state: 4852,
        state_count: 80,
        default_state: 4863,
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
        first_state: 4932,
        state_count: 80,
        default_state: 4943,
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
        first_state: 5012,
        state_count: 2,
        default_state: 5013,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 5014,
        state_count: 1,
        default_state: 5014,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 5015,
        state_count: 1,
        default_state: 5015,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 5016,
        state_count: 32,
        default_state: 5047,
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
        first_state: 5048,
        state_count: 80,
        default_state: 5059,
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
        first_state: 5128,
        state_count: 4,
        default_state: 5128,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 5132,
        state_count: 1,
        default_state: 5132,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 5133,
        state_count: 8,
        default_state: 5140,
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
        first_state: 5141,
        state_count: 4,
        default_state: 5141,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 5145,
        state_count: 1,
        default_state: 5145,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 5146,
        state_count: 8,
        default_state: 5150,
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
        first_state: 5154,
        state_count: 1,
        default_state: 5154,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 5155,
        state_count: 1,
        default_state: 5155,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 5156,
        state_count: 2,
        default_state: 5157,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 5158,
        state_count: 12,
        default_state: 5158,
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
        first_state: 5170,
        state_count: 80,
        default_state: 5181,
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
        first_state: 5250,
        state_count: 1,
        default_state: 5250,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 5251,
        state_count: 8,
        default_state: 5252,
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
        first_state: 5259,
        state_count: 16,
        default_state: 5268,
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
        first_state: 5275,
        state_count: 128,
        default_state: 5402,
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
        first_state: 5403,
        state_count: 1,
        default_state: 5403,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 5404,
        state_count: 80,
        default_state: 5415,
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
        first_state: 5484,
        state_count: 80,
        default_state: 5495,
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
        first_state: 5564,
        state_count: 80,
        default_state: 5575,
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
        first_state: 5644,
        state_count: 12,
        default_state: 5650,
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
        first_state: 5656,
        state_count: 1,
        default_state: 5656,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 5657,
        state_count: 324,
        default_state: 5660,
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
        first_state: 5981,
        state_count: 324,
        default_state: 5984,
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
        first_state: 6305,
        state_count: 1,
        default_state: 6305,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 6306,
        state_count: 1,
        default_state: 6306,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 6307,
        state_count: 1,
        default_state: 6307,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 6308,
        state_count: 1,
        default_state: 6308,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 6309,
        state_count: 1,
        default_state: 6309,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 6310,
        state_count: 1,
        default_state: 6310,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 6311,
        state_count: 1,
        default_state: 6311,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 6312,
        state_count: 1,
        default_state: 6312,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 6313,
        state_count: 1,
        default_state: 6313,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 6314,
        state_count: 1,
        default_state: 6314,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 6315,
        state_count: 1,
        default_state: 6315,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 6316,
        state_count: 1,
        default_state: 6316,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 6317,
        state_count: 1,
        default_state: 6317,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 6318,
        state_count: 1,
        default_state: 6318,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 6319,
        state_count: 1,
        default_state: 6319,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 6320,
        state_count: 1,
        default_state: 6320,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 6321,
        state_count: 1,
        default_state: 6321,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 6322,
        state_count: 1,
        default_state: 6322,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 6323,
        state_count: 1,
        default_state: 6323,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 6324,
        state_count: 1,
        default_state: 6324,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 6325,
        state_count: 1,
        default_state: 6325,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 6326,
        state_count: 1,
        default_state: 6326,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 6327,
        state_count: 1,
        default_state: 6327,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 6328,
        state_count: 1,
        default_state: 6328,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 6329,
        state_count: 1,
        default_state: 6329,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 6330,
        state_count: 8,
        default_state: 6330,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 6338,
        state_count: 8,
        default_state: 6338,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 6346,
        state_count: 24,
        default_state: 6355,
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
        first_state: 6370,
        state_count: 24,
        default_state: 6379,
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
        first_state: 6394,
        state_count: 24,
        default_state: 6403,
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
        first_state: 6418,
        state_count: 24,
        default_state: 6427,
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
        first_state: 6442,
        state_count: 24,
        default_state: 6451,
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
        first_state: 6466,
        state_count: 24,
        default_state: 6475,
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
        first_state: 6490,
        state_count: 16,
        default_state: 6490,
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
        first_state: 6506,
        state_count: 4,
        default_state: 6506,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 6510,
        state_count: 16,
        default_state: 6510,
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
        first_state: 6526,
        state_count: 4,
        default_state: 6526,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 6530,
        state_count: 16,
        default_state: 6530,
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
        first_state: 6546,
        state_count: 4,
        default_state: 6546,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 6550,
        state_count: 16,
        default_state: 6550,
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
        first_state: 6566,
        state_count: 4,
        default_state: 6566,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 6570,
        state_count: 16,
        default_state: 6570,
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
        first_state: 6586,
        state_count: 4,
        default_state: 6586,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 6590,
        state_count: 16,
        default_state: 6590,
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
        first_state: 6606,
        state_count: 4,
        default_state: 6606,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 6610,
        state_count: 4,
        default_state: 6610,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 6614,
        state_count: 4,
        default_state: 6614,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 6618,
        state_count: 4,
        default_state: 6618,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 6622,
        state_count: 24,
        default_state: 6623,
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
        first_state: 6646,
        state_count: 16,
        default_state: 6646,
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
        first_state: 6662,
        state_count: 16,
        default_state: 6662,
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
        first_state: 6678,
        state_count: 16,
        default_state: 6679,
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
        first_state: 6694,
        state_count: 32,
        default_state: 6710,
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
        first_state: 6726,
        state_count: 1,
        default_state: 6726,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 6727,
        state_count: 1,
        default_state: 6727,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 6728,
        state_count: 10,
        default_state: 6728,
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
        first_state: 6738,
        state_count: 1,
        default_state: 6738,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 6739,
        state_count: 1,
        default_state: 6739,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 6740,
        state_count: 3,
        default_state: 6741,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 6743,
        state_count: 80,
        default_state: 6754,
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
        first_state: 6823,
        state_count: 12,
        default_state: 6829,
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
        first_state: 6835,
        state_count: 12,
        default_state: 6836,
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
        first_state: 6847,
        state_count: 1,
        default_state: 6847,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 6848,
        state_count: 1,
        default_state: 6848,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 6849,
        state_count: 1,
        default_state: 6849,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 6850,
        state_count: 1,
        default_state: 6850,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 6851,
        state_count: 1,
        default_state: 6851,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 6852,
        state_count: 1,
        default_state: 6852,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 6853,
        state_count: 1,
        default_state: 6853,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 6854,
        state_count: 1,
        default_state: 6854,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 6855,
        state_count: 1,
        default_state: 6855,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 6856,
        state_count: 1,
        default_state: 6856,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 6857,
        state_count: 1,
        default_state: 6857,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 6858,
        state_count: 1,
        default_state: 6858,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 6859,
        state_count: 1,
        default_state: 6859,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 6860,
        state_count: 1,
        default_state: 6860,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 6861,
        state_count: 1,
        default_state: 6861,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 6862,
        state_count: 1,
        default_state: 6862,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 6863,
        state_count: 32,
        default_state: 6894,
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
        first_state: 6895,
        state_count: 32,
        default_state: 6926,
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
        first_state: 6927,
        state_count: 32,
        default_state: 6958,
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
        first_state: 6959,
        state_count: 32,
        default_state: 6990,
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
        first_state: 6991,
        state_count: 32,
        default_state: 7022,
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
        first_state: 7023,
        state_count: 32,
        default_state: 7054,
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
        first_state: 7055,
        state_count: 32,
        default_state: 7086,
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
        first_state: 7087,
        state_count: 32,
        default_state: 7118,
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
        first_state: 7119,
        state_count: 32,
        default_state: 7150,
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
        first_state: 7151,
        state_count: 32,
        default_state: 7182,
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
        first_state: 7183,
        state_count: 32,
        default_state: 7214,
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
        first_state: 7215,
        state_count: 32,
        default_state: 7246,
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
        first_state: 7247,
        state_count: 32,
        default_state: 7278,
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
        first_state: 7279,
        state_count: 32,
        default_state: 7310,
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
        first_state: 7311,
        state_count: 32,
        default_state: 7342,
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
        first_state: 7343,
        state_count: 32,
        default_state: 7374,
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
        first_state: 7375,
        state_count: 80,
        default_state: 7386,
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
        first_state: 7455,
        state_count: 80,
        default_state: 7466,
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
        first_state: 7535,
        state_count: 1,
        default_state: 7535,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 7536,
        state_count: 1,
        default_state: 7536,
        props: &[],
    },
    BlockInfo {
        name: "iron_trapdoor",
        first_state: 7537,
        state_count: 64,
        default_state: 7552,
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
        first_state: 7601,
        state_count: 1,
        default_state: 7601,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 7602,
        state_count: 1,
        default_state: 7602,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 7603,
        state_count: 1,
        default_state: 7603,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 7604,
        state_count: 80,
        default_state: 7615,
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
        first_state: 7684,
        state_count: 80,
        default_state: 7695,
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
        first_state: 7764,
        state_count: 80,
        default_state: 7775,
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
        first_state: 7844,
        state_count: 6,
        default_state: 7847,
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
        first_state: 7850,
        state_count: 6,
        default_state: 7853,
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
        first_state: 7856,
        state_count: 6,
        default_state: 7859,
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
        first_state: 7862,
        state_count: 1,
        default_state: 7862,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 7863,
        state_count: 3,
        default_state: 7864,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 7866,
        state_count: 1,
        default_state: 7866,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 7867,
        state_count: 1,
        default_state: 7867,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 7868,
        state_count: 1,
        default_state: 7868,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 7869,
        state_count: 1,
        default_state: 7869,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 7870,
        state_count: 1,
        default_state: 7870,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 7871,
        state_count: 1,
        default_state: 7871,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 7872,
        state_count: 1,
        default_state: 7872,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 7873,
        state_count: 1,
        default_state: 7873,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 7874,
        state_count: 1,
        default_state: 7874,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 7875,
        state_count: 1,
        default_state: 7875,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 7876,
        state_count: 1,
        default_state: 7876,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 7877,
        state_count: 1,
        default_state: 7877,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 7878,
        state_count: 1,
        default_state: 7878,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 7879,
        state_count: 1,
        default_state: 7879,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 7880,
        state_count: 1,
        default_state: 7880,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 7881,
        state_count: 1,
        default_state: 7881,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 7882,
        state_count: 1,
        default_state: 7882,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 7883,
        state_count: 1,
        default_state: 7883,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 7884,
        state_count: 1,
        default_state: 7884,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 7885,
        state_count: 2,
        default_state: 7886,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 7887,
        state_count: 2,
        default_state: 7888,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 7889,
        state_count: 2,
        default_state: 7890,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 7891,
        state_count: 2,
        default_state: 7892,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 7893,
        state_count: 2,
        default_state: 7894,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 7895,
        state_count: 2,
        default_state: 7896,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 7897,
        state_count: 16,
        default_state: 7897,
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
        first_state: 7913,
        state_count: 16,
        default_state: 7913,
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
        first_state: 7929,
        state_count: 16,
        default_state: 7929,
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
        first_state: 7945,
        state_count: 16,
        default_state: 7945,
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
        first_state: 7961,
        state_count: 16,
        default_state: 7961,
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
        first_state: 7977,
        state_count: 16,
        default_state: 7977,
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
        first_state: 7993,
        state_count: 16,
        default_state: 7993,
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
        first_state: 8009,
        state_count: 16,
        default_state: 8009,
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
        first_state: 8025,
        state_count: 16,
        default_state: 8025,
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
        first_state: 8041,
        state_count: 16,
        default_state: 8041,
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
        first_state: 8057,
        state_count: 16,
        default_state: 8057,
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
        first_state: 8073,
        state_count: 16,
        default_state: 8073,
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
        first_state: 8089,
        state_count: 16,
        default_state: 8089,
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
        first_state: 8105,
        state_count: 16,
        default_state: 8105,
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
        first_state: 8121,
        state_count: 16,
        default_state: 8121,
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
        first_state: 8137,
        state_count: 16,
        default_state: 8137,
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
        first_state: 8153,
        state_count: 4,
        default_state: 8153,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 8157,
        state_count: 4,
        default_state: 8157,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 8161,
        state_count: 4,
        default_state: 8161,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 8165,
        state_count: 4,
        default_state: 8165,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 8169,
        state_count: 4,
        default_state: 8169,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 8173,
        state_count: 4,
        default_state: 8173,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 8177,
        state_count: 4,
        default_state: 8177,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 8181,
        state_count: 4,
        default_state: 8181,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 8185,
        state_count: 4,
        default_state: 8185,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 8189,
        state_count: 4,
        default_state: 8189,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 8193,
        state_count: 4,
        default_state: 8193,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 8197,
        state_count: 4,
        default_state: 8197,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 8201,
        state_count: 4,
        default_state: 8201,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 8205,
        state_count: 4,
        default_state: 8205,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 8209,
        state_count: 4,
        default_state: 8209,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 8213,
        state_count: 4,
        default_state: 8213,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 8217,
        state_count: 1,
        default_state: 8217,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 8218,
        state_count: 1,
        default_state: 8218,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 8219,
        state_count: 1,
        default_state: 8219,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 8220,
        state_count: 80,
        default_state: 8231,
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
        first_state: 8300,
        state_count: 6,
        default_state: 8303,
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
        first_state: 8306,
        state_count: 6,
        default_state: 8309,
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
        first_state: 8312,
        state_count: 6,
        default_state: 8315,
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
        first_state: 8318,
        state_count: 6,
        default_state: 8321,
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
        first_state: 8324,
        state_count: 6,
        default_state: 8327,
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
        first_state: 8330,
        state_count: 6,
        default_state: 8333,
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
        first_state: 8336,
        state_count: 6,
        default_state: 8339,
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
        first_state: 8342,
        state_count: 6,
        default_state: 8345,
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
        first_state: 8348,
        state_count: 6,
        default_state: 8351,
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
        first_state: 8354,
        state_count: 6,
        default_state: 8357,
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
        first_state: 8360,
        state_count: 6,
        default_state: 8363,
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
        first_state: 8366,
        state_count: 6,
        default_state: 8369,
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
        first_state: 8372,
        state_count: 6,
        default_state: 8375,
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
        first_state: 8378,
        state_count: 6,
        default_state: 8381,
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
        first_state: 8384,
        state_count: 6,
        default_state: 8387,
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
        first_state: 8390,
        state_count: 6,
        default_state: 8393,
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
        first_state: 8396,
        state_count: 6,
        default_state: 8399,
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
        first_state: 8402,
        state_count: 6,
        default_state: 8405,
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
        first_state: 8408,
        state_count: 6,
        default_state: 8411,
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
        first_state: 8414,
        state_count: 1,
        default_state: 8414,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 8415,
        state_count: 1,
        default_state: 8415,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 8416,
        state_count: 1,
        default_state: 8416,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 8417,
        state_count: 1,
        default_state: 8417,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 8418,
        state_count: 32,
        default_state: 8425,
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
        first_state: 8450,
        state_count: 32,
        default_state: 8457,
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
        first_state: 8482,
        state_count: 32,
        default_state: 8489,
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
        first_state: 8514,
        state_count: 32,
        default_state: 8521,
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
        first_state: 8546,
        state_count: 32,
        default_state: 8553,
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
        first_state: 8578,
        state_count: 32,
        default_state: 8609,
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
        first_state: 8610,
        state_count: 32,
        default_state: 8641,
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
        first_state: 8642,
        state_count: 32,
        default_state: 8673,
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
        first_state: 8674,
        state_count: 32,
        default_state: 8705,
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
        first_state: 8706,
        state_count: 32,
        default_state: 8737,
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
        first_state: 8738,
        state_count: 64,
        default_state: 8749,
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
        first_state: 8802,
        state_count: 64,
        default_state: 8813,
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
        first_state: 8866,
        state_count: 64,
        default_state: 8877,
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
        first_state: 8930,
        state_count: 64,
        default_state: 8941,
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
        first_state: 8994,
        state_count: 64,
        default_state: 9005,
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
        first_state: 9058,
        state_count: 6,
        default_state: 9062,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 9064,
        state_count: 64,
        default_state: 9127,
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
        first_state: 9128,
        state_count: 6,
        default_state: 9128,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 9134,
        state_count: 1,
        default_state: 9134,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 9135,
        state_count: 3,
        default_state: 9136,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 9138,
        state_count: 80,
        default_state: 9149,
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
        first_state: 9218,
        state_count: 1,
        default_state: 9218,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 9219,
        state_count: 4,
        default_state: 9219,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "grass_path",
        first_state: 9223,
        state_count: 1,
        default_state: 9223,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 9224,
        state_count: 1,
        default_state: 9224,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 9225,
        state_count: 12,
        default_state: 9231,
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
        first_state: 9237,
        state_count: 12,
        default_state: 9243,
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
        first_state: 9249,
        state_count: 4,
        default_state: 9249,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 9253,
        state_count: 1,
        default_state: 9253,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 9254,
        state_count: 1,
        default_state: 9254,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 9255,
        state_count: 1,
        default_state: 9255,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 9256,
        state_count: 3,
        default_state: 9257,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 9259,
        state_count: 1,
        default_state: 9259,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 9260,
        state_count: 12,
        default_state: 9265,
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
        first_state: 9272,
        state_count: 6,
        default_state: 9276,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 9278,
        state_count: 6,
        default_state: 9282,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 9284,
        state_count: 6,
        default_state: 9288,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 9290,
        state_count: 6,
        default_state: 9294,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 9296,
        state_count: 6,
        default_state: 9300,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 9302,
        state_count: 6,
        default_state: 9306,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 9308,
        state_count: 6,
        default_state: 9312,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 9314,
        state_count: 6,
        default_state: 9318,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 9320,
        state_count: 6,
        default_state: 9324,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 9326,
        state_count: 6,
        default_state: 9330,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 9332,
        state_count: 6,
        default_state: 9336,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 9338,
        state_count: 6,
        default_state: 9342,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 9344,
        state_count: 6,
        default_state: 9348,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 9350,
        state_count: 6,
        default_state: 9354,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 9356,
        state_count: 6,
        default_state: 9360,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 9362,
        state_count: 6,
        default_state: 9366,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 9368,
        state_count: 6,
        default_state: 9372,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 9374,
        state_count: 4,
        default_state: 9374,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 9378,
        state_count: 4,
        default_state: 9378,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 9382,
        state_count: 4,
        default_state: 9382,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 9386,
        state_count: 4,
        default_state: 9386,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 9390,
        state_count: 4,
        default_state: 9390,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 9394,
        state_count: 4,
        default_state: 9394,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 9398,
        state_count: 4,
        default_state: 9398,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 9402,
        state_count: 4,
        default_state: 9402,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 9406,
        state_count: 4,
        default_state: 9406,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 9410,
        state_count: 4,
        default_state: 9410,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 9414,
        state_count: 4,
        default_state: 9414,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 9418,
        state_count: 4,
        default_state: 9418,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 9422,
        state_count: 4,
        default_state: 9422,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 9426,
        state_count: 4,
        default_state: 9426,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 9430,
        state_count: 4,
        default_state: 9430,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 9434,
        state_count: 4,
        default_state: 9434,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 9438,
        state_count: 1,
        default_state: 9438,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 9439,
        state_count: 1,
        default_state: 9439,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 9440,
        state_count: 1,
        default_state: 9440,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 9441,
        state_count: 1,
        default_state: 9441,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 9442,
        state_count: 1,
        default_state: 9442,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 9443,
        state_count: 1,
        default_state: 9443,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 9444,
        state_count: 1,
        default_state: 9444,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 9445,
        state_count: 1,
        default_state: 9445,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 9446,
        state_count: 1,
        default_state: 9446,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 9447,
        state_count: 1,
        default_state: 9447,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 9448,
        state_count: 1,
        default_state: 9448,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 9449,
        state_count: 1,
        default_state: 9449,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 9450,
        state_count: 1,
        default_state: 9450,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 9451,
        state_count: 1,
        default_state: 9451,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 9452,
        state_count: 1,
        default_state: 9452,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 9453,
        state_count: 1,
        default_state: 9453,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 9454,
        state_count: 1,
        default_state: 9454,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 9455,
        state_count: 1,
        default_state: 9455,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 9456,
        state_count: 1,
        default_state: 9456,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 9457,
        state_count: 1,
        default_state: 9457,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 9458,
        state_count: 1,
        default_state: 9458,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 9459,
        state_count: 1,
        default_state: 9459,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 9460,
        state_count: 1,
        default_state: 9460,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 9461,
        state_count: 1,
        default_state: 9461,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 9462,
        state_count: 1,
        default_state: 9462,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 9463,
        state_count: 1,
        default_state: 9463,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 9464,
        state_count: 1,
        default_state: 9464,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 9465,
        state_count: 1,
        default_state: 9465,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 9466,
        state_count: 1,
        default_state: 9466,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 9467,
        state_count: 1,
        default_state: 9467,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 9468,
        state_count: 1,
        default_state: 9468,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 9469,
        state_count: 1,
        default_state: 9469,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 9470,
        state_count: 26,
        default_state: 9470,
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
        first_state: 9496,
        state_count: 1,
        default_state: 9496,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 9497,
        state_count: 1,
        default_state: 9497,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 9498,
        state_count: 12,
        default_state: 9498,
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
        first_state: 9510,
        state_count: 1,
        default_state: 9510,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 9511,
        state_count: 1,
        default_state: 9511,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 9512,
        state_count: 1,
        default_state: 9512,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 9513,
        state_count: 1,
        default_state: 9513,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 9514,
        state_count: 1,
        default_state: 9514,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 9515,
        state_count: 1,
        default_state: 9515,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 9516,
        state_count: 1,
        default_state: 9516,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 9517,
        state_count: 1,
        default_state: 9517,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 9518,
        state_count: 1,
        default_state: 9518,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 9519,
        state_count: 1,
        default_state: 9519,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 9520,
        state_count: 2,
        default_state: 9520,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 9522,
        state_count: 2,
        default_state: 9522,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 9524,
        state_count: 2,
        default_state: 9524,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 9526,
        state_count: 2,
        default_state: 9526,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 9528,
        state_count: 2,
        default_state: 9528,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 9530,
        state_count: 2,
        default_state: 9530,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 9532,
        state_count: 2,
        default_state: 9532,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 9534,
        state_count: 2,
        default_state: 9534,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 9536,
        state_count: 2,
        default_state: 9536,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 9538,
        state_count: 2,
        default_state: 9538,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 9540,
        state_count: 2,
        default_state: 9540,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 9542,
        state_count: 2,
        default_state: 9542,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 9544,
        state_count: 2,
        default_state: 9544,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 9546,
        state_count: 2,
        default_state: 9546,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 9548,
        state_count: 2,
        default_state: 9548,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 9550,
        state_count: 2,
        default_state: 9550,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 9552,
        state_count: 2,
        default_state: 9552,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 9554,
        state_count: 2,
        default_state: 9554,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 9556,
        state_count: 2,
        default_state: 9556,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 9558,
        state_count: 2,
        default_state: 9558,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 9560,
        state_count: 8,
        default_state: 9560,
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
        first_state: 9568,
        state_count: 8,
        default_state: 9568,
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
        first_state: 9576,
        state_count: 8,
        default_state: 9576,
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
        first_state: 9584,
        state_count: 8,
        default_state: 9584,
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
        first_state: 9592,
        state_count: 8,
        default_state: 9592,
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
        first_state: 9600,
        state_count: 8,
        default_state: 9600,
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
        first_state: 9608,
        state_count: 8,
        default_state: 9608,
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
        first_state: 9616,
        state_count: 8,
        default_state: 9616,
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
        first_state: 9624,
        state_count: 8,
        default_state: 9624,
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
        first_state: 9632,
        state_count: 8,
        default_state: 9632,
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
        first_state: 9640,
        state_count: 8,
        default_state: 9640,
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
        first_state: 9648,
        state_count: 1,
        default_state: 9648,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 9649,
        state_count: 2,
        default_state: 9649,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 9651,
        state_count: 1,
        default_state: 9651,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 9652,
        state_count: 12,
        default_state: 9652,
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
        first_state: 9664,
        state_count: 1,
        default_state: 9664,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 9665,
        state_count: 1,
        default_state: 9665,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 9666,
        state_count: 1,
        default_state: 9666,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 9667,
        state_count: 2,
        default_state: 9667,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 9669,
        state_count: 80,
        default_state: 9680,
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
        first_state: 9749,
        state_count: 80,
        default_state: 9760,
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
        first_state: 9829,
        state_count: 80,
        default_state: 9840,
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
        first_state: 9909,
        state_count: 80,
        default_state: 9920,
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
        name: "end_stone_brick_stairs",
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
        name: "stone_stairs",
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
        name: "smooth_sandstone_stairs",
        first_state: 10229,
        state_count: 80,
        default_state: 10240,
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
        first_state: 10309,
        state_count: 80,
        default_state: 10320,
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
        first_state: 10389,
        state_count: 80,
        default_state: 10400,
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
        first_state: 10469,
        state_count: 80,
        default_state: 10480,
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
        first_state: 10549,
        state_count: 80,
        default_state: 10560,
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
        first_state: 10629,
        state_count: 80,
        default_state: 10640,
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
        first_state: 10709,
        state_count: 80,
        default_state: 10720,
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
        first_state: 10789,
        state_count: 6,
        default_state: 10792,
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
        first_state: 10795,
        state_count: 6,
        default_state: 10798,
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
        first_state: 10801,
        state_count: 6,
        default_state: 10804,
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
        first_state: 10807,
        state_count: 6,
        default_state: 10810,
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
        first_state: 10813,
        state_count: 6,
        default_state: 10816,
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
        first_state: 10819,
        state_count: 6,
        default_state: 10822,
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
        first_state: 10825,
        state_count: 6,
        default_state: 10828,
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
        first_state: 10831,
        state_count: 6,
        default_state: 10834,
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
        first_state: 10837,
        state_count: 6,
        default_state: 10840,
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
        first_state: 10843,
        state_count: 6,
        default_state: 10846,
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
        first_state: 10849,
        state_count: 6,
        default_state: 10852,
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
        first_state: 10855,
        state_count: 6,
        default_state: 10858,
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
        first_state: 10861,
        state_count: 6,
        default_state: 10864,
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
        first_state: 10867,
        state_count: 324,
        default_state: 10870,
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
        first_state: 11191,
        state_count: 324,
        default_state: 11194,
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
        first_state: 11515,
        state_count: 324,
        default_state: 11518,
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
        first_state: 11839,
        state_count: 324,
        default_state: 11842,
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
        first_state: 12163,
        state_count: 324,
        default_state: 12166,
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
        first_state: 12487,
        state_count: 324,
        default_state: 12490,
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
        first_state: 12811,
        state_count: 324,
        default_state: 12814,
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
        first_state: 13135,
        state_count: 324,
        default_state: 13138,
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
        first_state: 13459,
        state_count: 324,
        default_state: 13462,
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
        first_state: 13783,
        state_count: 324,
        default_state: 13786,
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
        first_state: 14107,
        state_count: 324,
        default_state: 14110,
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
        first_state: 14431,
        state_count: 324,
        default_state: 14434,
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
        first_state: 14755,
        state_count: 32,
        default_state: 14786,
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
        first_state: 14787,
        state_count: 4,
        default_state: 14787,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 14791,
        state_count: 12,
        default_state: 14792,
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
        first_state: 14803,
        state_count: 8,
        default_state: 14804,
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
        first_state: 14811,
        state_count: 8,
        default_state: 14812,
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
        first_state: 14819,
        state_count: 1,
        default_state: 14819,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 14820,
        state_count: 1,
        default_state: 14820,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 14821,
        state_count: 12,
        default_state: 14825,
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
        first_state: 14833,
        state_count: 16,
        default_state: 14836,
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
        first_state: 14849,
        state_count: 1,
        default_state: 14849,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 14850,
        state_count: 4,
        default_state: 14850,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 14854,
        state_count: 32,
        default_state: 14855,
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
        first_state: 14886,
        state_count: 2,
        default_state: 14887,
        props: &[PropInfo {
            name: "hanging",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "soul_lantern",
        first_state: 14888,
        state_count: 2,
        default_state: 14889,
        props: &[PropInfo {
            name: "hanging",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "campfire",
        first_state: 14890,
        state_count: 32,
        default_state: 14893,
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
        first_state: 14922,
        state_count: 32,
        default_state: 14925,
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
        first_state: 14954,
        state_count: 4,
        default_state: 14954,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 14958,
        state_count: 3,
        default_state: 14959,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 14961,
        state_count: 3,
        default_state: 14962,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 14964,
        state_count: 3,
        default_state: 14965,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 14967,
        state_count: 3,
        default_state: 14968,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 14970,
        state_count: 1,
        default_state: 14970,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 14971,
        state_count: 1,
        default_state: 14971,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 14972,
        state_count: 1,
        default_state: 14972,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 14973,
        state_count: 1,
        default_state: 14973,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 14974,
        state_count: 1,
        default_state: 14974,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 14975,
        state_count: 3,
        default_state: 14976,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 14978,
        state_count: 3,
        default_state: 14979,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 14981,
        state_count: 3,
        default_state: 14982,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 14984,
        state_count: 3,
        default_state: 14985,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 14987,
        state_count: 1,
        default_state: 14987,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 14988,
        state_count: 1,
        default_state: 14988,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 14989,
        state_count: 1,
        default_state: 14989,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 14990,
        state_count: 26,
        default_state: 14990,
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
        first_state: 15016,
        state_count: 1,
        default_state: 15016,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 15017,
        state_count: 26,
        default_state: 15017,
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
        first_state: 15043,
        state_count: 1,
        default_state: 15043,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 15044,
        state_count: 1,
        default_state: 15044,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 15045,
        state_count: 1,
        default_state: 15045,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 15046,
        state_count: 1,
        default_state: 15046,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 15047,
        state_count: 6,
        default_state: 15050,
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
        first_state: 15053,
        state_count: 6,
        default_state: 15056,
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
        first_state: 15059,
        state_count: 2,
        default_state: 15060,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 15061,
        state_count: 2,
        default_state: 15062,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 15063,
        state_count: 32,
        default_state: 15094,
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
        first_state: 15095,
        state_count: 32,
        default_state: 15126,
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
        first_state: 15127,
        state_count: 64,
        default_state: 15142,
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
        first_state: 15191,
        state_count: 64,
        default_state: 15206,
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
        first_state: 15255,
        state_count: 32,
        default_state: 15262,
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
        first_state: 15287,
        state_count: 32,
        default_state: 15294,
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
        first_state: 15319,
        state_count: 80,
        default_state: 15330,
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
        first_state: 15399,
        state_count: 80,
        default_state: 15410,
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
        first_state: 15479,
        state_count: 24,
        default_state: 15488,
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
        first_state: 15503,
        state_count: 24,
        default_state: 15512,
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
        first_state: 15527,
        state_count: 64,
        default_state: 15538,
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
        first_state: 15591,
        state_count: 64,
        default_state: 15602,
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
        first_state: 15655,
        state_count: 32,
        default_state: 15656,
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
        first_state: 15687,
        state_count: 32,
        default_state: 15688,
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
        first_state: 15719,
        state_count: 8,
        default_state: 15720,
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
        name: "structure_block",
        first_state: 15735,
        state_count: 4,
        default_state: 15735,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 15739,
        state_count: 12,
        default_state: 15749,
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
        first_state: 15751,
        state_count: 9,
        default_state: 15751,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 15760,
        state_count: 16,
        default_state: 15760,
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
        first_state: 15776,
        state_count: 24,
        default_state: 15776,
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
        first_state: 15800,
        state_count: 24,
        default_state: 15800,
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
        first_state: 15824,
        state_count: 1,
        default_state: 15824,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 15825,
        state_count: 1,
        default_state: 15825,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 15826,
        state_count: 1,
        default_state: 15826,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 15827,
        state_count: 1,
        default_state: 15827,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 15828,
        state_count: 1,
        default_state: 15828,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 15829,
        state_count: 5,
        default_state: 15829,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 15834,
        state_count: 1,
        default_state: 15834,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 15835,
        state_count: 1,
        default_state: 15835,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 15836,
        state_count: 1,
        default_state: 15836,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 15837,
        state_count: 1,
        default_state: 15837,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 15838,
        state_count: 1,
        default_state: 15838,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 15839,
        state_count: 1,
        default_state: 15839,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 15840,
        state_count: 80,
        default_state: 15851,
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
        first_state: 15920,
        state_count: 324,
        default_state: 15923,
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
        first_state: 16244,
        state_count: 6,
        default_state: 16247,
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
        first_state: 16250,
        state_count: 1,
        default_state: 16250,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 16251,
        state_count: 1,
        default_state: 16251,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 16252,
        state_count: 1,
        default_state: 16252,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 16253,
        state_count: 1,
        default_state: 16253,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 16254,
        state_count: 6,
        default_state: 16257,
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
        first_state: 16260,
        state_count: 80,
        default_state: 16271,
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
        first_state: 16340,
        state_count: 324,
        default_state: 16343,
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
        first_state: 16664,
        state_count: 1,
        default_state: 16664,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 16665,
        state_count: 80,
        default_state: 16676,
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
        first_state: 16745,
        state_count: 6,
        default_state: 16748,
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
        first_state: 16751,
        state_count: 2,
        default_state: 16752,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 16753,
        state_count: 24,
        default_state: 16762,
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
        first_state: 16777,
        state_count: 324,
        default_state: 16780,
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
        first_state: 17101,
        state_count: 1,
        default_state: 17101,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 17102,
        state_count: 1,
        default_state: 17102,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 17103,
        state_count: 1,
        default_state: 17103,
        props: &[],
    },
];
