// @generated `cargo xtask gen-blockstates 1.15.2` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.15.2";
pub const STATE_COUNT: u32 = 11337;

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
        name: "oak_log",
        first_state: 72,
        state_count: 3,
        default_state: 73,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 75,
        state_count: 3,
        default_state: 76,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 78,
        state_count: 3,
        default_state: 79,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 81,
        state_count: 3,
        default_state: 82,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 84,
        state_count: 3,
        default_state: 85,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 87,
        state_count: 3,
        default_state: 88,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 90,
        state_count: 3,
        default_state: 91,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 93,
        state_count: 3,
        default_state: 94,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 96,
        state_count: 3,
        default_state: 97,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 99,
        state_count: 3,
        default_state: 100,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 102,
        state_count: 3,
        default_state: 103,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 105,
        state_count: 3,
        default_state: 106,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 108,
        state_count: 3,
        default_state: 109,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 111,
        state_count: 3,
        default_state: 112,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 114,
        state_count: 3,
        default_state: 115,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 117,
        state_count: 3,
        default_state: 118,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 120,
        state_count: 3,
        default_state: 121,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 123,
        state_count: 3,
        default_state: 124,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 126,
        state_count: 3,
        default_state: 127,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 129,
        state_count: 3,
        default_state: 130,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 132,
        state_count: 3,
        default_state: 133,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 135,
        state_count: 3,
        default_state: 136,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 138,
        state_count: 3,
        default_state: 139,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 141,
        state_count: 3,
        default_state: 142,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 144,
        state_count: 14,
        default_state: 157,
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
        first_state: 158,
        state_count: 14,
        default_state: 171,
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
        first_state: 172,
        state_count: 14,
        default_state: 185,
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
        first_state: 186,
        state_count: 14,
        default_state: 199,
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
        first_state: 200,
        state_count: 14,
        default_state: 213,
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
        first_state: 214,
        state_count: 14,
        default_state: 227,
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
        first_state: 228,
        state_count: 1,
        default_state: 228,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 229,
        state_count: 1,
        default_state: 229,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 230,
        state_count: 1,
        default_state: 230,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 231,
        state_count: 1,
        default_state: 231,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 232,
        state_count: 1,
        default_state: 232,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 233,
        state_count: 12,
        default_state: 234,
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
        first_state: 245,
        state_count: 1,
        default_state: 245,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 246,
        state_count: 1,
        default_state: 246,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 247,
        state_count: 1,
        default_state: 247,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 248,
        state_count: 800,
        default_state: 249,
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
        first_state: 1048,
        state_count: 16,
        default_state: 1051,
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
        first_state: 1064,
        state_count: 16,
        default_state: 1067,
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
        first_state: 1080,
        state_count: 16,
        default_state: 1083,
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
        first_state: 1096,
        state_count: 16,
        default_state: 1099,
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
        first_state: 1112,
        state_count: 16,
        default_state: 1115,
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
        first_state: 1128,
        state_count: 16,
        default_state: 1131,
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
        first_state: 1144,
        state_count: 16,
        default_state: 1147,
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
        first_state: 1160,
        state_count: 16,
        default_state: 1163,
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
        first_state: 1176,
        state_count: 16,
        default_state: 1179,
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
        first_state: 1192,
        state_count: 16,
        default_state: 1195,
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
        first_state: 1208,
        state_count: 16,
        default_state: 1211,
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
        first_state: 1224,
        state_count: 16,
        default_state: 1227,
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
        first_state: 1240,
        state_count: 16,
        default_state: 1243,
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
        first_state: 1256,
        state_count: 16,
        default_state: 1259,
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
        first_state: 1272,
        state_count: 16,
        default_state: 1275,
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
        first_state: 1288,
        state_count: 16,
        default_state: 1291,
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
        first_state: 1304,
        state_count: 12,
        default_state: 1310,
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
        first_state: 1316,
        state_count: 12,
        default_state: 1322,
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
        first_state: 1328,
        state_count: 12,
        default_state: 1334,
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
        first_state: 1340,
        state_count: 1,
        default_state: 1340,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 1341,
        state_count: 1,
        default_state: 1341,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 1342,
        state_count: 1,
        default_state: 1342,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 1343,
        state_count: 1,
        default_state: 1343,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 1344,
        state_count: 1,
        default_state: 1344,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 1345,
        state_count: 2,
        default_state: 1346,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 1347,
        state_count: 12,
        default_state: 1353,
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
        first_state: 1359,
        state_count: 24,
        default_state: 1361,
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
        first_state: 1383,
        state_count: 1,
        default_state: 1383,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 1384,
        state_count: 1,
        default_state: 1384,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 1385,
        state_count: 1,
        default_state: 1385,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 1386,
        state_count: 1,
        default_state: 1386,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 1387,
        state_count: 1,
        default_state: 1387,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 1388,
        state_count: 1,
        default_state: 1388,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 1389,
        state_count: 1,
        default_state: 1389,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 1390,
        state_count: 1,
        default_state: 1390,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 1391,
        state_count: 1,
        default_state: 1391,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 1392,
        state_count: 1,
        default_state: 1392,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 1393,
        state_count: 1,
        default_state: 1393,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 1394,
        state_count: 1,
        default_state: 1394,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 1395,
        state_count: 1,
        default_state: 1395,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 1396,
        state_count: 1,
        default_state: 1396,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 1397,
        state_count: 1,
        default_state: 1397,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 1398,
        state_count: 1,
        default_state: 1398,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 1399,
        state_count: 12,
        default_state: 1399,
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
        first_state: 1411,
        state_count: 1,
        default_state: 1411,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 1412,
        state_count: 1,
        default_state: 1412,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 1413,
        state_count: 1,
        default_state: 1413,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 1414,
        state_count: 1,
        default_state: 1414,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 1415,
        state_count: 1,
        default_state: 1415,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 1416,
        state_count: 1,
        default_state: 1416,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 1417,
        state_count: 1,
        default_state: 1417,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 1418,
        state_count: 1,
        default_state: 1418,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 1419,
        state_count: 1,
        default_state: 1419,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 1420,
        state_count: 1,
        default_state: 1420,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 1421,
        state_count: 1,
        default_state: 1421,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 1422,
        state_count: 1,
        default_state: 1422,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 1423,
        state_count: 1,
        default_state: 1423,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 1424,
        state_count: 1,
        default_state: 1424,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 1425,
        state_count: 1,
        default_state: 1425,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 1426,
        state_count: 1,
        default_state: 1426,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 1427,
        state_count: 1,
        default_state: 1427,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 1428,
        state_count: 1,
        default_state: 1428,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 1429,
        state_count: 2,
        default_state: 1430,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 1431,
        state_count: 1,
        default_state: 1431,
        props: &[],
    },
    BlockInfo {
        name: "mossy_cobblestone",
        first_state: 1432,
        state_count: 1,
        default_state: 1432,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 1433,
        state_count: 1,
        default_state: 1433,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 1434,
        state_count: 1,
        default_state: 1434,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 1435,
        state_count: 4,
        default_state: 1435,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 1439,
        state_count: 512,
        default_state: 1470,
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
        name: "spawner",
        first_state: 1951,
        state_count: 1,
        default_state: 1951,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 1952,
        state_count: 80,
        default_state: 1963,
        props: &[
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
        first_state: 2032,
        state_count: 24,
        default_state: 2033,
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
        first_state: 2056,
        state_count: 1296,
        default_state: 3216,
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
        first_state: 3352,
        state_count: 1,
        default_state: 3352,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 3353,
        state_count: 1,
        default_state: 3353,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 3354,
        state_count: 1,
        default_state: 3354,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 3355,
        state_count: 8,
        default_state: 3355,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 3363,
        state_count: 8,
        default_state: 3363,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 3371,
        state_count: 8,
        default_state: 3372,
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
        first_state: 3379,
        state_count: 32,
        default_state: 3380,
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
        first_state: 3411,
        state_count: 32,
        default_state: 3412,
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
        first_state: 3443,
        state_count: 32,
        default_state: 3444,
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
        first_state: 3475,
        state_count: 32,
        default_state: 3476,
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
        first_state: 3507,
        state_count: 32,
        default_state: 3508,
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
        first_state: 3539,
        state_count: 32,
        default_state: 3540,
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
        first_state: 3571,
        state_count: 64,
        default_state: 3582,
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
        first_state: 3635,
        state_count: 8,
        default_state: 3636,
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
        first_state: 3643,
        state_count: 10,
        default_state: 3643,
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
        first_state: 3653,
        state_count: 80,
        default_state: 3664,
        props: &[
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
        first_state: 3733,
        state_count: 8,
        default_state: 3734,
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
        first_state: 3741,
        state_count: 8,
        default_state: 3742,
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
        first_state: 3749,
        state_count: 8,
        default_state: 3750,
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
        first_state: 3757,
        state_count: 8,
        default_state: 3758,
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
        first_state: 3765,
        state_count: 8,
        default_state: 3766,
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
        first_state: 3773,
        state_count: 8,
        default_state: 3774,
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
        first_state: 3781,
        state_count: 24,
        default_state: 3790,
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
        first_state: 3805,
        state_count: 2,
        default_state: 3806,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 3807,
        state_count: 64,
        default_state: 3818,
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
        first_state: 3871,
        state_count: 2,
        default_state: 3872,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 3873,
        state_count: 2,
        default_state: 3874,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 3875,
        state_count: 2,
        default_state: 3876,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 3877,
        state_count: 2,
        default_state: 3878,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 3879,
        state_count: 2,
        default_state: 3880,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 3881,
        state_count: 2,
        default_state: 3882,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 3883,
        state_count: 2,
        default_state: 3884,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 3885,
        state_count: 2,
        default_state: 3885,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 3887,
        state_count: 8,
        default_state: 3887,
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
        first_state: 3895,
        state_count: 24,
        default_state: 3904,
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
        first_state: 3919,
        state_count: 8,
        default_state: 3919,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 3927,
        state_count: 1,
        default_state: 3927,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 3928,
        state_count: 1,
        default_state: 3928,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 3929,
        state_count: 16,
        default_state: 3929,
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
        first_state: 3945,
        state_count: 1,
        default_state: 3945,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 3946,
        state_count: 16,
        default_state: 3946,
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
        first_state: 3962,
        state_count: 2,
        default_state: 3963,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 3964,
        state_count: 32,
        default_state: 3995,
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
        first_state: 3996,
        state_count: 1,
        default_state: 3996,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 3997,
        state_count: 1,
        default_state: 3997,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 3998,
        state_count: 1,
        default_state: 3998,
        props: &[],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 3999,
        state_count: 1,
        default_state: 3999,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 4000,
        state_count: 2,
        default_state: 4000,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 4002,
        state_count: 4,
        default_state: 4002,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 4006,
        state_count: 4,
        default_state: 4006,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 4010,
        state_count: 7,
        default_state: 4010,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 4017,
        state_count: 64,
        default_state: 4020,
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
        first_state: 4081,
        state_count: 1,
        default_state: 4081,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 4082,
        state_count: 1,
        default_state: 4082,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 4083,
        state_count: 1,
        default_state: 4083,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 4084,
        state_count: 1,
        default_state: 4084,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 4085,
        state_count: 1,
        default_state: 4085,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 4086,
        state_count: 1,
        default_state: 4086,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 4087,
        state_count: 1,
        default_state: 4087,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 4088,
        state_count: 1,
        default_state: 4088,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 4089,
        state_count: 1,
        default_state: 4089,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 4090,
        state_count: 1,
        default_state: 4090,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 4091,
        state_count: 1,
        default_state: 4091,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 4092,
        state_count: 1,
        default_state: 4092,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 4093,
        state_count: 1,
        default_state: 4093,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 4094,
        state_count: 1,
        default_state: 4094,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 4095,
        state_count: 1,
        default_state: 4095,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 4096,
        state_count: 1,
        default_state: 4096,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 4097,
        state_count: 64,
        default_state: 4112,
        props: &[
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
        first_state: 4161,
        state_count: 64,
        default_state: 4176,
        props: &[
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
        first_state: 4225,
        state_count: 64,
        default_state: 4240,
        props: &[
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
        first_state: 4289,
        state_count: 64,
        default_state: 4304,
        props: &[
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
        first_state: 4353,
        state_count: 64,
        default_state: 4368,
        props: &[
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
        first_state: 4417,
        state_count: 64,
        default_state: 4432,
        props: &[
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
        first_state: 4481,
        state_count: 1,
        default_state: 4481,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 4482,
        state_count: 1,
        default_state: 4482,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 4483,
        state_count: 1,
        default_state: 4483,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 4484,
        state_count: 1,
        default_state: 4484,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 4485,
        state_count: 1,
        default_state: 4485,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 4486,
        state_count: 1,
        default_state: 4486,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 4487,
        state_count: 1,
        default_state: 4487,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 4488,
        state_count: 1,
        default_state: 4488,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 4489,
        state_count: 1,
        default_state: 4489,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 4490,
        state_count: 1,
        default_state: 4490,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 4491,
        state_count: 64,
        default_state: 4491,
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
        first_state: 4555,
        state_count: 64,
        default_state: 4555,
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
        first_state: 4619,
        state_count: 64,
        default_state: 4619,
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
        first_state: 4683,
        state_count: 32,
        default_state: 4714,
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
        name: "glass_pane",
        first_state: 4715,
        state_count: 32,
        default_state: 4746,
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
        first_state: 4747,
        state_count: 1,
        default_state: 4747,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 4748,
        state_count: 4,
        default_state: 4748,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 4752,
        state_count: 4,
        default_state: 4752,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 4756,
        state_count: 8,
        default_state: 4756,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 4764,
        state_count: 8,
        default_state: 4764,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 4772,
        state_count: 32,
        default_state: 4803,
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
        first_state: 4804,
        state_count: 32,
        default_state: 4811,
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
        first_state: 4836,
        state_count: 80,
        default_state: 4847,
        props: &[
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
        first_state: 4916,
        state_count: 80,
        default_state: 4927,
        props: &[
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
        first_state: 4996,
        state_count: 2,
        default_state: 4997,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 4998,
        state_count: 1,
        default_state: 4998,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 4999,
        state_count: 1,
        default_state: 4999,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 5000,
        state_count: 32,
        default_state: 5031,
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
        first_state: 5032,
        state_count: 80,
        default_state: 5043,
        props: &[
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
        first_state: 5112,
        state_count: 4,
        default_state: 5112,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 5116,
        state_count: 1,
        default_state: 5116,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 5117,
        state_count: 8,
        default_state: 5124,
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
        first_state: 5125,
        state_count: 4,
        default_state: 5125,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 5129,
        state_count: 1,
        default_state: 5129,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 5130,
        state_count: 8,
        default_state: 5134,
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
        first_state: 5138,
        state_count: 1,
        default_state: 5138,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 5139,
        state_count: 1,
        default_state: 5139,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 5140,
        state_count: 2,
        default_state: 5141,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 5142,
        state_count: 12,
        default_state: 5142,
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
        first_state: 5154,
        state_count: 80,
        default_state: 5165,
        props: &[
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
        first_state: 5234,
        state_count: 1,
        default_state: 5234,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 5235,
        state_count: 8,
        default_state: 5236,
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
        first_state: 5243,
        state_count: 16,
        default_state: 5252,
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
        first_state: 5259,
        state_count: 128,
        default_state: 5386,
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
        first_state: 5387,
        state_count: 1,
        default_state: 5387,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 5388,
        state_count: 80,
        default_state: 5399,
        props: &[
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
        first_state: 5468,
        state_count: 80,
        default_state: 5479,
        props: &[
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
        first_state: 5548,
        state_count: 80,
        default_state: 5559,
        props: &[
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
        first_state: 5628,
        state_count: 12,
        default_state: 5634,
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
        first_state: 5640,
        state_count: 1,
        default_state: 5640,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 5641,
        state_count: 64,
        default_state: 5700,
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
        name: "mossy_cobblestone_wall",
        first_state: 5705,
        state_count: 64,
        default_state: 5764,
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
        name: "flower_pot",
        first_state: 5769,
        state_count: 1,
        default_state: 5769,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 5770,
        state_count: 1,
        default_state: 5770,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 5771,
        state_count: 1,
        default_state: 5771,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 5772,
        state_count: 1,
        default_state: 5772,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 5773,
        state_count: 1,
        default_state: 5773,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 5774,
        state_count: 1,
        default_state: 5774,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 5775,
        state_count: 1,
        default_state: 5775,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 5776,
        state_count: 1,
        default_state: 5776,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 5777,
        state_count: 1,
        default_state: 5777,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 5778,
        state_count: 1,
        default_state: 5778,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 5779,
        state_count: 1,
        default_state: 5779,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 5780,
        state_count: 1,
        default_state: 5780,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 5781,
        state_count: 1,
        default_state: 5781,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 5782,
        state_count: 1,
        default_state: 5782,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 5783,
        state_count: 1,
        default_state: 5783,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 5784,
        state_count: 1,
        default_state: 5784,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 5785,
        state_count: 1,
        default_state: 5785,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 5786,
        state_count: 1,
        default_state: 5786,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 5787,
        state_count: 1,
        default_state: 5787,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 5788,
        state_count: 1,
        default_state: 5788,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 5789,
        state_count: 1,
        default_state: 5789,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 5790,
        state_count: 1,
        default_state: 5790,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 5791,
        state_count: 1,
        default_state: 5791,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 5792,
        state_count: 1,
        default_state: 5792,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 5793,
        state_count: 1,
        default_state: 5793,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 5794,
        state_count: 8,
        default_state: 5794,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 5802,
        state_count: 8,
        default_state: 5802,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 5810,
        state_count: 24,
        default_state: 5819,
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
        first_state: 5834,
        state_count: 24,
        default_state: 5843,
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
        first_state: 5858,
        state_count: 24,
        default_state: 5867,
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
        first_state: 5882,
        state_count: 24,
        default_state: 5891,
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
        first_state: 5906,
        state_count: 24,
        default_state: 5915,
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
        first_state: 5930,
        state_count: 24,
        default_state: 5939,
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
        first_state: 5954,
        state_count: 16,
        default_state: 5954,
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
        first_state: 5970,
        state_count: 4,
        default_state: 5970,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 5974,
        state_count: 16,
        default_state: 5974,
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
        first_state: 5990,
        state_count: 4,
        default_state: 5990,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 5994,
        state_count: 16,
        default_state: 5994,
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
        first_state: 6010,
        state_count: 4,
        default_state: 6010,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 6014,
        state_count: 16,
        default_state: 6014,
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
        first_state: 6030,
        state_count: 4,
        default_state: 6030,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 6034,
        state_count: 16,
        default_state: 6034,
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
        first_state: 6050,
        state_count: 4,
        default_state: 6050,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 6054,
        state_count: 16,
        default_state: 6054,
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
        first_state: 6070,
        state_count: 4,
        default_state: 6070,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 6074,
        state_count: 4,
        default_state: 6074,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 6078,
        state_count: 4,
        default_state: 6078,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 6082,
        state_count: 4,
        default_state: 6082,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 6086,
        state_count: 24,
        default_state: 6087,
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
        first_state: 6110,
        state_count: 16,
        default_state: 6110,
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
        first_state: 6126,
        state_count: 16,
        default_state: 6126,
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
        first_state: 6142,
        state_count: 16,
        default_state: 6143,
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
        first_state: 6158,
        state_count: 32,
        default_state: 6174,
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
        first_state: 6190,
        state_count: 1,
        default_state: 6190,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 6191,
        state_count: 1,
        default_state: 6191,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 6192,
        state_count: 10,
        default_state: 6192,
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
        first_state: 6202,
        state_count: 1,
        default_state: 6202,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 6203,
        state_count: 1,
        default_state: 6203,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 6204,
        state_count: 3,
        default_state: 6205,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 6207,
        state_count: 80,
        default_state: 6218,
        props: &[
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
        first_state: 6287,
        state_count: 12,
        default_state: 6293,
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
        first_state: 6299,
        state_count: 12,
        default_state: 6300,
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
        first_state: 6311,
        state_count: 1,
        default_state: 6311,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 6312,
        state_count: 1,
        default_state: 6312,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 6313,
        state_count: 1,
        default_state: 6313,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 6314,
        state_count: 1,
        default_state: 6314,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 6315,
        state_count: 1,
        default_state: 6315,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 6316,
        state_count: 1,
        default_state: 6316,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 6317,
        state_count: 1,
        default_state: 6317,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 6318,
        state_count: 1,
        default_state: 6318,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 6319,
        state_count: 1,
        default_state: 6319,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 6320,
        state_count: 1,
        default_state: 6320,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 6321,
        state_count: 1,
        default_state: 6321,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 6322,
        state_count: 1,
        default_state: 6322,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 6323,
        state_count: 1,
        default_state: 6323,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 6324,
        state_count: 1,
        default_state: 6324,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 6325,
        state_count: 1,
        default_state: 6325,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 6326,
        state_count: 1,
        default_state: 6326,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 6327,
        state_count: 32,
        default_state: 6358,
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
        first_state: 6359,
        state_count: 32,
        default_state: 6390,
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
        first_state: 6391,
        state_count: 32,
        default_state: 6422,
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
        first_state: 6423,
        state_count: 32,
        default_state: 6454,
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
        first_state: 6455,
        state_count: 32,
        default_state: 6486,
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
        first_state: 6487,
        state_count: 32,
        default_state: 6518,
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
        first_state: 6519,
        state_count: 32,
        default_state: 6550,
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
        first_state: 6551,
        state_count: 32,
        default_state: 6582,
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
        first_state: 6583,
        state_count: 32,
        default_state: 6614,
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
        first_state: 6615,
        state_count: 32,
        default_state: 6646,
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
        first_state: 6647,
        state_count: 32,
        default_state: 6678,
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
        first_state: 6679,
        state_count: 32,
        default_state: 6710,
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
        first_state: 6711,
        state_count: 32,
        default_state: 6742,
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
        first_state: 6743,
        state_count: 32,
        default_state: 6774,
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
        first_state: 6775,
        state_count: 32,
        default_state: 6806,
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
        first_state: 6807,
        state_count: 32,
        default_state: 6838,
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
        first_state: 6839,
        state_count: 80,
        default_state: 6850,
        props: &[
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
        first_state: 6919,
        state_count: 80,
        default_state: 6930,
        props: &[
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
        first_state: 6999,
        state_count: 1,
        default_state: 6999,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 7000,
        state_count: 1,
        default_state: 7000,
        props: &[],
    },
    BlockInfo {
        name: "iron_trapdoor",
        first_state: 7001,
        state_count: 64,
        default_state: 7016,
        props: &[
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
        first_state: 7065,
        state_count: 1,
        default_state: 7065,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 7066,
        state_count: 1,
        default_state: 7066,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 7067,
        state_count: 1,
        default_state: 7067,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 7068,
        state_count: 80,
        default_state: 7079,
        props: &[
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
        first_state: 7148,
        state_count: 80,
        default_state: 7159,
        props: &[
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
        first_state: 7228,
        state_count: 80,
        default_state: 7239,
        props: &[
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
        first_state: 7308,
        state_count: 6,
        default_state: 7311,
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
        first_state: 7314,
        state_count: 6,
        default_state: 7317,
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
        first_state: 7320,
        state_count: 6,
        default_state: 7323,
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
        first_state: 7326,
        state_count: 1,
        default_state: 7326,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 7327,
        state_count: 3,
        default_state: 7328,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 7330,
        state_count: 1,
        default_state: 7330,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 7331,
        state_count: 1,
        default_state: 7331,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 7332,
        state_count: 1,
        default_state: 7332,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 7333,
        state_count: 1,
        default_state: 7333,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 7334,
        state_count: 1,
        default_state: 7334,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 7335,
        state_count: 1,
        default_state: 7335,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 7336,
        state_count: 1,
        default_state: 7336,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 7337,
        state_count: 1,
        default_state: 7337,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 7338,
        state_count: 1,
        default_state: 7338,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 7339,
        state_count: 1,
        default_state: 7339,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 7340,
        state_count: 1,
        default_state: 7340,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 7341,
        state_count: 1,
        default_state: 7341,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 7342,
        state_count: 1,
        default_state: 7342,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 7343,
        state_count: 1,
        default_state: 7343,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 7344,
        state_count: 1,
        default_state: 7344,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 7345,
        state_count: 1,
        default_state: 7345,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 7346,
        state_count: 1,
        default_state: 7346,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 7347,
        state_count: 1,
        default_state: 7347,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 7348,
        state_count: 1,
        default_state: 7348,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 7349,
        state_count: 2,
        default_state: 7350,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 7351,
        state_count: 2,
        default_state: 7352,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 7353,
        state_count: 2,
        default_state: 7354,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 7355,
        state_count: 2,
        default_state: 7356,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 7357,
        state_count: 2,
        default_state: 7358,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 7359,
        state_count: 2,
        default_state: 7360,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 7361,
        state_count: 16,
        default_state: 7361,
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
        first_state: 7377,
        state_count: 16,
        default_state: 7377,
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
        first_state: 7393,
        state_count: 16,
        default_state: 7393,
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
        first_state: 7409,
        state_count: 16,
        default_state: 7409,
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
        first_state: 7425,
        state_count: 16,
        default_state: 7425,
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
        first_state: 7441,
        state_count: 16,
        default_state: 7441,
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
        first_state: 7457,
        state_count: 16,
        default_state: 7457,
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
        first_state: 7473,
        state_count: 16,
        default_state: 7473,
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
        first_state: 7489,
        state_count: 16,
        default_state: 7489,
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
        first_state: 7505,
        state_count: 16,
        default_state: 7505,
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
        first_state: 7521,
        state_count: 16,
        default_state: 7521,
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
        first_state: 7537,
        state_count: 16,
        default_state: 7537,
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
        first_state: 7553,
        state_count: 16,
        default_state: 7553,
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
        first_state: 7569,
        state_count: 16,
        default_state: 7569,
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
        first_state: 7585,
        state_count: 16,
        default_state: 7585,
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
        first_state: 7601,
        state_count: 16,
        default_state: 7601,
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
        first_state: 7617,
        state_count: 4,
        default_state: 7617,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 7621,
        state_count: 4,
        default_state: 7621,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 7625,
        state_count: 4,
        default_state: 7625,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 7629,
        state_count: 4,
        default_state: 7629,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 7633,
        state_count: 4,
        default_state: 7633,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 7637,
        state_count: 4,
        default_state: 7637,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 7641,
        state_count: 4,
        default_state: 7641,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 7645,
        state_count: 4,
        default_state: 7645,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 7649,
        state_count: 4,
        default_state: 7649,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 7653,
        state_count: 4,
        default_state: 7653,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 7657,
        state_count: 4,
        default_state: 7657,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 7661,
        state_count: 4,
        default_state: 7661,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 7665,
        state_count: 4,
        default_state: 7665,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 7669,
        state_count: 4,
        default_state: 7669,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 7673,
        state_count: 4,
        default_state: 7673,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 7677,
        state_count: 4,
        default_state: 7677,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 7681,
        state_count: 1,
        default_state: 7681,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 7682,
        state_count: 1,
        default_state: 7682,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 7683,
        state_count: 1,
        default_state: 7683,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
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
        name: "oak_slab",
        first_state: 7764,
        state_count: 6,
        default_state: 7767,
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
        first_state: 7770,
        state_count: 6,
        default_state: 7773,
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
        first_state: 7776,
        state_count: 6,
        default_state: 7779,
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
        first_state: 7782,
        state_count: 6,
        default_state: 7785,
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
        first_state: 7788,
        state_count: 6,
        default_state: 7791,
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
        first_state: 7794,
        state_count: 6,
        default_state: 7797,
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
        first_state: 7800,
        state_count: 6,
        default_state: 7803,
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
        first_state: 7806,
        state_count: 6,
        default_state: 7809,
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
        first_state: 7812,
        state_count: 6,
        default_state: 7815,
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
        first_state: 7818,
        state_count: 6,
        default_state: 7821,
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
        first_state: 7824,
        state_count: 6,
        default_state: 7827,
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
        first_state: 7830,
        state_count: 6,
        default_state: 7833,
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
        first_state: 7836,
        state_count: 6,
        default_state: 7839,
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
        first_state: 7842,
        state_count: 6,
        default_state: 7845,
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
        name: "quartz_slab",
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
        name: "red_sandstone_slab",
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
        name: "cut_red_sandstone_slab",
        first_state: 7866,
        state_count: 6,
        default_state: 7869,
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
        first_state: 7872,
        state_count: 6,
        default_state: 7875,
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
        first_state: 7878,
        state_count: 1,
        default_state: 7878,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 7879,
        state_count: 1,
        default_state: 7879,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 7880,
        state_count: 1,
        default_state: 7880,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 7881,
        state_count: 1,
        default_state: 7881,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 7882,
        state_count: 32,
        default_state: 7889,
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
        first_state: 7914,
        state_count: 32,
        default_state: 7921,
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
        first_state: 7946,
        state_count: 32,
        default_state: 7953,
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
        first_state: 7978,
        state_count: 32,
        default_state: 7985,
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
        first_state: 8010,
        state_count: 32,
        default_state: 8017,
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
        first_state: 8042,
        state_count: 32,
        default_state: 8073,
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
        first_state: 8074,
        state_count: 32,
        default_state: 8105,
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
        first_state: 8106,
        state_count: 32,
        default_state: 8137,
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
        first_state: 8138,
        state_count: 32,
        default_state: 8169,
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
        first_state: 8170,
        state_count: 32,
        default_state: 8201,
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
        first_state: 8202,
        state_count: 64,
        default_state: 8213,
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
        first_state: 8266,
        state_count: 64,
        default_state: 8277,
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
        first_state: 8330,
        state_count: 64,
        default_state: 8341,
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
        first_state: 8394,
        state_count: 64,
        default_state: 8405,
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
        first_state: 8458,
        state_count: 64,
        default_state: 8469,
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
        first_state: 8522,
        state_count: 6,
        default_state: 8526,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 8528,
        state_count: 64,
        default_state: 8591,
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
        first_state: 8592,
        state_count: 6,
        default_state: 8592,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 8598,
        state_count: 1,
        default_state: 8598,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 8599,
        state_count: 3,
        default_state: 8600,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 8602,
        state_count: 80,
        default_state: 8613,
        props: &[
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
        first_state: 8682,
        state_count: 1,
        default_state: 8682,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 8683,
        state_count: 4,
        default_state: 8683,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "grass_path",
        first_state: 8687,
        state_count: 1,
        default_state: 8687,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 8688,
        state_count: 1,
        default_state: 8688,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 8689,
        state_count: 12,
        default_state: 8695,
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
        first_state: 8701,
        state_count: 12,
        default_state: 8707,
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
        first_state: 8713,
        state_count: 4,
        default_state: 8713,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 8717,
        state_count: 1,
        default_state: 8717,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 8718,
        state_count: 1,
        default_state: 8718,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 8719,
        state_count: 1,
        default_state: 8719,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 8720,
        state_count: 3,
        default_state: 8721,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 8723,
        state_count: 1,
        default_state: 8723,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 8724,
        state_count: 12,
        default_state: 8729,
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
        first_state: 8736,
        state_count: 6,
        default_state: 8740,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 8742,
        state_count: 6,
        default_state: 8746,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 8748,
        state_count: 6,
        default_state: 8752,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 8754,
        state_count: 6,
        default_state: 8758,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 8760,
        state_count: 6,
        default_state: 8764,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 8766,
        state_count: 6,
        default_state: 8770,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 8772,
        state_count: 6,
        default_state: 8776,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 8778,
        state_count: 6,
        default_state: 8782,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 8784,
        state_count: 6,
        default_state: 8788,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 8790,
        state_count: 6,
        default_state: 8794,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 8796,
        state_count: 6,
        default_state: 8800,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 8802,
        state_count: 6,
        default_state: 8806,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 8808,
        state_count: 6,
        default_state: 8812,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 8814,
        state_count: 6,
        default_state: 8818,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 8820,
        state_count: 6,
        default_state: 8824,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 8826,
        state_count: 6,
        default_state: 8830,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 8832,
        state_count: 6,
        default_state: 8836,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 8838,
        state_count: 4,
        default_state: 8838,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 8842,
        state_count: 4,
        default_state: 8842,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 8846,
        state_count: 4,
        default_state: 8846,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 8850,
        state_count: 4,
        default_state: 8850,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 8854,
        state_count: 4,
        default_state: 8854,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 8858,
        state_count: 4,
        default_state: 8858,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 8862,
        state_count: 4,
        default_state: 8862,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 8866,
        state_count: 4,
        default_state: 8866,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 8870,
        state_count: 4,
        default_state: 8870,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 8874,
        state_count: 4,
        default_state: 8874,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 8878,
        state_count: 4,
        default_state: 8878,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 8882,
        state_count: 4,
        default_state: 8882,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 8886,
        state_count: 4,
        default_state: 8886,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 8890,
        state_count: 4,
        default_state: 8890,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 8894,
        state_count: 4,
        default_state: 8894,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 8898,
        state_count: 4,
        default_state: 8898,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 8902,
        state_count: 1,
        default_state: 8902,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 8903,
        state_count: 1,
        default_state: 8903,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 8904,
        state_count: 1,
        default_state: 8904,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 8905,
        state_count: 1,
        default_state: 8905,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 8906,
        state_count: 1,
        default_state: 8906,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 8907,
        state_count: 1,
        default_state: 8907,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 8908,
        state_count: 1,
        default_state: 8908,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 8909,
        state_count: 1,
        default_state: 8909,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 8910,
        state_count: 1,
        default_state: 8910,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 8911,
        state_count: 1,
        default_state: 8911,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 8912,
        state_count: 1,
        default_state: 8912,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 8913,
        state_count: 1,
        default_state: 8913,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 8914,
        state_count: 1,
        default_state: 8914,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 8915,
        state_count: 1,
        default_state: 8915,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 8916,
        state_count: 1,
        default_state: 8916,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 8917,
        state_count: 1,
        default_state: 8917,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 8918,
        state_count: 1,
        default_state: 8918,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 8919,
        state_count: 1,
        default_state: 8919,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 8920,
        state_count: 1,
        default_state: 8920,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 8921,
        state_count: 1,
        default_state: 8921,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 8922,
        state_count: 1,
        default_state: 8922,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 8923,
        state_count: 1,
        default_state: 8923,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 8924,
        state_count: 1,
        default_state: 8924,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 8925,
        state_count: 1,
        default_state: 8925,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 8926,
        state_count: 1,
        default_state: 8926,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 8927,
        state_count: 1,
        default_state: 8927,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 8928,
        state_count: 1,
        default_state: 8928,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 8929,
        state_count: 1,
        default_state: 8929,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 8930,
        state_count: 1,
        default_state: 8930,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 8931,
        state_count: 1,
        default_state: 8931,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 8932,
        state_count: 1,
        default_state: 8932,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 8933,
        state_count: 1,
        default_state: 8933,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 8934,
        state_count: 26,
        default_state: 8934,
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
        first_state: 8960,
        state_count: 1,
        default_state: 8960,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 8961,
        state_count: 1,
        default_state: 8961,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 8962,
        state_count: 12,
        default_state: 8962,
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
        first_state: 8974,
        state_count: 1,
        default_state: 8974,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 8975,
        state_count: 1,
        default_state: 8975,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 8976,
        state_count: 1,
        default_state: 8976,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 8977,
        state_count: 1,
        default_state: 8977,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 8978,
        state_count: 1,
        default_state: 8978,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 8979,
        state_count: 1,
        default_state: 8979,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 8980,
        state_count: 1,
        default_state: 8980,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 8981,
        state_count: 1,
        default_state: 8981,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 8982,
        state_count: 1,
        default_state: 8982,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 8983,
        state_count: 1,
        default_state: 8983,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 8984,
        state_count: 2,
        default_state: 8984,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 8986,
        state_count: 2,
        default_state: 8986,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 8988,
        state_count: 2,
        default_state: 8988,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 8990,
        state_count: 2,
        default_state: 8990,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 8992,
        state_count: 2,
        default_state: 8992,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 8994,
        state_count: 2,
        default_state: 8994,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 8996,
        state_count: 2,
        default_state: 8996,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 8998,
        state_count: 2,
        default_state: 8998,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 9000,
        state_count: 2,
        default_state: 9000,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 9002,
        state_count: 2,
        default_state: 9002,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 9004,
        state_count: 2,
        default_state: 9004,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 9006,
        state_count: 2,
        default_state: 9006,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 9008,
        state_count: 2,
        default_state: 9008,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 9010,
        state_count: 2,
        default_state: 9010,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 9012,
        state_count: 2,
        default_state: 9012,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 9014,
        state_count: 2,
        default_state: 9014,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 9016,
        state_count: 2,
        default_state: 9016,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 9018,
        state_count: 2,
        default_state: 9018,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 9020,
        state_count: 2,
        default_state: 9020,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 9022,
        state_count: 2,
        default_state: 9022,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 9024,
        state_count: 8,
        default_state: 9024,
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
        first_state: 9032,
        state_count: 8,
        default_state: 9032,
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
        first_state: 9040,
        state_count: 8,
        default_state: 9040,
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
        first_state: 9048,
        state_count: 8,
        default_state: 9048,
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
        first_state: 9056,
        state_count: 8,
        default_state: 9056,
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
        first_state: 9064,
        state_count: 8,
        default_state: 9064,
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
        first_state: 9072,
        state_count: 8,
        default_state: 9072,
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
        first_state: 9080,
        state_count: 8,
        default_state: 9080,
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
        first_state: 9088,
        state_count: 8,
        default_state: 9088,
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
        first_state: 9096,
        state_count: 8,
        default_state: 9096,
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
        first_state: 9104,
        state_count: 8,
        default_state: 9104,
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
        first_state: 9112,
        state_count: 1,
        default_state: 9112,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 9113,
        state_count: 2,
        default_state: 9113,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 9115,
        state_count: 1,
        default_state: 9115,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 9116,
        state_count: 12,
        default_state: 9116,
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
        first_state: 9128,
        state_count: 1,
        default_state: 9128,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 9129,
        state_count: 1,
        default_state: 9129,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 9130,
        state_count: 1,
        default_state: 9130,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 9131,
        state_count: 2,
        default_state: 9131,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 9133,
        state_count: 80,
        default_state: 9144,
        props: &[
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
        first_state: 9213,
        state_count: 80,
        default_state: 9224,
        props: &[
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
        first_state: 9293,
        state_count: 80,
        default_state: 9304,
        props: &[
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
        first_state: 9373,
        state_count: 80,
        default_state: 9384,
        props: &[
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
        first_state: 9453,
        state_count: 80,
        default_state: 9464,
        props: &[
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
        first_state: 9533,
        state_count: 80,
        default_state: 9544,
        props: &[
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
        first_state: 9613,
        state_count: 80,
        default_state: 9624,
        props: &[
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
        first_state: 9693,
        state_count: 80,
        default_state: 9704,
        props: &[
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
        first_state: 9773,
        state_count: 80,
        default_state: 9784,
        props: &[
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
        first_state: 9853,
        state_count: 80,
        default_state: 9864,
        props: &[
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
        first_state: 9933,
        state_count: 80,
        default_state: 9944,
        props: &[
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
        first_state: 10013,
        state_count: 80,
        default_state: 10024,
        props: &[
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
        first_state: 10093,
        state_count: 80,
        default_state: 10104,
        props: &[
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
        first_state: 10173,
        state_count: 80,
        default_state: 10184,
        props: &[
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
        first_state: 10253,
        state_count: 6,
        default_state: 10256,
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
        first_state: 10259,
        state_count: 6,
        default_state: 10262,
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
        first_state: 10265,
        state_count: 6,
        default_state: 10268,
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
        first_state: 10271,
        state_count: 6,
        default_state: 10274,
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
        first_state: 10277,
        state_count: 6,
        default_state: 10280,
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
        first_state: 10283,
        state_count: 6,
        default_state: 10286,
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
        first_state: 10289,
        state_count: 6,
        default_state: 10292,
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
        first_state: 10295,
        state_count: 6,
        default_state: 10298,
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
        first_state: 10301,
        state_count: 6,
        default_state: 10304,
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
        first_state: 10307,
        state_count: 6,
        default_state: 10310,
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
        first_state: 10313,
        state_count: 6,
        default_state: 10316,
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
        first_state: 10319,
        state_count: 6,
        default_state: 10322,
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
        first_state: 10325,
        state_count: 6,
        default_state: 10328,
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
        first_state: 10331,
        state_count: 64,
        default_state: 10390,
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
        name: "prismarine_wall",
        first_state: 10395,
        state_count: 64,
        default_state: 10454,
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
        name: "red_sandstone_wall",
        first_state: 10459,
        state_count: 64,
        default_state: 10518,
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
        name: "mossy_stone_brick_wall",
        first_state: 10523,
        state_count: 64,
        default_state: 10582,
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
        name: "granite_wall",
        first_state: 10587,
        state_count: 64,
        default_state: 10646,
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
        name: "stone_brick_wall",
        first_state: 10651,
        state_count: 64,
        default_state: 10710,
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
        name: "nether_brick_wall",
        first_state: 10715,
        state_count: 64,
        default_state: 10774,
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
        name: "andesite_wall",
        first_state: 10779,
        state_count: 64,
        default_state: 10838,
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
        name: "red_nether_brick_wall",
        first_state: 10843,
        state_count: 64,
        default_state: 10902,
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
        name: "sandstone_wall",
        first_state: 10907,
        state_count: 64,
        default_state: 10966,
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
        name: "end_stone_brick_wall",
        first_state: 10971,
        state_count: 64,
        default_state: 11030,
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
        name: "diorite_wall",
        first_state: 11035,
        state_count: 64,
        default_state: 11094,
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
        name: "scaffolding",
        first_state: 11099,
        state_count: 32,
        default_state: 11130,
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
        first_state: 11131,
        state_count: 4,
        default_state: 11131,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 11135,
        state_count: 12,
        default_state: 11136,
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
        first_state: 11147,
        state_count: 8,
        default_state: 11148,
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
        first_state: 11155,
        state_count: 8,
        default_state: 11156,
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
        first_state: 11163,
        state_count: 1,
        default_state: 11163,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 11164,
        state_count: 1,
        default_state: 11164,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 11165,
        state_count: 12,
        default_state: 11169,
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
        first_state: 11177,
        state_count: 16,
        default_state: 11180,
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
        first_state: 11193,
        state_count: 1,
        default_state: 11193,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 11194,
        state_count: 4,
        default_state: 11194,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 11198,
        state_count: 32,
        default_state: 11199,
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
        first_state: 11230,
        state_count: 2,
        default_state: 11231,
        props: &[PropInfo {
            name: "hanging",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "campfire",
        first_state: 11232,
        state_count: 32,
        default_state: 11235,
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
        first_state: 11264,
        state_count: 4,
        default_state: 11264,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "structure_block",
        first_state: 11268,
        state_count: 4,
        default_state: 11268,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 11272,
        state_count: 6,
        default_state: 11276,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "composter",
        first_state: 11278,
        state_count: 9,
        default_state: 11278,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "bee_nest",
        first_state: 11287,
        state_count: 24,
        default_state: 11287,
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
        first_state: 11311,
        state_count: 24,
        default_state: 11311,
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
        first_state: 11335,
        state_count: 1,
        default_state: 11335,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 11336,
        state_count: 1,
        default_state: 11336,
        props: &[],
    },
];
