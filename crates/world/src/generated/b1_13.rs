// @generated `cargo xtask gen-blockstates 1.13` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.13";
pub const STATE_COUNT: u32 = 8582;

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
        state_count: 500,
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
        first_state: 748,
        state_count: 16,
        default_state: 751,
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
        first_state: 764,
        state_count: 16,
        default_state: 767,
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
        first_state: 780,
        state_count: 16,
        default_state: 783,
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
        first_state: 796,
        state_count: 16,
        default_state: 799,
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
        first_state: 812,
        state_count: 16,
        default_state: 815,
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
        first_state: 828,
        state_count: 16,
        default_state: 831,
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
        first_state: 844,
        state_count: 16,
        default_state: 847,
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
        first_state: 860,
        state_count: 16,
        default_state: 863,
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
        first_state: 876,
        state_count: 16,
        default_state: 879,
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
        first_state: 892,
        state_count: 16,
        default_state: 895,
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
        first_state: 908,
        state_count: 16,
        default_state: 911,
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
        first_state: 924,
        state_count: 16,
        default_state: 927,
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
        first_state: 940,
        state_count: 16,
        default_state: 943,
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
        first_state: 956,
        state_count: 16,
        default_state: 959,
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
        first_state: 972,
        state_count: 16,
        default_state: 975,
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
        first_state: 988,
        state_count: 16,
        default_state: 991,
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
        first_state: 1004,
        state_count: 12,
        default_state: 1010,
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
        first_state: 1016,
        state_count: 12,
        default_state: 1022,
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
        first_state: 1028,
        state_count: 12,
        default_state: 1034,
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
        first_state: 1040,
        state_count: 1,
        default_state: 1040,
        props: &[],
    },
    BlockInfo {
        name: "grass",
        first_state: 1041,
        state_count: 1,
        default_state: 1041,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 1042,
        state_count: 1,
        default_state: 1042,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 1043,
        state_count: 1,
        default_state: 1043,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 1044,
        state_count: 1,
        default_state: 1044,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 1045,
        state_count: 2,
        default_state: 1046,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 1047,
        state_count: 12,
        default_state: 1053,
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
        first_state: 1059,
        state_count: 24,
        default_state: 1061,
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
        first_state: 1083,
        state_count: 1,
        default_state: 1083,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 1084,
        state_count: 1,
        default_state: 1084,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 1085,
        state_count: 1,
        default_state: 1085,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 1086,
        state_count: 1,
        default_state: 1086,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 1087,
        state_count: 1,
        default_state: 1087,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 1088,
        state_count: 1,
        default_state: 1088,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 1089,
        state_count: 1,
        default_state: 1089,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 1090,
        state_count: 1,
        default_state: 1090,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 1091,
        state_count: 1,
        default_state: 1091,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 1092,
        state_count: 1,
        default_state: 1092,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 1093,
        state_count: 1,
        default_state: 1093,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 1094,
        state_count: 1,
        default_state: 1094,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 1095,
        state_count: 1,
        default_state: 1095,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 1096,
        state_count: 1,
        default_state: 1096,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 1097,
        state_count: 1,
        default_state: 1097,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 1098,
        state_count: 1,
        default_state: 1098,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 1099,
        state_count: 12,
        default_state: 1099,
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
        first_state: 1111,
        state_count: 1,
        default_state: 1111,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 1112,
        state_count: 1,
        default_state: 1112,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 1113,
        state_count: 1,
        default_state: 1113,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 1114,
        state_count: 1,
        default_state: 1114,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 1115,
        state_count: 1,
        default_state: 1115,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 1116,
        state_count: 1,
        default_state: 1116,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 1117,
        state_count: 1,
        default_state: 1117,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 1118,
        state_count: 1,
        default_state: 1118,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 1119,
        state_count: 1,
        default_state: 1119,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 1120,
        state_count: 1,
        default_state: 1120,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 1121,
        state_count: 1,
        default_state: 1121,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 1122,
        state_count: 1,
        default_state: 1122,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 1123,
        state_count: 1,
        default_state: 1123,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 1124,
        state_count: 1,
        default_state: 1124,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 1125,
        state_count: 1,
        default_state: 1125,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 1126,
        state_count: 1,
        default_state: 1126,
        props: &[],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 1127,
        state_count: 1,
        default_state: 1127,
        props: &[],
    },
    BlockInfo {
        name: "mossy_cobblestone",
        first_state: 1128,
        state_count: 1,
        default_state: 1128,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 1129,
        state_count: 1,
        default_state: 1129,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 1130,
        state_count: 1,
        default_state: 1130,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 1131,
        state_count: 4,
        default_state: 1131,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 1135,
        state_count: 512,
        default_state: 1166,
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
        first_state: 1647,
        state_count: 1,
        default_state: 1647,
        props: &[],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 1648,
        state_count: 80,
        default_state: 1659,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 1728,
        state_count: 24,
        default_state: 1729,
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
        first_state: 1752,
        state_count: 1296,
        default_state: 2912,
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
        first_state: 3048,
        state_count: 1,
        default_state: 3048,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 3049,
        state_count: 1,
        default_state: 3049,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 3050,
        state_count: 1,
        default_state: 3050,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 3051,
        state_count: 8,
        default_state: 3051,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 3059,
        state_count: 8,
        default_state: 3059,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 3067,
        state_count: 8,
        default_state: 3068,
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
        name: "sign",
        first_state: 3075,
        state_count: 32,
        default_state: 3076,
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
        first_state: 3107,
        state_count: 64,
        default_state: 3118,
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
        first_state: 3171,
        state_count: 8,
        default_state: 3172,
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
        first_state: 3179,
        state_count: 10,
        default_state: 3179,
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
        first_state: 3189,
        state_count: 80,
        default_state: 3200,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "wall_sign",
        first_state: 3269,
        state_count: 8,
        default_state: 3270,
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
        first_state: 3277,
        state_count: 24,
        default_state: 3286,
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
        first_state: 3301,
        state_count: 2,
        default_state: 3302,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 3303,
        state_count: 64,
        default_state: 3314,
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
        first_state: 3367,
        state_count: 2,
        default_state: 3368,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 3369,
        state_count: 2,
        default_state: 3370,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 3371,
        state_count: 2,
        default_state: 3372,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 3373,
        state_count: 2,
        default_state: 3374,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 3375,
        state_count: 2,
        default_state: 3376,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 3377,
        state_count: 2,
        default_state: 3378,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 3379,
        state_count: 2,
        default_state: 3380,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 3381,
        state_count: 2,
        default_state: 3381,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 3383,
        state_count: 8,
        default_state: 3383,
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
        first_state: 3391,
        state_count: 24,
        default_state: 3400,
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
        first_state: 3415,
        state_count: 8,
        default_state: 3415,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 3423,
        state_count: 1,
        default_state: 3423,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 3424,
        state_count: 1,
        default_state: 3424,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 3425,
        state_count: 16,
        default_state: 3425,
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
        first_state: 3441,
        state_count: 1,
        default_state: 3441,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 3442,
        state_count: 16,
        default_state: 3442,
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
        first_state: 3458,
        state_count: 2,
        default_state: 3459,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 3460,
        state_count: 32,
        default_state: 3491,
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
        first_state: 3492,
        state_count: 1,
        default_state: 3492,
        props: &[],
    },
    BlockInfo {
        name: "netherrack",
        first_state: 3493,
        state_count: 1,
        default_state: 3493,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 3494,
        state_count: 1,
        default_state: 3494,
        props: &[],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 3495,
        state_count: 1,
        default_state: 3495,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 3496,
        state_count: 2,
        default_state: 3496,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 3498,
        state_count: 4,
        default_state: 3498,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 3502,
        state_count: 4,
        default_state: 3502,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 3506,
        state_count: 7,
        default_state: 3506,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 3513,
        state_count: 64,
        default_state: 3516,
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
        first_state: 3577,
        state_count: 1,
        default_state: 3577,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 3578,
        state_count: 1,
        default_state: 3578,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 3579,
        state_count: 1,
        default_state: 3579,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 3580,
        state_count: 1,
        default_state: 3580,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 3581,
        state_count: 1,
        default_state: 3581,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 3582,
        state_count: 1,
        default_state: 3582,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 3583,
        state_count: 1,
        default_state: 3583,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 3584,
        state_count: 1,
        default_state: 3584,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 3585,
        state_count: 1,
        default_state: 3585,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 3586,
        state_count: 1,
        default_state: 3586,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 3587,
        state_count: 1,
        default_state: 3587,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 3588,
        state_count: 1,
        default_state: 3588,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 3589,
        state_count: 1,
        default_state: 3589,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 3590,
        state_count: 1,
        default_state: 3590,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 3591,
        state_count: 1,
        default_state: 3591,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 3592,
        state_count: 1,
        default_state: 3592,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 3593,
        state_count: 64,
        default_state: 3608,
        props: &[
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
        first_state: 3657,
        state_count: 64,
        default_state: 3672,
        props: &[
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
        first_state: 3721,
        state_count: 64,
        default_state: 3736,
        props: &[
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
        first_state: 3785,
        state_count: 64,
        default_state: 3800,
        props: &[
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
        first_state: 3849,
        state_count: 64,
        default_state: 3864,
        props: &[
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
        first_state: 3913,
        state_count: 64,
        default_state: 3928,
        props: &[
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
        name: "infested_stone",
        first_state: 3977,
        state_count: 1,
        default_state: 3977,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 3978,
        state_count: 1,
        default_state: 3978,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 3979,
        state_count: 1,
        default_state: 3979,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 3980,
        state_count: 1,
        default_state: 3980,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 3981,
        state_count: 1,
        default_state: 3981,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 3982,
        state_count: 1,
        default_state: 3982,
        props: &[],
    },
    BlockInfo {
        name: "stone_bricks",
        first_state: 3983,
        state_count: 1,
        default_state: 3983,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 3984,
        state_count: 1,
        default_state: 3984,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 3985,
        state_count: 1,
        default_state: 3985,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 3986,
        state_count: 1,
        default_state: 3986,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 3987,
        state_count: 64,
        default_state: 3987,
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
        first_state: 4051,
        state_count: 64,
        default_state: 4051,
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
        first_state: 4115,
        state_count: 64,
        default_state: 4115,
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
        first_state: 4179,
        state_count: 32,
        default_state: 4210,
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
        first_state: 4211,
        state_count: 32,
        default_state: 4242,
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
        first_state: 4243,
        state_count: 1,
        default_state: 4243,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 4244,
        state_count: 4,
        default_state: 4244,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 4248,
        state_count: 4,
        default_state: 4248,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 4252,
        state_count: 8,
        default_state: 4252,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 4260,
        state_count: 8,
        default_state: 4260,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 4268,
        state_count: 32,
        default_state: 4299,
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
        first_state: 4300,
        state_count: 32,
        default_state: 4307,
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
        first_state: 4332,
        state_count: 80,
        default_state: 4343,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4412,
        state_count: 80,
        default_state: 4423,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4492,
        state_count: 2,
        default_state: 4493,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 4494,
        state_count: 1,
        default_state: 4494,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 4495,
        state_count: 1,
        default_state: 4495,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 4496,
        state_count: 32,
        default_state: 4527,
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
        first_state: 4528,
        state_count: 80,
        default_state: 4539,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4608,
        state_count: 4,
        default_state: 4608,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 4612,
        state_count: 1,
        default_state: 4612,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 4613,
        state_count: 8,
        default_state: 4620,
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
        first_state: 4621,
        state_count: 4,
        default_state: 4621,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 4625,
        state_count: 1,
        default_state: 4625,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 4626,
        state_count: 8,
        default_state: 4630,
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
        first_state: 4634,
        state_count: 1,
        default_state: 4634,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 4635,
        state_count: 1,
        default_state: 4635,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 4636,
        state_count: 2,
        default_state: 4637,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 4638,
        state_count: 12,
        default_state: 4638,
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
        first_state: 4650,
        state_count: 80,
        default_state: 4661,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4730,
        state_count: 1,
        default_state: 4730,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 4731,
        state_count: 8,
        default_state: 4732,
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
        first_state: 4739,
        state_count: 16,
        default_state: 4748,
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
        first_state: 4755,
        state_count: 128,
        default_state: 4882,
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
        first_state: 4883,
        state_count: 1,
        default_state: 4883,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 4884,
        state_count: 80,
        default_state: 4895,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4964,
        state_count: 80,
        default_state: 4975,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 5044,
        state_count: 80,
        default_state: 5055,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 5124,
        state_count: 12,
        default_state: 5130,
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
        first_state: 5136,
        state_count: 1,
        default_state: 5136,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 5137,
        state_count: 64,
        default_state: 5196,
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
        first_state: 5201,
        state_count: 64,
        default_state: 5260,
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
        first_state: 5265,
        state_count: 1,
        default_state: 5265,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 5266,
        state_count: 1,
        default_state: 5266,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 5267,
        state_count: 1,
        default_state: 5267,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 5268,
        state_count: 1,
        default_state: 5268,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 5269,
        state_count: 1,
        default_state: 5269,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 5270,
        state_count: 1,
        default_state: 5270,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 5271,
        state_count: 1,
        default_state: 5271,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 5272,
        state_count: 1,
        default_state: 5272,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 5273,
        state_count: 1,
        default_state: 5273,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 5274,
        state_count: 1,
        default_state: 5274,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 5275,
        state_count: 1,
        default_state: 5275,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 5276,
        state_count: 1,
        default_state: 5276,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 5277,
        state_count: 1,
        default_state: 5277,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 5278,
        state_count: 1,
        default_state: 5278,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 5279,
        state_count: 1,
        default_state: 5279,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 5280,
        state_count: 1,
        default_state: 5280,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 5281,
        state_count: 1,
        default_state: 5281,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 5282,
        state_count: 1,
        default_state: 5282,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 5283,
        state_count: 1,
        default_state: 5283,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 5284,
        state_count: 1,
        default_state: 5284,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 5285,
        state_count: 1,
        default_state: 5285,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 5286,
        state_count: 1,
        default_state: 5286,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 5287,
        state_count: 8,
        default_state: 5287,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 5295,
        state_count: 8,
        default_state: 5295,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 5303,
        state_count: 24,
        default_state: 5312,
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
        first_state: 5327,
        state_count: 24,
        default_state: 5336,
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
        first_state: 5351,
        state_count: 24,
        default_state: 5360,
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
        first_state: 5375,
        state_count: 24,
        default_state: 5384,
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
        first_state: 5399,
        state_count: 24,
        default_state: 5408,
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
        first_state: 5423,
        state_count: 24,
        default_state: 5432,
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
        name: "skeleton_wall_skull",
        first_state: 5447,
        state_count: 4,
        default_state: 5447,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "skeleton_skull",
        first_state: 5451,
        state_count: 16,
        default_state: 5451,
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
        first_state: 5467,
        state_count: 4,
        default_state: 5467,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "wither_skeleton_skull",
        first_state: 5471,
        state_count: 16,
        default_state: 5471,
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
        first_state: 5487,
        state_count: 4,
        default_state: 5487,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "zombie_head",
        first_state: 5491,
        state_count: 16,
        default_state: 5491,
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
        first_state: 5507,
        state_count: 4,
        default_state: 5507,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "player_head",
        first_state: 5511,
        state_count: 16,
        default_state: 5511,
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
        first_state: 5527,
        state_count: 4,
        default_state: 5527,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "creeper_head",
        first_state: 5531,
        state_count: 16,
        default_state: 5531,
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
        first_state: 5547,
        state_count: 4,
        default_state: 5547,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "dragon_head",
        first_state: 5551,
        state_count: 16,
        default_state: 5551,
        props: &[PropInfo {
            name: "rotation",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "anvil",
        first_state: 5567,
        state_count: 4,
        default_state: 5567,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 5571,
        state_count: 4,
        default_state: 5571,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 5575,
        state_count: 4,
        default_state: 5575,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 5579,
        state_count: 24,
        default_state: 5580,
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
        first_state: 5603,
        state_count: 16,
        default_state: 5603,
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
        first_state: 5619,
        state_count: 16,
        default_state: 5619,
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
        first_state: 5635,
        state_count: 16,
        default_state: 5636,
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
        first_state: 5651,
        state_count: 32,
        default_state: 5667,
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
        first_state: 5683,
        state_count: 1,
        default_state: 5683,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 5684,
        state_count: 1,
        default_state: 5684,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 5685,
        state_count: 10,
        default_state: 5685,
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
        first_state: 5695,
        state_count: 1,
        default_state: 5695,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 5696,
        state_count: 1,
        default_state: 5696,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 5697,
        state_count: 3,
        default_state: 5698,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 5700,
        state_count: 80,
        default_state: 5711,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 5780,
        state_count: 12,
        default_state: 5786,
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
        first_state: 5792,
        state_count: 12,
        default_state: 5793,
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
        first_state: 5804,
        state_count: 1,
        default_state: 5804,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 5805,
        state_count: 1,
        default_state: 5805,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 5806,
        state_count: 1,
        default_state: 5806,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 5807,
        state_count: 1,
        default_state: 5807,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 5808,
        state_count: 1,
        default_state: 5808,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 5809,
        state_count: 1,
        default_state: 5809,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 5810,
        state_count: 1,
        default_state: 5810,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 5811,
        state_count: 1,
        default_state: 5811,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 5812,
        state_count: 1,
        default_state: 5812,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 5813,
        state_count: 1,
        default_state: 5813,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 5814,
        state_count: 1,
        default_state: 5814,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 5815,
        state_count: 1,
        default_state: 5815,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 5816,
        state_count: 1,
        default_state: 5816,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 5817,
        state_count: 1,
        default_state: 5817,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 5818,
        state_count: 1,
        default_state: 5818,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 5819,
        state_count: 1,
        default_state: 5819,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 5820,
        state_count: 32,
        default_state: 5851,
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
        first_state: 5852,
        state_count: 32,
        default_state: 5883,
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
        first_state: 5884,
        state_count: 32,
        default_state: 5915,
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
        first_state: 5916,
        state_count: 32,
        default_state: 5947,
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
        first_state: 5948,
        state_count: 32,
        default_state: 5979,
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
        first_state: 5980,
        state_count: 32,
        default_state: 6011,
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
        first_state: 6012,
        state_count: 32,
        default_state: 6043,
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
        first_state: 6044,
        state_count: 32,
        default_state: 6075,
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
        first_state: 6076,
        state_count: 32,
        default_state: 6107,
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
        first_state: 6108,
        state_count: 32,
        default_state: 6139,
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
        first_state: 6140,
        state_count: 32,
        default_state: 6171,
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
        first_state: 6172,
        state_count: 32,
        default_state: 6203,
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
        first_state: 6204,
        state_count: 32,
        default_state: 6235,
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
        first_state: 6236,
        state_count: 32,
        default_state: 6267,
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
        first_state: 6268,
        state_count: 32,
        default_state: 6299,
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
        first_state: 6300,
        state_count: 32,
        default_state: 6331,
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
        first_state: 6332,
        state_count: 80,
        default_state: 6343,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 6412,
        state_count: 80,
        default_state: 6423,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 6492,
        state_count: 1,
        default_state: 6492,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 6493,
        state_count: 1,
        default_state: 6493,
        props: &[],
    },
    BlockInfo {
        name: "iron_trapdoor",
        first_state: 6494,
        state_count: 64,
        default_state: 6509,
        props: &[
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
        first_state: 6558,
        state_count: 1,
        default_state: 6558,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 6559,
        state_count: 1,
        default_state: 6559,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 6560,
        state_count: 1,
        default_state: 6560,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 6561,
        state_count: 80,
        default_state: 6572,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 6641,
        state_count: 80,
        default_state: 6652,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 6721,
        state_count: 80,
        default_state: 6732,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 6801,
        state_count: 6,
        default_state: 6804,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 6807,
        state_count: 6,
        default_state: 6810,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 6813,
        state_count: 6,
        default_state: 6816,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 6819,
        state_count: 1,
        default_state: 6819,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 6820,
        state_count: 3,
        default_state: 6821,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 6823,
        state_count: 1,
        default_state: 6823,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 6824,
        state_count: 1,
        default_state: 6824,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 6825,
        state_count: 1,
        default_state: 6825,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 6826,
        state_count: 1,
        default_state: 6826,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 6827,
        state_count: 1,
        default_state: 6827,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 6828,
        state_count: 1,
        default_state: 6828,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 6829,
        state_count: 1,
        default_state: 6829,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 6830,
        state_count: 1,
        default_state: 6830,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 6831,
        state_count: 1,
        default_state: 6831,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 6832,
        state_count: 1,
        default_state: 6832,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 6833,
        state_count: 1,
        default_state: 6833,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 6834,
        state_count: 1,
        default_state: 6834,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 6835,
        state_count: 1,
        default_state: 6835,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 6836,
        state_count: 1,
        default_state: 6836,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 6837,
        state_count: 1,
        default_state: 6837,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 6838,
        state_count: 1,
        default_state: 6838,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 6839,
        state_count: 1,
        default_state: 6839,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 6840,
        state_count: 1,
        default_state: 6840,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 6841,
        state_count: 1,
        default_state: 6841,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 6842,
        state_count: 2,
        default_state: 6843,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 6844,
        state_count: 2,
        default_state: 6845,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 6846,
        state_count: 2,
        default_state: 6847,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 6848,
        state_count: 2,
        default_state: 6849,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 6850,
        state_count: 2,
        default_state: 6851,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 6852,
        state_count: 2,
        default_state: 6853,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 6854,
        state_count: 16,
        default_state: 6854,
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
        first_state: 6870,
        state_count: 16,
        default_state: 6870,
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
        first_state: 6886,
        state_count: 16,
        default_state: 6886,
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
        first_state: 6902,
        state_count: 16,
        default_state: 6902,
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
        first_state: 6918,
        state_count: 16,
        default_state: 6918,
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
        first_state: 6934,
        state_count: 16,
        default_state: 6934,
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
        first_state: 6950,
        state_count: 16,
        default_state: 6950,
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
        first_state: 6966,
        state_count: 16,
        default_state: 6966,
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
        first_state: 6982,
        state_count: 16,
        default_state: 6982,
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
        first_state: 6998,
        state_count: 16,
        default_state: 6998,
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
        first_state: 7014,
        state_count: 16,
        default_state: 7014,
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
        first_state: 7030,
        state_count: 16,
        default_state: 7030,
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
        first_state: 7046,
        state_count: 16,
        default_state: 7046,
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
        first_state: 7062,
        state_count: 16,
        default_state: 7062,
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
        first_state: 7078,
        state_count: 16,
        default_state: 7078,
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
        first_state: 7094,
        state_count: 16,
        default_state: 7094,
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
        first_state: 7110,
        state_count: 4,
        default_state: 7110,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 7114,
        state_count: 4,
        default_state: 7114,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 7118,
        state_count: 4,
        default_state: 7118,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 7122,
        state_count: 4,
        default_state: 7122,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 7126,
        state_count: 4,
        default_state: 7126,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 7130,
        state_count: 4,
        default_state: 7130,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 7134,
        state_count: 4,
        default_state: 7134,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 7138,
        state_count: 4,
        default_state: 7138,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 7142,
        state_count: 4,
        default_state: 7142,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 7146,
        state_count: 4,
        default_state: 7146,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 7150,
        state_count: 4,
        default_state: 7150,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 7154,
        state_count: 4,
        default_state: 7154,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 7158,
        state_count: 4,
        default_state: 7158,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 7162,
        state_count: 4,
        default_state: 7162,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 7166,
        state_count: 4,
        default_state: 7166,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 7170,
        state_count: 4,
        default_state: 7170,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 7174,
        state_count: 1,
        default_state: 7174,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 7175,
        state_count: 1,
        default_state: 7175,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 7176,
        state_count: 1,
        default_state: 7176,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 7177,
        state_count: 80,
        default_state: 7188,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7257,
        state_count: 6,
        default_state: 7260,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7263,
        state_count: 6,
        default_state: 7266,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7269,
        state_count: 6,
        default_state: 7272,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7275,
        state_count: 6,
        default_state: 7278,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7281,
        state_count: 6,
        default_state: 7284,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7287,
        state_count: 6,
        default_state: 7290,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7293,
        state_count: 6,
        default_state: 7296,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7299,
        state_count: 6,
        default_state: 7302,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7305,
        state_count: 6,
        default_state: 7308,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7311,
        state_count: 6,
        default_state: 7314,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7317,
        state_count: 6,
        default_state: 7320,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7323,
        state_count: 6,
        default_state: 7326,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7329,
        state_count: 6,
        default_state: 7332,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7335,
        state_count: 6,
        default_state: 7338,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7341,
        state_count: 6,
        default_state: 7344,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7347,
        state_count: 6,
        default_state: 7350,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 7353,
        state_count: 1,
        default_state: 7353,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 7354,
        state_count: 1,
        default_state: 7354,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 7355,
        state_count: 1,
        default_state: 7355,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 7356,
        state_count: 1,
        default_state: 7356,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 7357,
        state_count: 32,
        default_state: 7364,
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
        first_state: 7389,
        state_count: 32,
        default_state: 7396,
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
        first_state: 7421,
        state_count: 32,
        default_state: 7428,
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
        first_state: 7453,
        state_count: 32,
        default_state: 7460,
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
        first_state: 7485,
        state_count: 32,
        default_state: 7492,
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
        first_state: 7517,
        state_count: 32,
        default_state: 7548,
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
        first_state: 7549,
        state_count: 32,
        default_state: 7580,
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
        first_state: 7581,
        state_count: 32,
        default_state: 7612,
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
        first_state: 7613,
        state_count: 32,
        default_state: 7644,
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
        first_state: 7645,
        state_count: 32,
        default_state: 7676,
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
        first_state: 7677,
        state_count: 64,
        default_state: 7688,
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
        first_state: 7741,
        state_count: 64,
        default_state: 7752,
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
        first_state: 7805,
        state_count: 64,
        default_state: 7816,
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
        first_state: 7869,
        state_count: 64,
        default_state: 7880,
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
        first_state: 7933,
        state_count: 64,
        default_state: 7944,
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
        first_state: 7997,
        state_count: 6,
        default_state: 8001,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 8003,
        state_count: 64,
        default_state: 8066,
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
        first_state: 8067,
        state_count: 6,
        default_state: 8067,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 8073,
        state_count: 1,
        default_state: 8073,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 8074,
        state_count: 3,
        default_state: 8075,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 8077,
        state_count: 80,
        default_state: 8088,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8157,
        state_count: 1,
        default_state: 8157,
        props: &[],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 8158,
        state_count: 4,
        default_state: 8158,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "grass_path",
        first_state: 8162,
        state_count: 1,
        default_state: 8162,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 8163,
        state_count: 1,
        default_state: 8163,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 8164,
        state_count: 12,
        default_state: 8170,
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
        first_state: 8176,
        state_count: 12,
        default_state: 8182,
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
        first_state: 8188,
        state_count: 4,
        default_state: 8188,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 8192,
        state_count: 1,
        default_state: 8192,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 8193,
        state_count: 1,
        default_state: 8193,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 8194,
        state_count: 1,
        default_state: 8194,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 8195,
        state_count: 3,
        default_state: 8196,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 8198,
        state_count: 1,
        default_state: 8198,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 8199,
        state_count: 12,
        default_state: 8204,
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
        first_state: 8211,
        state_count: 6,
        default_state: 8215,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 8217,
        state_count: 6,
        default_state: 8221,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 8223,
        state_count: 6,
        default_state: 8227,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 8229,
        state_count: 6,
        default_state: 8233,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 8235,
        state_count: 6,
        default_state: 8239,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 8241,
        state_count: 6,
        default_state: 8245,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 8247,
        state_count: 6,
        default_state: 8251,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 8253,
        state_count: 6,
        default_state: 8257,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 8259,
        state_count: 6,
        default_state: 8263,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 8265,
        state_count: 6,
        default_state: 8269,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 8271,
        state_count: 6,
        default_state: 8275,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 8277,
        state_count: 6,
        default_state: 8281,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 8283,
        state_count: 6,
        default_state: 8287,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 8289,
        state_count: 6,
        default_state: 8293,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 8295,
        state_count: 6,
        default_state: 8299,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 8301,
        state_count: 6,
        default_state: 8305,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 8307,
        state_count: 6,
        default_state: 8311,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 8313,
        state_count: 4,
        default_state: 8313,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 8317,
        state_count: 4,
        default_state: 8317,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 8321,
        state_count: 4,
        default_state: 8321,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 8325,
        state_count: 4,
        default_state: 8325,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 8329,
        state_count: 4,
        default_state: 8329,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 8333,
        state_count: 4,
        default_state: 8333,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 8337,
        state_count: 4,
        default_state: 8337,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 8341,
        state_count: 4,
        default_state: 8341,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 8345,
        state_count: 4,
        default_state: 8345,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 8349,
        state_count: 4,
        default_state: 8349,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 8353,
        state_count: 4,
        default_state: 8353,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 8357,
        state_count: 4,
        default_state: 8357,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 8361,
        state_count: 4,
        default_state: 8361,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 8365,
        state_count: 4,
        default_state: 8365,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 8369,
        state_count: 4,
        default_state: 8369,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 8373,
        state_count: 4,
        default_state: 8373,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 8377,
        state_count: 1,
        default_state: 8377,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 8378,
        state_count: 1,
        default_state: 8378,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 8379,
        state_count: 1,
        default_state: 8379,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 8380,
        state_count: 1,
        default_state: 8380,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 8381,
        state_count: 1,
        default_state: 8381,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 8382,
        state_count: 1,
        default_state: 8382,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 8383,
        state_count: 1,
        default_state: 8383,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 8384,
        state_count: 1,
        default_state: 8384,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 8385,
        state_count: 1,
        default_state: 8385,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 8386,
        state_count: 1,
        default_state: 8386,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 8387,
        state_count: 1,
        default_state: 8387,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 8388,
        state_count: 1,
        default_state: 8388,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 8389,
        state_count: 1,
        default_state: 8389,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 8390,
        state_count: 1,
        default_state: 8390,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 8391,
        state_count: 1,
        default_state: 8391,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 8392,
        state_count: 1,
        default_state: 8392,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 8393,
        state_count: 1,
        default_state: 8393,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 8394,
        state_count: 1,
        default_state: 8394,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 8395,
        state_count: 1,
        default_state: 8395,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 8396,
        state_count: 1,
        default_state: 8396,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 8397,
        state_count: 1,
        default_state: 8397,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 8398,
        state_count: 1,
        default_state: 8398,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 8399,
        state_count: 1,
        default_state: 8399,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 8400,
        state_count: 1,
        default_state: 8400,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 8401,
        state_count: 1,
        default_state: 8401,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 8402,
        state_count: 1,
        default_state: 8402,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 8403,
        state_count: 1,
        default_state: 8403,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 8404,
        state_count: 1,
        default_state: 8404,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 8405,
        state_count: 1,
        default_state: 8405,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 8406,
        state_count: 1,
        default_state: 8406,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 8407,
        state_count: 1,
        default_state: 8407,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 8408,
        state_count: 1,
        default_state: 8408,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 8409,
        state_count: 26,
        default_state: 8409,
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
        first_state: 8435,
        state_count: 1,
        default_state: 8435,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 8436,
        state_count: 1,
        default_state: 8436,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 8437,
        state_count: 12,
        default_state: 8437,
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
        first_state: 8449,
        state_count: 1,
        default_state: 8449,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 8450,
        state_count: 1,
        default_state: 8450,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 8451,
        state_count: 1,
        default_state: 8451,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 8452,
        state_count: 1,
        default_state: 8452,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 8453,
        state_count: 1,
        default_state: 8453,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 8454,
        state_count: 1,
        default_state: 8454,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 8455,
        state_count: 1,
        default_state: 8455,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 8456,
        state_count: 1,
        default_state: 8456,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 8457,
        state_count: 1,
        default_state: 8457,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 8458,
        state_count: 1,
        default_state: 8458,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 8459,
        state_count: 1,
        default_state: 8459,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 8460,
        state_count: 1,
        default_state: 8460,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 8461,
        state_count: 1,
        default_state: 8461,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 8462,
        state_count: 1,
        default_state: 8462,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 8463,
        state_count: 1,
        default_state: 8463,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 8464,
        state_count: 8,
        default_state: 8464,
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
        first_state: 8472,
        state_count: 8,
        default_state: 8472,
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
        first_state: 8480,
        state_count: 8,
        default_state: 8480,
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
        first_state: 8488,
        state_count: 8,
        default_state: 8488,
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
        first_state: 8496,
        state_count: 8,
        default_state: 8496,
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
        first_state: 8504,
        state_count: 8,
        default_state: 8504,
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
        first_state: 8512,
        state_count: 8,
        default_state: 8512,
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
        first_state: 8520,
        state_count: 8,
        default_state: 8520,
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
        first_state: 8528,
        state_count: 8,
        default_state: 8528,
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
        first_state: 8536,
        state_count: 8,
        default_state: 8536,
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
        name: "dead_tube_coral_fan",
        first_state: 8544,
        state_count: 2,
        default_state: 8544,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 8546,
        state_count: 2,
        default_state: 8546,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 8548,
        state_count: 2,
        default_state: 8548,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 8550,
        state_count: 2,
        default_state: 8550,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 8552,
        state_count: 2,
        default_state: 8552,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 8554,
        state_count: 2,
        default_state: 8554,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 8556,
        state_count: 2,
        default_state: 8556,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 8558,
        state_count: 2,
        default_state: 8558,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 8560,
        state_count: 2,
        default_state: 8560,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 8562,
        state_count: 2,
        default_state: 8562,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sea_pickle",
        first_state: 8564,
        state_count: 8,
        default_state: 8564,
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
        first_state: 8572,
        state_count: 1,
        default_state: 8572,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 8573,
        state_count: 1,
        default_state: 8573,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 8574,
        state_count: 1,
        default_state: 8574,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 8575,
        state_count: 1,
        default_state: 8575,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 8576,
        state_count: 2,
        default_state: 8576,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "structure_block",
        first_state: 8578,
        state_count: 4,
        default_state: 8578,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
];
