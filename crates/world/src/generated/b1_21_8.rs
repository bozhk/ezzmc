// @generated `cargo xtask gen-blockstates 1.21.8` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.21.8";
pub const STATE_COUNT: u32 = 27946;

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
        name: "pale_oak_wood",
        first_state: 22,
        state_count: 3,
        default_state: 23,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pale_oak_planks",
        first_state: 25,
        state_count: 1,
        default_state: 25,
        props: &[],
    },
    BlockInfo {
        name: "mangrove_planks",
        first_state: 26,
        state_count: 1,
        default_state: 26,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_planks",
        first_state: 27,
        state_count: 1,
        default_state: 27,
        props: &[],
    },
    BlockInfo {
        name: "bamboo_mosaic",
        first_state: 28,
        state_count: 1,
        default_state: 28,
        props: &[],
    },
    BlockInfo {
        name: "oak_sapling",
        first_state: 29,
        state_count: 2,
        default_state: 29,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "spruce_sapling",
        first_state: 31,
        state_count: 2,
        default_state: 31,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "birch_sapling",
        first_state: 33,
        state_count: 2,
        default_state: 33,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "jungle_sapling",
        first_state: 35,
        state_count: 2,
        default_state: 35,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "acacia_sapling",
        first_state: 37,
        state_count: 2,
        default_state: 37,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "cherry_sapling",
        first_state: 39,
        state_count: 2,
        default_state: 39,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "dark_oak_sapling",
        first_state: 41,
        state_count: 2,
        default_state: 41,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "pale_oak_sapling",
        first_state: 43,
        state_count: 2,
        default_state: 43,
        props: &[PropInfo {
            name: "stage",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "mangrove_propagule",
        first_state: 45,
        state_count: 40,
        default_state: 50,
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
        first_state: 85,
        state_count: 1,
        default_state: 85,
        props: &[],
    },
    BlockInfo {
        name: "water",
        first_state: 86,
        state_count: 16,
        default_state: 86,
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
        first_state: 102,
        state_count: 16,
        default_state: 102,
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
        first_state: 118,
        state_count: 1,
        default_state: 118,
        props: &[],
    },
    BlockInfo {
        name: "suspicious_sand",
        first_state: 119,
        state_count: 4,
        default_state: 119,
        props: &[PropInfo {
            name: "dusted",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "red_sand",
        first_state: 123,
        state_count: 1,
        default_state: 123,
        props: &[],
    },
    BlockInfo {
        name: "gravel",
        first_state: 124,
        state_count: 1,
        default_state: 124,
        props: &[],
    },
    BlockInfo {
        name: "suspicious_gravel",
        first_state: 125,
        state_count: 4,
        default_state: 125,
        props: &[PropInfo {
            name: "dusted",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "gold_ore",
        first_state: 129,
        state_count: 1,
        default_state: 129,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_gold_ore",
        first_state: 130,
        state_count: 1,
        default_state: 130,
        props: &[],
    },
    BlockInfo {
        name: "iron_ore",
        first_state: 131,
        state_count: 1,
        default_state: 131,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_iron_ore",
        first_state: 132,
        state_count: 1,
        default_state: 132,
        props: &[],
    },
    BlockInfo {
        name: "coal_ore",
        first_state: 133,
        state_count: 1,
        default_state: 133,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_coal_ore",
        first_state: 134,
        state_count: 1,
        default_state: 134,
        props: &[],
    },
    BlockInfo {
        name: "nether_gold_ore",
        first_state: 135,
        state_count: 1,
        default_state: 135,
        props: &[],
    },
    BlockInfo {
        name: "oak_log",
        first_state: 136,
        state_count: 3,
        default_state: 137,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_log",
        first_state: 139,
        state_count: 3,
        default_state: 140,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_log",
        first_state: 142,
        state_count: 3,
        default_state: 143,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_log",
        first_state: 145,
        state_count: 3,
        default_state: 146,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_log",
        first_state: 148,
        state_count: 3,
        default_state: 149,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cherry_log",
        first_state: 151,
        state_count: 3,
        default_state: 152,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_log",
        first_state: 154,
        state_count: 3,
        default_state: 155,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pale_oak_log",
        first_state: 157,
        state_count: 3,
        default_state: 158,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_log",
        first_state: 160,
        state_count: 3,
        default_state: 161,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_roots",
        first_state: 163,
        state_count: 2,
        default_state: 164,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "muddy_mangrove_roots",
        first_state: 165,
        state_count: 3,
        default_state: 166,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "bamboo_block",
        first_state: 168,
        state_count: 3,
        default_state: 169,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_log",
        first_state: 171,
        state_count: 3,
        default_state: 172,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_log",
        first_state: 174,
        state_count: 3,
        default_state: 175,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_log",
        first_state: 177,
        state_count: 3,
        default_state: 178,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_log",
        first_state: 180,
        state_count: 3,
        default_state: 181,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_cherry_log",
        first_state: 183,
        state_count: 3,
        default_state: 184,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_log",
        first_state: 186,
        state_count: 3,
        default_state: 187,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_pale_oak_log",
        first_state: 189,
        state_count: 3,
        default_state: 190,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_log",
        first_state: 192,
        state_count: 3,
        default_state: 193,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_log",
        first_state: 195,
        state_count: 3,
        default_state: 196,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_bamboo_block",
        first_state: 198,
        state_count: 3,
        default_state: 199,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_wood",
        first_state: 201,
        state_count: 3,
        default_state: 202,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "spruce_wood",
        first_state: 204,
        state_count: 3,
        default_state: 205,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "birch_wood",
        first_state: 207,
        state_count: 3,
        default_state: 208,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "jungle_wood",
        first_state: 210,
        state_count: 3,
        default_state: 211,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "acacia_wood",
        first_state: 213,
        state_count: 3,
        default_state: 214,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cherry_wood",
        first_state: 216,
        state_count: 3,
        default_state: 217,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "dark_oak_wood",
        first_state: 219,
        state_count: 3,
        default_state: 220,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "mangrove_wood",
        first_state: 222,
        state_count: 3,
        default_state: 223,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_oak_wood",
        first_state: 225,
        state_count: 3,
        default_state: 226,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_spruce_wood",
        first_state: 228,
        state_count: 3,
        default_state: 229,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_birch_wood",
        first_state: 231,
        state_count: 3,
        default_state: 232,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_jungle_wood",
        first_state: 234,
        state_count: 3,
        default_state: 235,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_acacia_wood",
        first_state: 237,
        state_count: 3,
        default_state: 238,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_cherry_wood",
        first_state: 240,
        state_count: 3,
        default_state: 241,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_dark_oak_wood",
        first_state: 243,
        state_count: 3,
        default_state: 244,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_pale_oak_wood",
        first_state: 246,
        state_count: 3,
        default_state: 247,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_mangrove_wood",
        first_state: 249,
        state_count: 3,
        default_state: 250,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "oak_leaves",
        first_state: 252,
        state_count: 28,
        default_state: 279,
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
        first_state: 280,
        state_count: 28,
        default_state: 307,
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
        first_state: 308,
        state_count: 28,
        default_state: 335,
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
        first_state: 336,
        state_count: 28,
        default_state: 363,
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
        first_state: 364,
        state_count: 28,
        default_state: 391,
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
        first_state: 392,
        state_count: 28,
        default_state: 419,
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
        first_state: 420,
        state_count: 28,
        default_state: 447,
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
        name: "pale_oak_leaves",
        first_state: 448,
        state_count: 28,
        default_state: 475,
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
        first_state: 476,
        state_count: 28,
        default_state: 503,
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
        first_state: 504,
        state_count: 28,
        default_state: 531,
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
        first_state: 532,
        state_count: 28,
        default_state: 559,
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
        first_state: 560,
        state_count: 1,
        default_state: 560,
        props: &[],
    },
    BlockInfo {
        name: "wet_sponge",
        first_state: 561,
        state_count: 1,
        default_state: 561,
        props: &[],
    },
    BlockInfo {
        name: "glass",
        first_state: 562,
        state_count: 1,
        default_state: 562,
        props: &[],
    },
    BlockInfo {
        name: "lapis_ore",
        first_state: 563,
        state_count: 1,
        default_state: 563,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_lapis_ore",
        first_state: 564,
        state_count: 1,
        default_state: 564,
        props: &[],
    },
    BlockInfo {
        name: "lapis_block",
        first_state: 565,
        state_count: 1,
        default_state: 565,
        props: &[],
    },
    BlockInfo {
        name: "dispenser",
        first_state: 566,
        state_count: 12,
        default_state: 567,
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
        first_state: 578,
        state_count: 1,
        default_state: 578,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_sandstone",
        first_state: 579,
        state_count: 1,
        default_state: 579,
        props: &[],
    },
    BlockInfo {
        name: "cut_sandstone",
        first_state: 580,
        state_count: 1,
        default_state: 580,
        props: &[],
    },
    BlockInfo {
        name: "note_block",
        first_state: 581,
        state_count: 1150,
        default_state: 582,
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
        first_state: 1731,
        state_count: 16,
        default_state: 1734,
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
        first_state: 1747,
        state_count: 16,
        default_state: 1750,
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
        first_state: 1763,
        state_count: 16,
        default_state: 1766,
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
        first_state: 1779,
        state_count: 16,
        default_state: 1782,
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
        first_state: 1795,
        state_count: 16,
        default_state: 1798,
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
        first_state: 1811,
        state_count: 16,
        default_state: 1814,
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
        first_state: 1827,
        state_count: 16,
        default_state: 1830,
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
        first_state: 1843,
        state_count: 16,
        default_state: 1846,
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
        first_state: 1859,
        state_count: 16,
        default_state: 1862,
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
        first_state: 1875,
        state_count: 16,
        default_state: 1878,
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
        first_state: 1891,
        state_count: 16,
        default_state: 1894,
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
        first_state: 1907,
        state_count: 16,
        default_state: 1910,
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
        first_state: 1923,
        state_count: 16,
        default_state: 1926,
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
        first_state: 1939,
        state_count: 16,
        default_state: 1942,
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
        first_state: 1955,
        state_count: 16,
        default_state: 1958,
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
        first_state: 1971,
        state_count: 16,
        default_state: 1974,
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
        first_state: 1987,
        state_count: 24,
        default_state: 2000,
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
        first_state: 2011,
        state_count: 24,
        default_state: 2024,
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
        first_state: 2035,
        state_count: 12,
        default_state: 2041,
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
        first_state: 2047,
        state_count: 1,
        default_state: 2047,
        props: &[],
    },
    BlockInfo {
        name: "short_grass",
        first_state: 2048,
        state_count: 1,
        default_state: 2048,
        props: &[],
    },
    BlockInfo {
        name: "fern",
        first_state: 2049,
        state_count: 1,
        default_state: 2049,
        props: &[],
    },
    BlockInfo {
        name: "dead_bush",
        first_state: 2050,
        state_count: 1,
        default_state: 2050,
        props: &[],
    },
    BlockInfo {
        name: "bush",
        first_state: 2051,
        state_count: 1,
        default_state: 2051,
        props: &[],
    },
    BlockInfo {
        name: "short_dry_grass",
        first_state: 2052,
        state_count: 1,
        default_state: 2052,
        props: &[],
    },
    BlockInfo {
        name: "tall_dry_grass",
        first_state: 2053,
        state_count: 1,
        default_state: 2053,
        props: &[],
    },
    BlockInfo {
        name: "seagrass",
        first_state: 2054,
        state_count: 1,
        default_state: 2054,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 2055,
        state_count: 2,
        default_state: 2056,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 2057,
        state_count: 12,
        default_state: 2063,
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
        first_state: 2069,
        state_count: 24,
        default_state: 2071,
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
        first_state: 2093,
        state_count: 1,
        default_state: 2093,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 2094,
        state_count: 1,
        default_state: 2094,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 2095,
        state_count: 1,
        default_state: 2095,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 2096,
        state_count: 1,
        default_state: 2096,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 2097,
        state_count: 1,
        default_state: 2097,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 2098,
        state_count: 1,
        default_state: 2098,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 2099,
        state_count: 1,
        default_state: 2099,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 2100,
        state_count: 1,
        default_state: 2100,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 2101,
        state_count: 1,
        default_state: 2101,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 2102,
        state_count: 1,
        default_state: 2102,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 2103,
        state_count: 1,
        default_state: 2103,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 2104,
        state_count: 1,
        default_state: 2104,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 2105,
        state_count: 1,
        default_state: 2105,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 2106,
        state_count: 1,
        default_state: 2106,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 2107,
        state_count: 1,
        default_state: 2107,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 2108,
        state_count: 1,
        default_state: 2108,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 2109,
        state_count: 12,
        default_state: 2109,
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
        first_state: 2121,
        state_count: 1,
        default_state: 2121,
        props: &[],
    },
    BlockInfo {
        name: "torchflower",
        first_state: 2122,
        state_count: 1,
        default_state: 2122,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 2123,
        state_count: 1,
        default_state: 2123,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 2124,
        state_count: 1,
        default_state: 2124,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 2125,
        state_count: 1,
        default_state: 2125,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 2126,
        state_count: 1,
        default_state: 2126,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 2127,
        state_count: 1,
        default_state: 2127,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 2128,
        state_count: 1,
        default_state: 2128,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 2129,
        state_count: 1,
        default_state: 2129,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 2130,
        state_count: 1,
        default_state: 2130,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 2131,
        state_count: 1,
        default_state: 2131,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 2132,
        state_count: 1,
        default_state: 2132,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 2133,
        state_count: 1,
        default_state: 2133,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 2134,
        state_count: 1,
        default_state: 2134,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 2135,
        state_count: 1,
        default_state: 2135,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 2136,
        state_count: 1,
        default_state: 2136,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 2137,
        state_count: 1,
        default_state: 2137,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 2138,
        state_count: 1,
        default_state: 2138,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 2139,
        state_count: 1,
        default_state: 2139,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 2140,
        state_count: 2,
        default_state: 2141,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 2142,
        state_count: 1,
        default_state: 2142,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_bookshelf",
        first_state: 2143,
        state_count: 256,
        default_state: 2206,
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
        first_state: 2399,
        state_count: 1,
        default_state: 2399,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 2400,
        state_count: 1,
        default_state: 2400,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 2401,
        state_count: 1,
        default_state: 2401,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 2402,
        state_count: 4,
        default_state: 2402,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 2406,
        state_count: 512,
        default_state: 2437,
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
        first_state: 2918,
        state_count: 1,
        default_state: 2918,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 2919,
        state_count: 1,
        default_state: 2919,
        props: &[],
    },
    BlockInfo {
        name: "creaking_heart",
        first_state: 2920,
        state_count: 18,
        default_state: 2927,
        props: &[
            PropInfo {
                name: "axis",
                values: &["x", "y", "z"],
            },
            PropInfo {
                name: "creaking_heart_state",
                values: &["uprooted", "dormant", "awake"],
            },
            PropInfo {
                name: "natural",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 2938,
        state_count: 80,
        default_state: 2949,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 3018,
        state_count: 24,
        default_state: 3019,
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
        first_state: 3042,
        state_count: 1296,
        default_state: 4202,
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
        first_state: 4338,
        state_count: 1,
        default_state: 4338,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_diamond_ore",
        first_state: 4339,
        state_count: 1,
        default_state: 4339,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 4340,
        state_count: 1,
        default_state: 4340,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 4341,
        state_count: 1,
        default_state: 4341,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 4342,
        state_count: 8,
        default_state: 4342,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 4350,
        state_count: 8,
        default_state: 4350,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 4358,
        state_count: 8,
        default_state: 4359,
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
        first_state: 4366,
        state_count: 32,
        default_state: 4367,
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
        first_state: 4398,
        state_count: 32,
        default_state: 4399,
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
        first_state: 4430,
        state_count: 32,
        default_state: 4431,
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
        first_state: 4462,
        state_count: 32,
        default_state: 4463,
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
        first_state: 4494,
        state_count: 32,
        default_state: 4495,
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
        first_state: 4526,
        state_count: 32,
        default_state: 4527,
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
        first_state: 4558,
        state_count: 32,
        default_state: 4559,
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
        name: "pale_oak_sign",
        first_state: 4590,
        state_count: 32,
        default_state: 4591,
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
        first_state: 4622,
        state_count: 32,
        default_state: 4623,
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
        first_state: 4654,
        state_count: 32,
        default_state: 4655,
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
        first_state: 4686,
        state_count: 64,
        default_state: 4697,
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
        first_state: 4750,
        state_count: 8,
        default_state: 4751,
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
        first_state: 4758,
        state_count: 20,
        default_state: 4759,
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
        first_state: 4778,
        state_count: 80,
        default_state: 4789,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4858,
        state_count: 8,
        default_state: 4859,
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
        first_state: 4866,
        state_count: 8,
        default_state: 4867,
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
        first_state: 4874,
        state_count: 8,
        default_state: 4875,
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
        first_state: 4882,
        state_count: 8,
        default_state: 4883,
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
        first_state: 4890,
        state_count: 8,
        default_state: 4891,
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
        first_state: 4898,
        state_count: 8,
        default_state: 4899,
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
        first_state: 4906,
        state_count: 8,
        default_state: 4907,
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
        name: "pale_oak_wall_sign",
        first_state: 4914,
        state_count: 8,
        default_state: 4915,
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
        first_state: 4922,
        state_count: 8,
        default_state: 4923,
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
        first_state: 4930,
        state_count: 8,
        default_state: 4931,
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
        first_state: 4938,
        state_count: 64,
        default_state: 4971,
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
        first_state: 5002,
        state_count: 64,
        default_state: 5035,
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
        first_state: 5066,
        state_count: 64,
        default_state: 5099,
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
        first_state: 5130,
        state_count: 64,
        default_state: 5163,
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
        first_state: 5194,
        state_count: 64,
        default_state: 5227,
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
        first_state: 5258,
        state_count: 64,
        default_state: 5291,
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
        first_state: 5322,
        state_count: 64,
        default_state: 5355,
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
        name: "pale_oak_hanging_sign",
        first_state: 5386,
        state_count: 64,
        default_state: 5419,
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
        first_state: 5450,
        state_count: 64,
        default_state: 5483,
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
        first_state: 5514,
        state_count: 64,
        default_state: 5547,
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
        first_state: 5578,
        state_count: 64,
        default_state: 5611,
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
        first_state: 5642,
        state_count: 64,
        default_state: 5675,
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
        first_state: 5706,
        state_count: 8,
        default_state: 5707,
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
        first_state: 5714,
        state_count: 8,
        default_state: 5715,
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
        first_state: 5722,
        state_count: 8,
        default_state: 5723,
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
        first_state: 5730,
        state_count: 8,
        default_state: 5731,
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
        first_state: 5738,
        state_count: 8,
        default_state: 5739,
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
        first_state: 5746,
        state_count: 8,
        default_state: 5747,
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
        first_state: 5754,
        state_count: 8,
        default_state: 5755,
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
        name: "pale_oak_wall_hanging_sign",
        first_state: 5762,
        state_count: 8,
        default_state: 5763,
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
        first_state: 5770,
        state_count: 8,
        default_state: 5771,
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
        first_state: 5778,
        state_count: 8,
        default_state: 5779,
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
        first_state: 5786,
        state_count: 8,
        default_state: 5787,
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
        first_state: 5794,
        state_count: 8,
        default_state: 5795,
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
        first_state: 5802,
        state_count: 24,
        default_state: 5811,
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
        first_state: 5826,
        state_count: 2,
        default_state: 5827,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 5828,
        state_count: 64,
        default_state: 5839,
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
        first_state: 5892,
        state_count: 2,
        default_state: 5893,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 5894,
        state_count: 2,
        default_state: 5895,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 5896,
        state_count: 2,
        default_state: 5897,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 5898,
        state_count: 2,
        default_state: 5899,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 5900,
        state_count: 2,
        default_state: 5901,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cherry_pressure_plate",
        first_state: 5902,
        state_count: 2,
        default_state: 5903,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 5904,
        state_count: 2,
        default_state: 5905,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pale_oak_pressure_plate",
        first_state: 5906,
        state_count: 2,
        default_state: 5907,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "mangrove_pressure_plate",
        first_state: 5908,
        state_count: 2,
        default_state: 5909,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_pressure_plate",
        first_state: 5910,
        state_count: 2,
        default_state: 5911,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 5912,
        state_count: 2,
        default_state: 5913,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "deepslate_redstone_ore",
        first_state: 5914,
        state_count: 2,
        default_state: 5915,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 5916,
        state_count: 2,
        default_state: 5916,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 5918,
        state_count: 8,
        default_state: 5918,
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
        first_state: 5926,
        state_count: 24,
        default_state: 5935,
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
        first_state: 5950,
        state_count: 8,
        default_state: 5950,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 5958,
        state_count: 1,
        default_state: 5958,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 5959,
        state_count: 1,
        default_state: 5959,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 5960,
        state_count: 16,
        default_state: 5960,
        props: &[PropInfo {
            name: "age",
            values: &[
                "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                "15",
            ],
        }],
    },
    BlockInfo {
        name: "cactus_flower",
        first_state: 5976,
        state_count: 1,
        default_state: 5976,
        props: &[],
    },
    BlockInfo {
        name: "clay",
        first_state: 5977,
        state_count: 1,
        default_state: 5977,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 5978,
        state_count: 16,
        default_state: 5978,
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
        first_state: 5994,
        state_count: 2,
        default_state: 5995,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 5996,
        state_count: 32,
        default_state: 6027,
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
        name: "netherrack",
        first_state: 6028,
        state_count: 1,
        default_state: 6028,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 6029,
        state_count: 1,
        default_state: 6029,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 6030,
        state_count: 1,
        default_state: 6030,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 6031,
        state_count: 3,
        default_state: 6032,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 6034,
        state_count: 3,
        default_state: 6035,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 6037,
        state_count: 1,
        default_state: 6037,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 6038,
        state_count: 4,
        default_state: 6038,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 6042,
        state_count: 1,
        default_state: 6042,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 6043,
        state_count: 2,
        default_state: 6043,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 6045,
        state_count: 4,
        default_state: 6045,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 6049,
        state_count: 4,
        default_state: 6049,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 6053,
        state_count: 7,
        default_state: 6053,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 6060,
        state_count: 64,
        default_state: 6063,
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
        first_state: 6124,
        state_count: 1,
        default_state: 6124,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 6125,
        state_count: 1,
        default_state: 6125,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 6126,
        state_count: 1,
        default_state: 6126,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 6127,
        state_count: 1,
        default_state: 6127,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 6128,
        state_count: 1,
        default_state: 6128,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 6129,
        state_count: 1,
        default_state: 6129,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 6130,
        state_count: 1,
        default_state: 6130,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 6131,
        state_count: 1,
        default_state: 6131,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 6132,
        state_count: 1,
        default_state: 6132,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 6133,
        state_count: 1,
        default_state: 6133,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 6134,
        state_count: 1,
        default_state: 6134,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 6135,
        state_count: 1,
        default_state: 6135,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 6136,
        state_count: 1,
        default_state: 6136,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 6137,
        state_count: 1,
        default_state: 6137,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 6138,
        state_count: 1,
        default_state: 6138,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 6139,
        state_count: 1,
        default_state: 6139,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 6140,
        state_count: 64,
        default_state: 6155,
        props: &[
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
        first_state: 6204,
        state_count: 64,
        default_state: 6219,
        props: &[
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
        first_state: 6268,
        state_count: 64,
        default_state: 6283,
        props: &[
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
        first_state: 6332,
        state_count: 64,
        default_state: 6347,
        props: &[
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
        first_state: 6396,
        state_count: 64,
        default_state: 6411,
        props: &[
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
        first_state: 6460,
        state_count: 64,
        default_state: 6475,
        props: &[
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
        first_state: 6524,
        state_count: 64,
        default_state: 6539,
        props: &[
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
        name: "pale_oak_trapdoor",
        first_state: 6588,
        state_count: 64,
        default_state: 6603,
        props: &[
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
        first_state: 6652,
        state_count: 64,
        default_state: 6667,
        props: &[
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
        first_state: 6716,
        state_count: 64,
        default_state: 6731,
        props: &[
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
        first_state: 6780,
        state_count: 1,
        default_state: 6780,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 6781,
        state_count: 1,
        default_state: 6781,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 6782,
        state_count: 1,
        default_state: 6782,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 6783,
        state_count: 1,
        default_state: 6783,
        props: &[],
    },
    BlockInfo {
        name: "packed_mud",
        first_state: 6784,
        state_count: 1,
        default_state: 6784,
        props: &[],
    },
    BlockInfo {
        name: "mud_bricks",
        first_state: 6785,
        state_count: 1,
        default_state: 6785,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 6786,
        state_count: 1,
        default_state: 6786,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 6787,
        state_count: 1,
        default_state: 6787,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 6788,
        state_count: 1,
        default_state: 6788,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 6789,
        state_count: 1,
        default_state: 6789,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 6790,
        state_count: 1,
        default_state: 6790,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 6791,
        state_count: 1,
        default_state: 6791,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 6792,
        state_count: 64,
        default_state: 6792,
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
        first_state: 6856,
        state_count: 64,
        default_state: 6856,
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
        first_state: 6920,
        state_count: 64,
        default_state: 6920,
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
        first_state: 6984,
        state_count: 32,
        default_state: 7015,
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
        first_state: 7016,
        state_count: 6,
        default_state: 7019,
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
        first_state: 7022,
        state_count: 32,
        default_state: 7053,
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
        first_state: 7054,
        state_count: 1,
        default_state: 7054,
        props: &[],
    },
    BlockInfo {
        name: "melon",
        first_state: 7055,
        state_count: 1,
        default_state: 7055,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 7056,
        state_count: 4,
        default_state: 7056,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 7060,
        state_count: 4,
        default_state: 7060,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 7064,
        state_count: 8,
        default_state: 7064,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 7072,
        state_count: 8,
        default_state: 7072,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 7080,
        state_count: 32,
        default_state: 7111,
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
        first_state: 7112,
        state_count: 128,
        default_state: 7239,
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
        name: "resin_clump",
        first_state: 7240,
        state_count: 128,
        default_state: 7367,
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
        first_state: 7368,
        state_count: 32,
        default_state: 7375,
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
        first_state: 7400,
        state_count: 80,
        default_state: 7411,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7480,
        state_count: 80,
        default_state: 7491,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7560,
        state_count: 80,
        default_state: 7571,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7640,
        state_count: 2,
        default_state: 7641,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 7642,
        state_count: 1,
        default_state: 7642,
        props: &[],
    },
    BlockInfo {
        name: "resin_block",
        first_state: 7643,
        state_count: 1,
        default_state: 7643,
        props: &[],
    },
    BlockInfo {
        name: "resin_bricks",
        first_state: 7644,
        state_count: 1,
        default_state: 7644,
        props: &[],
    },
    BlockInfo {
        name: "resin_brick_stairs",
        first_state: 7645,
        state_count: 80,
        default_state: 7656,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "resin_brick_slab",
        first_state: 7725,
        state_count: 6,
        default_state: 7728,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "resin_brick_wall",
        first_state: 7731,
        state_count: 324,
        default_state: 7734,
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
        name: "chiseled_resin_bricks",
        first_state: 8055,
        state_count: 1,
        default_state: 8055,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 8056,
        state_count: 1,
        default_state: 8056,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 8057,
        state_count: 32,
        default_state: 8088,
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
        first_state: 8089,
        state_count: 80,
        default_state: 8100,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8169,
        state_count: 4,
        default_state: 8169,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 8173,
        state_count: 1,
        default_state: 8173,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 8174,
        state_count: 8,
        default_state: 8181,
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
        first_state: 8182,
        state_count: 1,
        default_state: 8182,
        props: &[],
    },
    BlockInfo {
        name: "water_cauldron",
        first_state: 8183,
        state_count: 3,
        default_state: 8183,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "lava_cauldron",
        first_state: 8186,
        state_count: 1,
        default_state: 8186,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow_cauldron",
        first_state: 8187,
        state_count: 3,
        default_state: 8187,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 8190,
        state_count: 1,
        default_state: 8190,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 8191,
        state_count: 8,
        default_state: 8195,
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
        first_state: 8199,
        state_count: 1,
        default_state: 8199,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 8200,
        state_count: 1,
        default_state: 8200,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 8201,
        state_count: 2,
        default_state: 8202,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 8203,
        state_count: 12,
        default_state: 8203,
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
        first_state: 8215,
        state_count: 80,
        default_state: 8226,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8295,
        state_count: 1,
        default_state: 8295,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_emerald_ore",
        first_state: 8296,
        state_count: 1,
        default_state: 8296,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 8297,
        state_count: 8,
        default_state: 8298,
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
        first_state: 8305,
        state_count: 16,
        default_state: 8314,
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
        first_state: 8321,
        state_count: 128,
        default_state: 8448,
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
        first_state: 8449,
        state_count: 1,
        default_state: 8449,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 8450,
        state_count: 80,
        default_state: 8461,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8530,
        state_count: 80,
        default_state: 8541,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8610,
        state_count: 80,
        default_state: 8621,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8690,
        state_count: 12,
        default_state: 8696,
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
        first_state: 8702,
        state_count: 1,
        default_state: 8702,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 8703,
        state_count: 324,
        default_state: 8706,
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
        first_state: 9027,
        state_count: 324,
        default_state: 9030,
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
        first_state: 9351,
        state_count: 1,
        default_state: 9351,
        props: &[],
    },
    BlockInfo {
        name: "potted_torchflower",
        first_state: 9352,
        state_count: 1,
        default_state: 9352,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 9353,
        state_count: 1,
        default_state: 9353,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 9354,
        state_count: 1,
        default_state: 9354,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 9355,
        state_count: 1,
        default_state: 9355,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 9356,
        state_count: 1,
        default_state: 9356,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 9357,
        state_count: 1,
        default_state: 9357,
        props: &[],
    },
    BlockInfo {
        name: "potted_cherry_sapling",
        first_state: 9358,
        state_count: 1,
        default_state: 9358,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 9359,
        state_count: 1,
        default_state: 9359,
        props: &[],
    },
    BlockInfo {
        name: "potted_pale_oak_sapling",
        first_state: 9360,
        state_count: 1,
        default_state: 9360,
        props: &[],
    },
    BlockInfo {
        name: "potted_mangrove_propagule",
        first_state: 9361,
        state_count: 1,
        default_state: 9361,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 9362,
        state_count: 1,
        default_state: 9362,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 9363,
        state_count: 1,
        default_state: 9363,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 9364,
        state_count: 1,
        default_state: 9364,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 9365,
        state_count: 1,
        default_state: 9365,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 9366,
        state_count: 1,
        default_state: 9366,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 9367,
        state_count: 1,
        default_state: 9367,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 9368,
        state_count: 1,
        default_state: 9368,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 9369,
        state_count: 1,
        default_state: 9369,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 9370,
        state_count: 1,
        default_state: 9370,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 9371,
        state_count: 1,
        default_state: 9371,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 9372,
        state_count: 1,
        default_state: 9372,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 9373,
        state_count: 1,
        default_state: 9373,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 9374,
        state_count: 1,
        default_state: 9374,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 9375,
        state_count: 1,
        default_state: 9375,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 9376,
        state_count: 1,
        default_state: 9376,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 9377,
        state_count: 1,
        default_state: 9377,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 9378,
        state_count: 1,
        default_state: 9378,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 9379,
        state_count: 1,
        default_state: 9379,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 9380,
        state_count: 8,
        default_state: 9380,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 9388,
        state_count: 8,
        default_state: 9388,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 9396,
        state_count: 24,
        default_state: 9405,
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
        first_state: 9420,
        state_count: 24,
        default_state: 9429,
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
        first_state: 9444,
        state_count: 24,
        default_state: 9453,
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
        first_state: 9468,
        state_count: 24,
        default_state: 9477,
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
        first_state: 9492,
        state_count: 24,
        default_state: 9501,
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
        first_state: 9516,
        state_count: 24,
        default_state: 9525,
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
        first_state: 9540,
        state_count: 24,
        default_state: 9549,
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
        name: "pale_oak_button",
        first_state: 9564,
        state_count: 24,
        default_state: 9573,
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
        first_state: 9588,
        state_count: 24,
        default_state: 9597,
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
        first_state: 9612,
        state_count: 24,
        default_state: 9621,
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
        first_state: 9636,
        state_count: 32,
        default_state: 9652,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "skeleton_wall_skull",
        first_state: 9668,
        state_count: 8,
        default_state: 9669,
        props: &[
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
        name: "wither_skeleton_skull",
        first_state: 9676,
        state_count: 32,
        default_state: 9692,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "wither_skeleton_wall_skull",
        first_state: 9708,
        state_count: 8,
        default_state: 9709,
        props: &[
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
        name: "zombie_head",
        first_state: 9716,
        state_count: 32,
        default_state: 9732,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "zombie_wall_head",
        first_state: 9748,
        state_count: 8,
        default_state: 9749,
        props: &[
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
        name: "player_head",
        first_state: 9756,
        state_count: 32,
        default_state: 9772,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "player_wall_head",
        first_state: 9788,
        state_count: 8,
        default_state: 9789,
        props: &[
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
        name: "creeper_head",
        first_state: 9796,
        state_count: 32,
        default_state: 9812,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "creeper_wall_head",
        first_state: 9828,
        state_count: 8,
        default_state: 9829,
        props: &[
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
        name: "dragon_head",
        first_state: 9836,
        state_count: 32,
        default_state: 9852,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "dragon_wall_head",
        first_state: 9868,
        state_count: 8,
        default_state: 9869,
        props: &[
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
        name: "piglin_head",
        first_state: 9876,
        state_count: 32,
        default_state: 9892,
        props: &[
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
            PropInfo {
                name: "rotation",
                values: &[
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15",
                ],
            },
        ],
    },
    BlockInfo {
        name: "piglin_wall_head",
        first_state: 9908,
        state_count: 8,
        default_state: 9909,
        props: &[
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
        name: "anvil",
        first_state: 9916,
        state_count: 4,
        default_state: 9916,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 9920,
        state_count: 4,
        default_state: 9920,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 9924,
        state_count: 4,
        default_state: 9924,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 9928,
        state_count: 24,
        default_state: 9929,
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
        first_state: 9952,
        state_count: 16,
        default_state: 9952,
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
        first_state: 9968,
        state_count: 16,
        default_state: 9968,
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
        first_state: 9984,
        state_count: 16,
        default_state: 9985,
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
        first_state: 10000,
        state_count: 32,
        default_state: 10016,
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
        first_state: 10032,
        state_count: 1,
        default_state: 10032,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 10033,
        state_count: 1,
        default_state: 10033,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 10034,
        state_count: 10,
        default_state: 10034,
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
        first_state: 10044,
        state_count: 1,
        default_state: 10044,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 10045,
        state_count: 1,
        default_state: 10045,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 10046,
        state_count: 3,
        default_state: 10047,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 10049,
        state_count: 80,
        default_state: 10060,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10129,
        state_count: 24,
        default_state: 10142,
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
        first_state: 10153,
        state_count: 12,
        default_state: 10154,
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
        first_state: 10165,
        state_count: 1,
        default_state: 10165,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 10166,
        state_count: 1,
        default_state: 10166,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 10167,
        state_count: 1,
        default_state: 10167,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 10168,
        state_count: 1,
        default_state: 10168,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 10169,
        state_count: 1,
        default_state: 10169,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 10170,
        state_count: 1,
        default_state: 10170,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 10171,
        state_count: 1,
        default_state: 10171,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 10172,
        state_count: 1,
        default_state: 10172,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 10173,
        state_count: 1,
        default_state: 10173,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 10174,
        state_count: 1,
        default_state: 10174,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 10175,
        state_count: 1,
        default_state: 10175,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 10176,
        state_count: 1,
        default_state: 10176,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 10177,
        state_count: 1,
        default_state: 10177,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 10178,
        state_count: 1,
        default_state: 10178,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 10179,
        state_count: 1,
        default_state: 10179,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 10180,
        state_count: 1,
        default_state: 10180,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 10181,
        state_count: 32,
        default_state: 10212,
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
        first_state: 10213,
        state_count: 32,
        default_state: 10244,
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
        first_state: 10245,
        state_count: 32,
        default_state: 10276,
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
        first_state: 10277,
        state_count: 32,
        default_state: 10308,
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
        first_state: 10309,
        state_count: 32,
        default_state: 10340,
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
        first_state: 10341,
        state_count: 32,
        default_state: 10372,
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
        first_state: 10373,
        state_count: 32,
        default_state: 10404,
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
        first_state: 10405,
        state_count: 32,
        default_state: 10436,
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
        first_state: 10437,
        state_count: 32,
        default_state: 10468,
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
        first_state: 10469,
        state_count: 32,
        default_state: 10500,
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
        first_state: 10501,
        state_count: 32,
        default_state: 10532,
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
        first_state: 10533,
        state_count: 32,
        default_state: 10564,
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
        first_state: 10565,
        state_count: 32,
        default_state: 10596,
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
        first_state: 10597,
        state_count: 32,
        default_state: 10628,
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
        first_state: 10629,
        state_count: 32,
        default_state: 10660,
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
        first_state: 10661,
        state_count: 32,
        default_state: 10692,
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
        first_state: 10693,
        state_count: 80,
        default_state: 10704,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10773,
        state_count: 80,
        default_state: 10784,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10853,
        state_count: 80,
        default_state: 10864,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pale_oak_stairs",
        first_state: 10933,
        state_count: 80,
        default_state: 10944,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11013,
        state_count: 80,
        default_state: 11024,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11093,
        state_count: 80,
        default_state: 11104,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11173,
        state_count: 80,
        default_state: 11184,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11253,
        state_count: 1,
        default_state: 11253,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 11254,
        state_count: 2,
        default_state: 11255,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light",
        first_state: 11256,
        state_count: 32,
        default_state: 11287,
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
        first_state: 11288,
        state_count: 64,
        default_state: 11303,
        props: &[
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
        first_state: 11352,
        state_count: 1,
        default_state: 11352,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 11353,
        state_count: 1,
        default_state: 11353,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 11354,
        state_count: 1,
        default_state: 11354,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 11355,
        state_count: 80,
        default_state: 11366,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11435,
        state_count: 80,
        default_state: 11446,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11515,
        state_count: 80,
        default_state: 11526,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11595,
        state_count: 6,
        default_state: 11598,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11601,
        state_count: 6,
        default_state: 11604,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11607,
        state_count: 6,
        default_state: 11610,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11613,
        state_count: 1,
        default_state: 11613,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 11614,
        state_count: 3,
        default_state: 11615,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 11617,
        state_count: 1,
        default_state: 11617,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 11618,
        state_count: 1,
        default_state: 11618,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 11619,
        state_count: 1,
        default_state: 11619,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 11620,
        state_count: 1,
        default_state: 11620,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 11621,
        state_count: 1,
        default_state: 11621,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 11622,
        state_count: 1,
        default_state: 11622,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 11623,
        state_count: 1,
        default_state: 11623,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 11624,
        state_count: 1,
        default_state: 11624,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 11625,
        state_count: 1,
        default_state: 11625,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 11626,
        state_count: 1,
        default_state: 11626,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 11627,
        state_count: 1,
        default_state: 11627,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 11628,
        state_count: 1,
        default_state: 11628,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 11629,
        state_count: 1,
        default_state: 11629,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 11630,
        state_count: 1,
        default_state: 11630,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 11631,
        state_count: 1,
        default_state: 11631,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 11632,
        state_count: 1,
        default_state: 11632,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 11633,
        state_count: 1,
        default_state: 11633,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 11634,
        state_count: 1,
        default_state: 11634,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 11635,
        state_count: 1,
        default_state: 11635,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 11636,
        state_count: 2,
        default_state: 11637,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 11638,
        state_count: 2,
        default_state: 11639,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 11640,
        state_count: 2,
        default_state: 11641,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 11642,
        state_count: 2,
        default_state: 11643,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 11644,
        state_count: 2,
        default_state: 11645,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 11646,
        state_count: 2,
        default_state: 11647,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 11648,
        state_count: 16,
        default_state: 11648,
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
        first_state: 11664,
        state_count: 16,
        default_state: 11664,
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
        first_state: 11680,
        state_count: 16,
        default_state: 11680,
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
        first_state: 11696,
        state_count: 16,
        default_state: 11696,
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
        first_state: 11712,
        state_count: 16,
        default_state: 11712,
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
        first_state: 11728,
        state_count: 16,
        default_state: 11728,
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
        first_state: 11744,
        state_count: 16,
        default_state: 11744,
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
        first_state: 11760,
        state_count: 16,
        default_state: 11760,
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
        first_state: 11776,
        state_count: 16,
        default_state: 11776,
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
        first_state: 11792,
        state_count: 16,
        default_state: 11792,
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
        first_state: 11808,
        state_count: 16,
        default_state: 11808,
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
        first_state: 11824,
        state_count: 16,
        default_state: 11824,
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
        first_state: 11840,
        state_count: 16,
        default_state: 11840,
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
        first_state: 11856,
        state_count: 16,
        default_state: 11856,
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
        first_state: 11872,
        state_count: 16,
        default_state: 11872,
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
        first_state: 11888,
        state_count: 16,
        default_state: 11888,
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
        first_state: 11904,
        state_count: 4,
        default_state: 11904,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 11908,
        state_count: 4,
        default_state: 11908,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 11912,
        state_count: 4,
        default_state: 11912,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 11916,
        state_count: 4,
        default_state: 11916,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 11920,
        state_count: 4,
        default_state: 11920,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 11924,
        state_count: 4,
        default_state: 11924,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 11928,
        state_count: 4,
        default_state: 11928,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 11932,
        state_count: 4,
        default_state: 11932,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 11936,
        state_count: 4,
        default_state: 11936,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 11940,
        state_count: 4,
        default_state: 11940,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 11944,
        state_count: 4,
        default_state: 11944,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 11948,
        state_count: 4,
        default_state: 11948,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 11952,
        state_count: 4,
        default_state: 11952,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 11956,
        state_count: 4,
        default_state: 11956,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 11960,
        state_count: 4,
        default_state: 11960,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 11964,
        state_count: 4,
        default_state: 11964,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 11968,
        state_count: 1,
        default_state: 11968,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 11969,
        state_count: 1,
        default_state: 11969,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 11970,
        state_count: 1,
        default_state: 11970,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 11971,
        state_count: 80,
        default_state: 11982,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 12051,
        state_count: 6,
        default_state: 12054,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12057,
        state_count: 6,
        default_state: 12060,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12063,
        state_count: 6,
        default_state: 12066,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12069,
        state_count: 6,
        default_state: 12072,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12075,
        state_count: 6,
        default_state: 12078,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12081,
        state_count: 6,
        default_state: 12084,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12087,
        state_count: 6,
        default_state: 12090,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "pale_oak_slab",
        first_state: 12093,
        state_count: 6,
        default_state: 12096,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12099,
        state_count: 6,
        default_state: 12102,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12105,
        state_count: 6,
        default_state: 12108,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12111,
        state_count: 6,
        default_state: 12114,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12117,
        state_count: 6,
        default_state: 12120,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12123,
        state_count: 6,
        default_state: 12126,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12129,
        state_count: 6,
        default_state: 12132,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12135,
        state_count: 6,
        default_state: 12138,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12141,
        state_count: 6,
        default_state: 12144,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12147,
        state_count: 6,
        default_state: 12150,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12153,
        state_count: 6,
        default_state: 12156,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12159,
        state_count: 6,
        default_state: 12162,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12165,
        state_count: 6,
        default_state: 12168,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12171,
        state_count: 6,
        default_state: 12174,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12177,
        state_count: 6,
        default_state: 12180,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12183,
        state_count: 6,
        default_state: 12186,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12189,
        state_count: 6,
        default_state: 12192,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12195,
        state_count: 6,
        default_state: 12198,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 12201,
        state_count: 1,
        default_state: 12201,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 12202,
        state_count: 1,
        default_state: 12202,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 12203,
        state_count: 1,
        default_state: 12203,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 12204,
        state_count: 1,
        default_state: 12204,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 12205,
        state_count: 32,
        default_state: 12212,
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
        first_state: 12237,
        state_count: 32,
        default_state: 12244,
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
        first_state: 12269,
        state_count: 32,
        default_state: 12276,
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
        first_state: 12301,
        state_count: 32,
        default_state: 12308,
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
        first_state: 12333,
        state_count: 32,
        default_state: 12340,
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
        first_state: 12365,
        state_count: 32,
        default_state: 12372,
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
        name: "pale_oak_fence_gate",
        first_state: 12397,
        state_count: 32,
        default_state: 12404,
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
        first_state: 12429,
        state_count: 32,
        default_state: 12436,
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
        first_state: 12461,
        state_count: 32,
        default_state: 12468,
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
        first_state: 12493,
        state_count: 32,
        default_state: 12524,
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
        first_state: 12525,
        state_count: 32,
        default_state: 12556,
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
        first_state: 12557,
        state_count: 32,
        default_state: 12588,
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
        first_state: 12589,
        state_count: 32,
        default_state: 12620,
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
        first_state: 12621,
        state_count: 32,
        default_state: 12652,
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
        first_state: 12653,
        state_count: 32,
        default_state: 12684,
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
        name: "pale_oak_fence",
        first_state: 12685,
        state_count: 32,
        default_state: 12716,
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
        first_state: 12717,
        state_count: 32,
        default_state: 12748,
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
        first_state: 12749,
        state_count: 32,
        default_state: 12780,
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
        first_state: 12781,
        state_count: 64,
        default_state: 12792,
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
        first_state: 12845,
        state_count: 64,
        default_state: 12856,
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
        first_state: 12909,
        state_count: 64,
        default_state: 12920,
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
        first_state: 12973,
        state_count: 64,
        default_state: 12984,
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
        first_state: 13037,
        state_count: 64,
        default_state: 13048,
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
        first_state: 13101,
        state_count: 64,
        default_state: 13112,
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
        name: "pale_oak_door",
        first_state: 13165,
        state_count: 64,
        default_state: 13176,
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
        first_state: 13229,
        state_count: 64,
        default_state: 13240,
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
        first_state: 13293,
        state_count: 64,
        default_state: 13304,
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
        first_state: 13357,
        state_count: 6,
        default_state: 13361,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 13363,
        state_count: 64,
        default_state: 13426,
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
        first_state: 13427,
        state_count: 6,
        default_state: 13427,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 13433,
        state_count: 1,
        default_state: 13433,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 13434,
        state_count: 3,
        default_state: 13435,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
        first_state: 13437,
        state_count: 80,
        default_state: 13448,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13517,
        state_count: 1,
        default_state: 13517,
        props: &[],
    },
    BlockInfo {
        name: "torchflower_crop",
        first_state: 13518,
        state_count: 2,
        default_state: 13518,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "pitcher_crop",
        first_state: 13520,
        state_count: 10,
        default_state: 13521,
        props: &[
            PropInfo {
                name: "age",
                values: &["0", "1", "2", "3", "4"],
            },
            PropInfo {
                name: "half",
                values: &["upper", "lower"],
            },
        ],
    },
    BlockInfo {
        name: "pitcher_plant",
        first_state: 13530,
        state_count: 2,
        default_state: 13531,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 13532,
        state_count: 4,
        default_state: 13532,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "dirt_path",
        first_state: 13536,
        state_count: 1,
        default_state: 13536,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 13537,
        state_count: 1,
        default_state: 13537,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 13538,
        state_count: 12,
        default_state: 13544,
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
        first_state: 13550,
        state_count: 12,
        default_state: 13556,
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
        first_state: 13562,
        state_count: 4,
        default_state: 13562,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 13566,
        state_count: 1,
        default_state: 13566,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 13567,
        state_count: 1,
        default_state: 13567,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 13568,
        state_count: 1,
        default_state: 13568,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 13569,
        state_count: 3,
        default_state: 13570,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 13572,
        state_count: 1,
        default_state: 13572,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 13573,
        state_count: 12,
        default_state: 13578,
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
        first_state: 13585,
        state_count: 6,
        default_state: 13589,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 13591,
        state_count: 6,
        default_state: 13595,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 13597,
        state_count: 6,
        default_state: 13601,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 13603,
        state_count: 6,
        default_state: 13607,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 13609,
        state_count: 6,
        default_state: 13613,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 13615,
        state_count: 6,
        default_state: 13619,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 13621,
        state_count: 6,
        default_state: 13625,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 13627,
        state_count: 6,
        default_state: 13631,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 13633,
        state_count: 6,
        default_state: 13637,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 13639,
        state_count: 6,
        default_state: 13643,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 13645,
        state_count: 6,
        default_state: 13649,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 13651,
        state_count: 6,
        default_state: 13655,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 13657,
        state_count: 6,
        default_state: 13661,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 13663,
        state_count: 6,
        default_state: 13667,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 13669,
        state_count: 6,
        default_state: 13673,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 13675,
        state_count: 6,
        default_state: 13679,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 13681,
        state_count: 6,
        default_state: 13685,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 13687,
        state_count: 4,
        default_state: 13687,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 13691,
        state_count: 4,
        default_state: 13691,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 13695,
        state_count: 4,
        default_state: 13695,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 13699,
        state_count: 4,
        default_state: 13699,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 13703,
        state_count: 4,
        default_state: 13703,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 13707,
        state_count: 4,
        default_state: 13707,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 13711,
        state_count: 4,
        default_state: 13711,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 13715,
        state_count: 4,
        default_state: 13715,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 13719,
        state_count: 4,
        default_state: 13719,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 13723,
        state_count: 4,
        default_state: 13723,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 13727,
        state_count: 4,
        default_state: 13727,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 13731,
        state_count: 4,
        default_state: 13731,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 13735,
        state_count: 4,
        default_state: 13735,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 13739,
        state_count: 4,
        default_state: 13739,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 13743,
        state_count: 4,
        default_state: 13743,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 13747,
        state_count: 4,
        default_state: 13747,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 13751,
        state_count: 1,
        default_state: 13751,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 13752,
        state_count: 1,
        default_state: 13752,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 13753,
        state_count: 1,
        default_state: 13753,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 13754,
        state_count: 1,
        default_state: 13754,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 13755,
        state_count: 1,
        default_state: 13755,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 13756,
        state_count: 1,
        default_state: 13756,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 13757,
        state_count: 1,
        default_state: 13757,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 13758,
        state_count: 1,
        default_state: 13758,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 13759,
        state_count: 1,
        default_state: 13759,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 13760,
        state_count: 1,
        default_state: 13760,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 13761,
        state_count: 1,
        default_state: 13761,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 13762,
        state_count: 1,
        default_state: 13762,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 13763,
        state_count: 1,
        default_state: 13763,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 13764,
        state_count: 1,
        default_state: 13764,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 13765,
        state_count: 1,
        default_state: 13765,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 13766,
        state_count: 1,
        default_state: 13766,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 13767,
        state_count: 1,
        default_state: 13767,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 13768,
        state_count: 1,
        default_state: 13768,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 13769,
        state_count: 1,
        default_state: 13769,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 13770,
        state_count: 1,
        default_state: 13770,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 13771,
        state_count: 1,
        default_state: 13771,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 13772,
        state_count: 1,
        default_state: 13772,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 13773,
        state_count: 1,
        default_state: 13773,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 13774,
        state_count: 1,
        default_state: 13774,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 13775,
        state_count: 1,
        default_state: 13775,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 13776,
        state_count: 1,
        default_state: 13776,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 13777,
        state_count: 1,
        default_state: 13777,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 13778,
        state_count: 1,
        default_state: 13778,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 13779,
        state_count: 1,
        default_state: 13779,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 13780,
        state_count: 1,
        default_state: 13780,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 13781,
        state_count: 1,
        default_state: 13781,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 13782,
        state_count: 1,
        default_state: 13782,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 13783,
        state_count: 26,
        default_state: 13783,
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
        first_state: 13809,
        state_count: 1,
        default_state: 13809,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 13810,
        state_count: 1,
        default_state: 13810,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 13811,
        state_count: 12,
        default_state: 13811,
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
        name: "sniffer_egg",
        first_state: 13823,
        state_count: 3,
        default_state: 13823,
        props: &[PropInfo {
            name: "hatch",
            values: &["0", "1", "2"],
        }],
    },
    BlockInfo {
        name: "dried_ghast",
        first_state: 13826,
        state_count: 32,
        default_state: 13827,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "hydration",
                values: &["0", "1", "2", "3"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "dead_tube_coral_block",
        first_state: 13858,
        state_count: 1,
        default_state: 13858,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 13859,
        state_count: 1,
        default_state: 13859,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 13860,
        state_count: 1,
        default_state: 13860,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 13861,
        state_count: 1,
        default_state: 13861,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 13862,
        state_count: 1,
        default_state: 13862,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 13863,
        state_count: 1,
        default_state: 13863,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 13864,
        state_count: 1,
        default_state: 13864,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 13865,
        state_count: 1,
        default_state: 13865,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 13866,
        state_count: 1,
        default_state: 13866,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 13867,
        state_count: 1,
        default_state: 13867,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 13868,
        state_count: 2,
        default_state: 13868,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 13870,
        state_count: 2,
        default_state: 13870,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 13872,
        state_count: 2,
        default_state: 13872,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 13874,
        state_count: 2,
        default_state: 13874,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 13876,
        state_count: 2,
        default_state: 13876,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 13878,
        state_count: 2,
        default_state: 13878,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 13880,
        state_count: 2,
        default_state: 13880,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 13882,
        state_count: 2,
        default_state: 13882,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 13884,
        state_count: 2,
        default_state: 13884,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 13886,
        state_count: 2,
        default_state: 13886,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 13888,
        state_count: 2,
        default_state: 13888,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 13890,
        state_count: 2,
        default_state: 13890,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 13892,
        state_count: 2,
        default_state: 13892,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 13894,
        state_count: 2,
        default_state: 13894,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 13896,
        state_count: 2,
        default_state: 13896,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 13898,
        state_count: 2,
        default_state: 13898,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 13900,
        state_count: 2,
        default_state: 13900,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 13902,
        state_count: 2,
        default_state: 13902,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 13904,
        state_count: 2,
        default_state: 13904,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 13906,
        state_count: 2,
        default_state: 13906,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 13908,
        state_count: 8,
        default_state: 13908,
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
        first_state: 13916,
        state_count: 8,
        default_state: 13916,
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
        first_state: 13924,
        state_count: 8,
        default_state: 13924,
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
        first_state: 13932,
        state_count: 8,
        default_state: 13932,
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
        first_state: 13940,
        state_count: 8,
        default_state: 13940,
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
        first_state: 13948,
        state_count: 8,
        default_state: 13948,
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
        first_state: 13956,
        state_count: 8,
        default_state: 13956,
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
        first_state: 13964,
        state_count: 8,
        default_state: 13964,
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
        first_state: 13972,
        state_count: 8,
        default_state: 13972,
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
        first_state: 13980,
        state_count: 8,
        default_state: 13980,
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
        first_state: 13988,
        state_count: 8,
        default_state: 13988,
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
        first_state: 13996,
        state_count: 1,
        default_state: 13996,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 13997,
        state_count: 2,
        default_state: 13997,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 13999,
        state_count: 1,
        default_state: 13999,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 14000,
        state_count: 12,
        default_state: 14000,
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
        first_state: 14012,
        state_count: 1,
        default_state: 14012,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 14013,
        state_count: 1,
        default_state: 14013,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 14014,
        state_count: 1,
        default_state: 14014,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 14015,
        state_count: 2,
        default_state: 14015,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 14017,
        state_count: 80,
        default_state: 14028,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14097,
        state_count: 80,
        default_state: 14108,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14177,
        state_count: 80,
        default_state: 14188,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14257,
        state_count: 80,
        default_state: 14268,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14337,
        state_count: 80,
        default_state: 14348,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14417,
        state_count: 80,
        default_state: 14428,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14497,
        state_count: 80,
        default_state: 14508,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14577,
        state_count: 80,
        default_state: 14588,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14657,
        state_count: 80,
        default_state: 14668,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14737,
        state_count: 80,
        default_state: 14748,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14817,
        state_count: 80,
        default_state: 14828,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14897,
        state_count: 80,
        default_state: 14908,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14977,
        state_count: 80,
        default_state: 14988,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 15057,
        state_count: 80,
        default_state: 15068,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 15137,
        state_count: 6,
        default_state: 15140,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15143,
        state_count: 6,
        default_state: 15146,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15149,
        state_count: 6,
        default_state: 15152,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15155,
        state_count: 6,
        default_state: 15158,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15161,
        state_count: 6,
        default_state: 15164,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15167,
        state_count: 6,
        default_state: 15170,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15173,
        state_count: 6,
        default_state: 15176,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15179,
        state_count: 6,
        default_state: 15182,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15185,
        state_count: 6,
        default_state: 15188,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15191,
        state_count: 6,
        default_state: 15194,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15197,
        state_count: 6,
        default_state: 15200,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15203,
        state_count: 6,
        default_state: 15206,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15209,
        state_count: 6,
        default_state: 15212,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 15215,
        state_count: 324,
        default_state: 15218,
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
        first_state: 15539,
        state_count: 324,
        default_state: 15542,
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
        first_state: 15863,
        state_count: 324,
        default_state: 15866,
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
        first_state: 16187,
        state_count: 324,
        default_state: 16190,
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
        first_state: 16511,
        state_count: 324,
        default_state: 16514,
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
        first_state: 16835,
        state_count: 324,
        default_state: 16838,
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
        first_state: 17159,
        state_count: 324,
        default_state: 17162,
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
        first_state: 17483,
        state_count: 324,
        default_state: 17486,
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
        first_state: 17807,
        state_count: 324,
        default_state: 17810,
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
        first_state: 18131,
        state_count: 324,
        default_state: 18134,
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
        first_state: 18455,
        state_count: 324,
        default_state: 18458,
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
        first_state: 18779,
        state_count: 324,
        default_state: 18782,
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
        first_state: 19103,
        state_count: 324,
        default_state: 19106,
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
        first_state: 19427,
        state_count: 32,
        default_state: 19458,
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
        first_state: 19459,
        state_count: 4,
        default_state: 19459,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 19463,
        state_count: 12,
        default_state: 19464,
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
        first_state: 19475,
        state_count: 8,
        default_state: 19476,
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
        first_state: 19483,
        state_count: 8,
        default_state: 19484,
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
        first_state: 19491,
        state_count: 1,
        default_state: 19491,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 19492,
        state_count: 1,
        default_state: 19492,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 19493,
        state_count: 12,
        default_state: 19497,
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
        first_state: 19505,
        state_count: 16,
        default_state: 19508,
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
        first_state: 19521,
        state_count: 1,
        default_state: 19521,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 19522,
        state_count: 4,
        default_state: 19522,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 19526,
        state_count: 32,
        default_state: 19527,
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
        first_state: 19558,
        state_count: 4,
        default_state: 19561,
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
        first_state: 19562,
        state_count: 4,
        default_state: 19565,
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
        first_state: 19566,
        state_count: 32,
        default_state: 19569,
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
        first_state: 19598,
        state_count: 32,
        default_state: 19601,
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
        first_state: 19630,
        state_count: 4,
        default_state: 19630,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 19634,
        state_count: 3,
        default_state: 19635,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 19637,
        state_count: 3,
        default_state: 19638,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 19640,
        state_count: 3,
        default_state: 19641,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 19643,
        state_count: 3,
        default_state: 19644,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 19646,
        state_count: 1,
        default_state: 19646,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 19647,
        state_count: 1,
        default_state: 19647,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 19648,
        state_count: 1,
        default_state: 19648,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 19649,
        state_count: 1,
        default_state: 19649,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 19650,
        state_count: 1,
        default_state: 19650,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 19651,
        state_count: 3,
        default_state: 19652,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 19654,
        state_count: 3,
        default_state: 19655,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 19657,
        state_count: 3,
        default_state: 19658,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 19660,
        state_count: 3,
        default_state: 19661,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 19663,
        state_count: 1,
        default_state: 19663,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 19664,
        state_count: 1,
        default_state: 19664,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 19665,
        state_count: 1,
        default_state: 19665,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 19666,
        state_count: 26,
        default_state: 19666,
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
        first_state: 19692,
        state_count: 1,
        default_state: 19692,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 19693,
        state_count: 26,
        default_state: 19693,
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
        first_state: 19719,
        state_count: 1,
        default_state: 19719,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 19720,
        state_count: 1,
        default_state: 19720,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 19721,
        state_count: 1,
        default_state: 19721,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 19722,
        state_count: 1,
        default_state: 19722,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 19723,
        state_count: 6,
        default_state: 19726,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 19729,
        state_count: 6,
        default_state: 19732,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 19735,
        state_count: 2,
        default_state: 19736,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 19737,
        state_count: 2,
        default_state: 19738,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 19739,
        state_count: 32,
        default_state: 19770,
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
        first_state: 19771,
        state_count: 32,
        default_state: 19802,
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
        first_state: 19803,
        state_count: 64,
        default_state: 19818,
        props: &[
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
        first_state: 19867,
        state_count: 64,
        default_state: 19882,
        props: &[
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
        first_state: 19931,
        state_count: 32,
        default_state: 19938,
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
        first_state: 19963,
        state_count: 32,
        default_state: 19970,
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
        first_state: 19995,
        state_count: 80,
        default_state: 20006,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20075,
        state_count: 80,
        default_state: 20086,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20155,
        state_count: 24,
        default_state: 20164,
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
        first_state: 20179,
        state_count: 24,
        default_state: 20188,
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
        first_state: 20203,
        state_count: 64,
        default_state: 20214,
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
        first_state: 20267,
        state_count: 64,
        default_state: 20278,
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
        first_state: 20331,
        state_count: 32,
        default_state: 20332,
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
        first_state: 20363,
        state_count: 32,
        default_state: 20364,
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
        first_state: 20395,
        state_count: 8,
        default_state: 20396,
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
        first_state: 20403,
        state_count: 8,
        default_state: 20404,
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
        first_state: 20411,
        state_count: 4,
        default_state: 20412,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 20415,
        state_count: 12,
        default_state: 20425,
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
        name: "test_block",
        first_state: 20427,
        state_count: 4,
        default_state: 20427,
        props: &[PropInfo {
            name: "mode",
            values: &["start", "log", "fail", "accept"],
        }],
    },
    BlockInfo {
        name: "test_instance_block",
        first_state: 20431,
        state_count: 1,
        default_state: 20431,
        props: &[],
    },
    BlockInfo {
        name: "composter",
        first_state: 20432,
        state_count: 9,
        default_state: 20432,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 20441,
        state_count: 16,
        default_state: 20441,
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
        first_state: 20457,
        state_count: 24,
        default_state: 20457,
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
        first_state: 20481,
        state_count: 24,
        default_state: 20481,
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
        first_state: 20505,
        state_count: 1,
        default_state: 20505,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 20506,
        state_count: 1,
        default_state: 20506,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 20507,
        state_count: 1,
        default_state: 20507,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 20508,
        state_count: 1,
        default_state: 20508,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 20509,
        state_count: 1,
        default_state: 20509,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 20510,
        state_count: 5,
        default_state: 20510,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 20515,
        state_count: 1,
        default_state: 20515,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 20516,
        state_count: 1,
        default_state: 20516,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 20517,
        state_count: 1,
        default_state: 20517,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 20518,
        state_count: 1,
        default_state: 20518,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 20519,
        state_count: 1,
        default_state: 20519,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 20520,
        state_count: 1,
        default_state: 20520,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 20521,
        state_count: 80,
        default_state: 20532,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20601,
        state_count: 324,
        default_state: 20604,
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
        first_state: 20925,
        state_count: 6,
        default_state: 20928,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 20931,
        state_count: 1,
        default_state: 20931,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 20932,
        state_count: 1,
        default_state: 20932,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 20933,
        state_count: 1,
        default_state: 20933,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 20934,
        state_count: 1,
        default_state: 20934,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 20935,
        state_count: 6,
        default_state: 20938,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 20941,
        state_count: 80,
        default_state: 20952,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 21021,
        state_count: 324,
        default_state: 21024,
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
        first_state: 21345,
        state_count: 1,
        default_state: 21345,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 21346,
        state_count: 80,
        default_state: 21357,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 21426,
        state_count: 6,
        default_state: 21429,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 21432,
        state_count: 2,
        default_state: 21433,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 21434,
        state_count: 24,
        default_state: 21443,
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
        first_state: 21458,
        state_count: 324,
        default_state: 21461,
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
        first_state: 21782,
        state_count: 1,
        default_state: 21782,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 21783,
        state_count: 1,
        default_state: 21783,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 21784,
        state_count: 1,
        default_state: 21784,
        props: &[],
    },
    BlockInfo {
        name: "candle",
        first_state: 21785,
        state_count: 16,
        default_state: 21788,
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
        first_state: 21801,
        state_count: 16,
        default_state: 21804,
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
        first_state: 21817,
        state_count: 16,
        default_state: 21820,
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
        first_state: 21833,
        state_count: 16,
        default_state: 21836,
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
        first_state: 21849,
        state_count: 16,
        default_state: 21852,
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
        first_state: 21865,
        state_count: 16,
        default_state: 21868,
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
        first_state: 21881,
        state_count: 16,
        default_state: 21884,
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
        first_state: 21897,
        state_count: 16,
        default_state: 21900,
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
        first_state: 21913,
        state_count: 16,
        default_state: 21916,
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
        first_state: 21929,
        state_count: 16,
        default_state: 21932,
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
        first_state: 21945,
        state_count: 16,
        default_state: 21948,
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
        first_state: 21961,
        state_count: 16,
        default_state: 21964,
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
        first_state: 21977,
        state_count: 16,
        default_state: 21980,
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
        first_state: 21993,
        state_count: 16,
        default_state: 21996,
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
        first_state: 22009,
        state_count: 16,
        default_state: 22012,
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
        first_state: 22025,
        state_count: 16,
        default_state: 22028,
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
        first_state: 22041,
        state_count: 16,
        default_state: 22044,
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
        first_state: 22057,
        state_count: 2,
        default_state: 22058,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "white_candle_cake",
        first_state: 22059,
        state_count: 2,
        default_state: 22060,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "orange_candle_cake",
        first_state: 22061,
        state_count: 2,
        default_state: 22062,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "magenta_candle_cake",
        first_state: 22063,
        state_count: 2,
        default_state: 22064,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_blue_candle_cake",
        first_state: 22065,
        state_count: 2,
        default_state: 22066,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "yellow_candle_cake",
        first_state: 22067,
        state_count: 2,
        default_state: 22068,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lime_candle_cake",
        first_state: 22069,
        state_count: 2,
        default_state: 22070,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pink_candle_cake",
        first_state: 22071,
        state_count: 2,
        default_state: 22072,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "gray_candle_cake",
        first_state: 22073,
        state_count: 2,
        default_state: 22074,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_gray_candle_cake",
        first_state: 22075,
        state_count: 2,
        default_state: 22076,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cyan_candle_cake",
        first_state: 22077,
        state_count: 2,
        default_state: 22078,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "purple_candle_cake",
        first_state: 22079,
        state_count: 2,
        default_state: 22080,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "blue_candle_cake",
        first_state: 22081,
        state_count: 2,
        default_state: 22082,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brown_candle_cake",
        first_state: 22083,
        state_count: 2,
        default_state: 22084,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "green_candle_cake",
        first_state: 22085,
        state_count: 2,
        default_state: 22086,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "red_candle_cake",
        first_state: 22087,
        state_count: 2,
        default_state: 22088,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "black_candle_cake",
        first_state: 22089,
        state_count: 2,
        default_state: 22090,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "amethyst_block",
        first_state: 22091,
        state_count: 1,
        default_state: 22091,
        props: &[],
    },
    BlockInfo {
        name: "budding_amethyst",
        first_state: 22092,
        state_count: 1,
        default_state: 22092,
        props: &[],
    },
    BlockInfo {
        name: "amethyst_cluster",
        first_state: 22093,
        state_count: 12,
        default_state: 22102,
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
        first_state: 22105,
        state_count: 12,
        default_state: 22114,
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
        first_state: 22117,
        state_count: 12,
        default_state: 22126,
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
        first_state: 22129,
        state_count: 12,
        default_state: 22138,
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
        first_state: 22141,
        state_count: 1,
        default_state: 22141,
        props: &[],
    },
    BlockInfo {
        name: "tuff_slab",
        first_state: 22142,
        state_count: 6,
        default_state: 22145,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tuff_stairs",
        first_state: 22148,
        state_count: 80,
        default_state: 22159,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tuff_wall",
        first_state: 22228,
        state_count: 324,
        default_state: 22231,
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
        name: "polished_tuff",
        first_state: 22552,
        state_count: 1,
        default_state: 22552,
        props: &[],
    },
    BlockInfo {
        name: "polished_tuff_slab",
        first_state: 22553,
        state_count: 6,
        default_state: 22556,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_tuff_stairs",
        first_state: 22559,
        state_count: 80,
        default_state: 22570,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "polished_tuff_wall",
        first_state: 22639,
        state_count: 324,
        default_state: 22642,
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
        name: "chiseled_tuff",
        first_state: 22963,
        state_count: 1,
        default_state: 22963,
        props: &[],
    },
    BlockInfo {
        name: "tuff_bricks",
        first_state: 22964,
        state_count: 1,
        default_state: 22964,
        props: &[],
    },
    BlockInfo {
        name: "tuff_brick_slab",
        first_state: 22965,
        state_count: 6,
        default_state: 22968,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tuff_brick_stairs",
        first_state: 22971,
        state_count: 80,
        default_state: 22982,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
                ],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "tuff_brick_wall",
        first_state: 23051,
        state_count: 324,
        default_state: 23054,
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
        name: "chiseled_tuff_bricks",
        first_state: 23375,
        state_count: 1,
        default_state: 23375,
        props: &[],
    },
    BlockInfo {
        name: "calcite",
        first_state: 23376,
        state_count: 1,
        default_state: 23376,
        props: &[],
    },
    BlockInfo {
        name: "tinted_glass",
        first_state: 23377,
        state_count: 1,
        default_state: 23377,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow",
        first_state: 23378,
        state_count: 1,
        default_state: 23378,
        props: &[],
    },
    BlockInfo {
        name: "sculk_sensor",
        first_state: 23379,
        state_count: 96,
        default_state: 23380,
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
        name: "calibrated_sculk_sensor",
        first_state: 23475,
        state_count: 384,
        default_state: 23476,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
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
        first_state: 23859,
        state_count: 1,
        default_state: 23859,
        props: &[],
    },
    BlockInfo {
        name: "sculk_vein",
        first_state: 23860,
        state_count: 128,
        default_state: 23987,
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
        first_state: 23988,
        state_count: 2,
        default_state: 23989,
        props: &[PropInfo {
            name: "bloom",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sculk_shrieker",
        first_state: 23990,
        state_count: 8,
        default_state: 23997,
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
        name: "copper_block",
        first_state: 23998,
        state_count: 1,
        default_state: 23998,
        props: &[],
    },
    BlockInfo {
        name: "exposed_copper",
        first_state: 23999,
        state_count: 1,
        default_state: 23999,
        props: &[],
    },
    BlockInfo {
        name: "weathered_copper",
        first_state: 24000,
        state_count: 1,
        default_state: 24000,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_copper",
        first_state: 24001,
        state_count: 1,
        default_state: 24001,
        props: &[],
    },
    BlockInfo {
        name: "copper_ore",
        first_state: 24002,
        state_count: 1,
        default_state: 24002,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_copper_ore",
        first_state: 24003,
        state_count: 1,
        default_state: 24003,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper",
        first_state: 24004,
        state_count: 1,
        default_state: 24004,
        props: &[],
    },
    BlockInfo {
        name: "weathered_cut_copper",
        first_state: 24005,
        state_count: 1,
        default_state: 24005,
        props: &[],
    },
    BlockInfo {
        name: "exposed_cut_copper",
        first_state: 24006,
        state_count: 1,
        default_state: 24006,
        props: &[],
    },
    BlockInfo {
        name: "cut_copper",
        first_state: 24007,
        state_count: 1,
        default_state: 24007,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_chiseled_copper",
        first_state: 24008,
        state_count: 1,
        default_state: 24008,
        props: &[],
    },
    BlockInfo {
        name: "weathered_chiseled_copper",
        first_state: 24009,
        state_count: 1,
        default_state: 24009,
        props: &[],
    },
    BlockInfo {
        name: "exposed_chiseled_copper",
        first_state: 24010,
        state_count: 1,
        default_state: 24010,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_copper",
        first_state: 24011,
        state_count: 1,
        default_state: 24011,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_chiseled_copper",
        first_state: 24012,
        state_count: 1,
        default_state: 24012,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_chiseled_copper",
        first_state: 24013,
        state_count: 1,
        default_state: 24013,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_chiseled_copper",
        first_state: 24014,
        state_count: 1,
        default_state: 24014,
        props: &[],
    },
    BlockInfo {
        name: "waxed_chiseled_copper",
        first_state: 24015,
        state_count: 1,
        default_state: 24015,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper_stairs",
        first_state: 24016,
        state_count: 80,
        default_state: 24027,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24096,
        state_count: 80,
        default_state: 24107,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24176,
        state_count: 80,
        default_state: 24187,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24256,
        state_count: 80,
        default_state: 24267,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24336,
        state_count: 6,
        default_state: 24339,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24342,
        state_count: 6,
        default_state: 24345,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24348,
        state_count: 6,
        default_state: 24351,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24354,
        state_count: 6,
        default_state: 24357,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24360,
        state_count: 1,
        default_state: 24360,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_copper",
        first_state: 24361,
        state_count: 1,
        default_state: 24361,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_copper",
        first_state: 24362,
        state_count: 1,
        default_state: 24362,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_copper",
        first_state: 24363,
        state_count: 1,
        default_state: 24363,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper",
        first_state: 24364,
        state_count: 1,
        default_state: 24364,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper",
        first_state: 24365,
        state_count: 1,
        default_state: 24365,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper",
        first_state: 24366,
        state_count: 1,
        default_state: 24366,
        props: &[],
    },
    BlockInfo {
        name: "waxed_cut_copper",
        first_state: 24367,
        state_count: 1,
        default_state: 24367,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_stairs",
        first_state: 24368,
        state_count: 80,
        default_state: 24379,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24448,
        state_count: 80,
        default_state: 24459,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24528,
        state_count: 80,
        default_state: 24539,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24608,
        state_count: 80,
        default_state: 24619,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24688,
        state_count: 6,
        default_state: 24691,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24694,
        state_count: 6,
        default_state: 24697,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24700,
        state_count: 6,
        default_state: 24703,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24706,
        state_count: 6,
        default_state: 24709,
        props: &[
            PropInfo {
                name: "type",
                values: &["top", "bottom", "double"],
            },
            PropInfo {
                name: "waterlogged",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "copper_door",
        first_state: 24712,
        state_count: 64,
        default_state: 24723,
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
        name: "exposed_copper_door",
        first_state: 24776,
        state_count: 64,
        default_state: 24787,
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
        name: "oxidized_copper_door",
        first_state: 24840,
        state_count: 64,
        default_state: 24851,
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
        name: "weathered_copper_door",
        first_state: 24904,
        state_count: 64,
        default_state: 24915,
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
        name: "waxed_copper_door",
        first_state: 24968,
        state_count: 64,
        default_state: 24979,
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
        name: "waxed_exposed_copper_door",
        first_state: 25032,
        state_count: 64,
        default_state: 25043,
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
        name: "waxed_oxidized_copper_door",
        first_state: 25096,
        state_count: 64,
        default_state: 25107,
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
        name: "waxed_weathered_copper_door",
        first_state: 25160,
        state_count: 64,
        default_state: 25171,
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
        name: "copper_trapdoor",
        first_state: 25224,
        state_count: 64,
        default_state: 25239,
        props: &[
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
        name: "exposed_copper_trapdoor",
        first_state: 25288,
        state_count: 64,
        default_state: 25303,
        props: &[
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
        name: "oxidized_copper_trapdoor",
        first_state: 25352,
        state_count: 64,
        default_state: 25367,
        props: &[
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
        name: "weathered_copper_trapdoor",
        first_state: 25416,
        state_count: 64,
        default_state: 25431,
        props: &[
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
        name: "waxed_copper_trapdoor",
        first_state: 25480,
        state_count: 64,
        default_state: 25495,
        props: &[
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
        name: "waxed_exposed_copper_trapdoor",
        first_state: 25544,
        state_count: 64,
        default_state: 25559,
        props: &[
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
        name: "waxed_oxidized_copper_trapdoor",
        first_state: 25608,
        state_count: 64,
        default_state: 25623,
        props: &[
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
        name: "waxed_weathered_copper_trapdoor",
        first_state: 25672,
        state_count: 64,
        default_state: 25687,
        props: &[
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
        name: "copper_grate",
        first_state: 25736,
        state_count: 2,
        default_state: 25737,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "exposed_copper_grate",
        first_state: 25738,
        state_count: 2,
        default_state: 25739,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "weathered_copper_grate",
        first_state: 25740,
        state_count: 2,
        default_state: 25741,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oxidized_copper_grate",
        first_state: 25742,
        state_count: 2,
        default_state: 25743,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_copper_grate",
        first_state: 25744,
        state_count: 2,
        default_state: 25745,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_exposed_copper_grate",
        first_state: 25746,
        state_count: 2,
        default_state: 25747,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_weathered_copper_grate",
        first_state: 25748,
        state_count: 2,
        default_state: 25749,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_oxidized_copper_grate",
        first_state: 25750,
        state_count: 2,
        default_state: 25751,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "copper_bulb",
        first_state: 25752,
        state_count: 4,
        default_state: 25755,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "exposed_copper_bulb",
        first_state: 25756,
        state_count: 4,
        default_state: 25759,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "weathered_copper_bulb",
        first_state: 25760,
        state_count: 4,
        default_state: 25763,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "oxidized_copper_bulb",
        first_state: 25764,
        state_count: 4,
        default_state: 25767,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_copper_bulb",
        first_state: 25768,
        state_count: 4,
        default_state: 25771,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_exposed_copper_bulb",
        first_state: 25772,
        state_count: 4,
        default_state: 25775,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_weathered_copper_bulb",
        first_state: 25776,
        state_count: 4,
        default_state: 25779,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "waxed_oxidized_copper_bulb",
        first_state: 25780,
        state_count: 4,
        default_state: 25783,
        props: &[
            PropInfo {
                name: "lit",
                values: &["true", "false"],
            },
            PropInfo {
                name: "powered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "lightning_rod",
        first_state: 25784,
        state_count: 24,
        default_state: 25803,
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
        first_state: 25808,
        state_count: 20,
        default_state: 25813,
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
        first_state: 25828,
        state_count: 1,
        default_state: 25828,
        props: &[],
    },
    BlockInfo {
        name: "cave_vines",
        first_state: 25829,
        state_count: 52,
        default_state: 25830,
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
        first_state: 25881,
        state_count: 2,
        default_state: 25882,
        props: &[PropInfo {
            name: "berries",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spore_blossom",
        first_state: 25883,
        state_count: 1,
        default_state: 25883,
        props: &[],
    },
    BlockInfo {
        name: "azalea",
        first_state: 25884,
        state_count: 1,
        default_state: 25884,
        props: &[],
    },
    BlockInfo {
        name: "flowering_azalea",
        first_state: 25885,
        state_count: 1,
        default_state: 25885,
        props: &[],
    },
    BlockInfo {
        name: "moss_carpet",
        first_state: 25886,
        state_count: 1,
        default_state: 25886,
        props: &[],
    },
    BlockInfo {
        name: "pink_petals",
        first_state: 25887,
        state_count: 16,
        default_state: 25887,
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
        name: "wildflowers",
        first_state: 25903,
        state_count: 16,
        default_state: 25903,
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
        name: "leaf_litter",
        first_state: 25919,
        state_count: 16,
        default_state: 25919,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "segment_amount",
                values: &["1", "2", "3", "4"],
            },
        ],
    },
    BlockInfo {
        name: "moss_block",
        first_state: 25935,
        state_count: 1,
        default_state: 25935,
        props: &[],
    },
    BlockInfo {
        name: "big_dripleaf",
        first_state: 25936,
        state_count: 32,
        default_state: 25937,
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
        first_state: 25968,
        state_count: 8,
        default_state: 25969,
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
        first_state: 25976,
        state_count: 16,
        default_state: 25979,
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
        first_state: 25992,
        state_count: 2,
        default_state: 25993,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "rooted_dirt",
        first_state: 25994,
        state_count: 1,
        default_state: 25994,
        props: &[],
    },
    BlockInfo {
        name: "mud",
        first_state: 25995,
        state_count: 1,
        default_state: 25995,
        props: &[],
    },
    BlockInfo {
        name: "deepslate",
        first_state: 25996,
        state_count: 3,
        default_state: 25997,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cobbled_deepslate",
        first_state: 25999,
        state_count: 1,
        default_state: 25999,
        props: &[],
    },
    BlockInfo {
        name: "cobbled_deepslate_stairs",
        first_state: 26000,
        state_count: 80,
        default_state: 26011,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 26080,
        state_count: 6,
        default_state: 26083,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 26086,
        state_count: 324,
        default_state: 26089,
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
        first_state: 26410,
        state_count: 1,
        default_state: 26410,
        props: &[],
    },
    BlockInfo {
        name: "polished_deepslate_stairs",
        first_state: 26411,
        state_count: 80,
        default_state: 26422,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 26491,
        state_count: 6,
        default_state: 26494,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 26497,
        state_count: 324,
        default_state: 26500,
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
        first_state: 26821,
        state_count: 1,
        default_state: 26821,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_tile_stairs",
        first_state: 26822,
        state_count: 80,
        default_state: 26833,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 26902,
        state_count: 6,
        default_state: 26905,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 26908,
        state_count: 324,
        default_state: 26911,
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
        first_state: 27232,
        state_count: 1,
        default_state: 27232,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_brick_stairs",
        first_state: 27233,
        state_count: 80,
        default_state: 27244,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 27313,
        state_count: 6,
        default_state: 27316,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 27319,
        state_count: 324,
        default_state: 27322,
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
        first_state: 27643,
        state_count: 1,
        default_state: 27643,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_bricks",
        first_state: 27644,
        state_count: 1,
        default_state: 27644,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_tiles",
        first_state: 27645,
        state_count: 1,
        default_state: 27645,
        props: &[],
    },
    BlockInfo {
        name: "infested_deepslate",
        first_state: 27646,
        state_count: 3,
        default_state: 27647,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "smooth_basalt",
        first_state: 27649,
        state_count: 1,
        default_state: 27649,
        props: &[],
    },
    BlockInfo {
        name: "raw_iron_block",
        first_state: 27650,
        state_count: 1,
        default_state: 27650,
        props: &[],
    },
    BlockInfo {
        name: "raw_copper_block",
        first_state: 27651,
        state_count: 1,
        default_state: 27651,
        props: &[],
    },
    BlockInfo {
        name: "raw_gold_block",
        first_state: 27652,
        state_count: 1,
        default_state: 27652,
        props: &[],
    },
    BlockInfo {
        name: "potted_azalea_bush",
        first_state: 27653,
        state_count: 1,
        default_state: 27653,
        props: &[],
    },
    BlockInfo {
        name: "potted_flowering_azalea_bush",
        first_state: 27654,
        state_count: 1,
        default_state: 27654,
        props: &[],
    },
    BlockInfo {
        name: "ochre_froglight",
        first_state: 27655,
        state_count: 3,
        default_state: 27656,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "verdant_froglight",
        first_state: 27658,
        state_count: 3,
        default_state: 27659,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pearlescent_froglight",
        first_state: 27661,
        state_count: 3,
        default_state: 27662,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "frogspawn",
        first_state: 27664,
        state_count: 1,
        default_state: 27664,
        props: &[],
    },
    BlockInfo {
        name: "reinforced_deepslate",
        first_state: 27665,
        state_count: 1,
        default_state: 27665,
        props: &[],
    },
    BlockInfo {
        name: "decorated_pot",
        first_state: 27666,
        state_count: 16,
        default_state: 27675,
        props: &[
            PropInfo {
                name: "cracked",
                values: &["true", "false"],
            },
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
        name: "crafter",
        first_state: 27682,
        state_count: 48,
        default_state: 27727,
        props: &[
            PropInfo {
                name: "crafting",
                values: &["true", "false"],
            },
            PropInfo {
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
            },
            PropInfo {
                name: "triggered",
                values: &["true", "false"],
            },
        ],
    },
    BlockInfo {
        name: "trial_spawner",
        first_state: 27730,
        state_count: 12,
        default_state: 27736,
        props: &[
            PropInfo {
                name: "ominous",
                values: &["true", "false"],
            },
            PropInfo {
                name: "trial_spawner_state",
                values: &[
                    "inactive",
                    "waiting_for_players",
                    "active",
                    "waiting_for_reward_ejection",
                    "ejecting_reward",
                    "cooldown",
                ],
            },
        ],
    },
    BlockInfo {
        name: "vault",
        first_state: 27742,
        state_count: 32,
        default_state: 27746,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "ominous",
                values: &["true", "false"],
            },
            PropInfo {
                name: "vault_state",
                values: &["inactive", "active", "unlocking", "ejecting"],
            },
        ],
    },
    BlockInfo {
        name: "heavy_core",
        first_state: 27774,
        state_count: 2,
        default_state: 27775,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pale_moss_block",
        first_state: 27776,
        state_count: 1,
        default_state: 27776,
        props: &[],
    },
    BlockInfo {
        name: "pale_moss_carpet",
        first_state: 27777,
        state_count: 162,
        default_state: 27777,
        props: &[
            PropInfo {
                name: "bottom",
                values: &["true", "false"],
            },
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
                name: "west",
                values: &["none", "low", "tall"],
            },
        ],
    },
    BlockInfo {
        name: "pale_hanging_moss",
        first_state: 27939,
        state_count: 2,
        default_state: 27939,
        props: &[PropInfo {
            name: "tip",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "open_eyeblossom",
        first_state: 27941,
        state_count: 1,
        default_state: 27941,
        props: &[],
    },
    BlockInfo {
        name: "closed_eyeblossom",
        first_state: 27942,
        state_count: 1,
        default_state: 27942,
        props: &[],
    },
    BlockInfo {
        name: "potted_open_eyeblossom",
        first_state: 27943,
        state_count: 1,
        default_state: 27943,
        props: &[],
    },
    BlockInfo {
        name: "potted_closed_eyeblossom",
        first_state: 27944,
        state_count: 1,
        default_state: 27944,
        props: &[],
    },
    BlockInfo {
        name: "firefly_bush",
        first_state: 27945,
        state_count: 1,
        default_state: 27945,
        props: &[],
    },
];
