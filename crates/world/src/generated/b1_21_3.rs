// @generated `cargo xtask gen-blockstates 1.21.3` из reports/blocks.json. НЕ РЕДАКТИРОВАТЬ.
use crate::block::{BlockInfo, PropInfo};

pub const VERSION: &str = "1.21.3";
pub const STATE_COUNT: u32 = 27318;

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
        name: "seagrass",
        first_state: 2051,
        state_count: 1,
        default_state: 2051,
        props: &[],
    },
    BlockInfo {
        name: "tall_seagrass",
        first_state: 2052,
        state_count: 2,
        default_state: 2053,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "piston",
        first_state: 2054,
        state_count: 12,
        default_state: 2060,
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
        first_state: 2066,
        state_count: 24,
        default_state: 2068,
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
        first_state: 2090,
        state_count: 1,
        default_state: 2090,
        props: &[],
    },
    BlockInfo {
        name: "orange_wool",
        first_state: 2091,
        state_count: 1,
        default_state: 2091,
        props: &[],
    },
    BlockInfo {
        name: "magenta_wool",
        first_state: 2092,
        state_count: 1,
        default_state: 2092,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_wool",
        first_state: 2093,
        state_count: 1,
        default_state: 2093,
        props: &[],
    },
    BlockInfo {
        name: "yellow_wool",
        first_state: 2094,
        state_count: 1,
        default_state: 2094,
        props: &[],
    },
    BlockInfo {
        name: "lime_wool",
        first_state: 2095,
        state_count: 1,
        default_state: 2095,
        props: &[],
    },
    BlockInfo {
        name: "pink_wool",
        first_state: 2096,
        state_count: 1,
        default_state: 2096,
        props: &[],
    },
    BlockInfo {
        name: "gray_wool",
        first_state: 2097,
        state_count: 1,
        default_state: 2097,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_wool",
        first_state: 2098,
        state_count: 1,
        default_state: 2098,
        props: &[],
    },
    BlockInfo {
        name: "cyan_wool",
        first_state: 2099,
        state_count: 1,
        default_state: 2099,
        props: &[],
    },
    BlockInfo {
        name: "purple_wool",
        first_state: 2100,
        state_count: 1,
        default_state: 2100,
        props: &[],
    },
    BlockInfo {
        name: "blue_wool",
        first_state: 2101,
        state_count: 1,
        default_state: 2101,
        props: &[],
    },
    BlockInfo {
        name: "brown_wool",
        first_state: 2102,
        state_count: 1,
        default_state: 2102,
        props: &[],
    },
    BlockInfo {
        name: "green_wool",
        first_state: 2103,
        state_count: 1,
        default_state: 2103,
        props: &[],
    },
    BlockInfo {
        name: "red_wool",
        first_state: 2104,
        state_count: 1,
        default_state: 2104,
        props: &[],
    },
    BlockInfo {
        name: "black_wool",
        first_state: 2105,
        state_count: 1,
        default_state: 2105,
        props: &[],
    },
    BlockInfo {
        name: "moving_piston",
        first_state: 2106,
        state_count: 12,
        default_state: 2106,
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
        first_state: 2118,
        state_count: 1,
        default_state: 2118,
        props: &[],
    },
    BlockInfo {
        name: "torchflower",
        first_state: 2119,
        state_count: 1,
        default_state: 2119,
        props: &[],
    },
    BlockInfo {
        name: "poppy",
        first_state: 2120,
        state_count: 1,
        default_state: 2120,
        props: &[],
    },
    BlockInfo {
        name: "blue_orchid",
        first_state: 2121,
        state_count: 1,
        default_state: 2121,
        props: &[],
    },
    BlockInfo {
        name: "allium",
        first_state: 2122,
        state_count: 1,
        default_state: 2122,
        props: &[],
    },
    BlockInfo {
        name: "azure_bluet",
        first_state: 2123,
        state_count: 1,
        default_state: 2123,
        props: &[],
    },
    BlockInfo {
        name: "red_tulip",
        first_state: 2124,
        state_count: 1,
        default_state: 2124,
        props: &[],
    },
    BlockInfo {
        name: "orange_tulip",
        first_state: 2125,
        state_count: 1,
        default_state: 2125,
        props: &[],
    },
    BlockInfo {
        name: "white_tulip",
        first_state: 2126,
        state_count: 1,
        default_state: 2126,
        props: &[],
    },
    BlockInfo {
        name: "pink_tulip",
        first_state: 2127,
        state_count: 1,
        default_state: 2127,
        props: &[],
    },
    BlockInfo {
        name: "oxeye_daisy",
        first_state: 2128,
        state_count: 1,
        default_state: 2128,
        props: &[],
    },
    BlockInfo {
        name: "cornflower",
        first_state: 2129,
        state_count: 1,
        default_state: 2129,
        props: &[],
    },
    BlockInfo {
        name: "wither_rose",
        first_state: 2130,
        state_count: 1,
        default_state: 2130,
        props: &[],
    },
    BlockInfo {
        name: "lily_of_the_valley",
        first_state: 2131,
        state_count: 1,
        default_state: 2131,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom",
        first_state: 2132,
        state_count: 1,
        default_state: 2132,
        props: &[],
    },
    BlockInfo {
        name: "red_mushroom",
        first_state: 2133,
        state_count: 1,
        default_state: 2133,
        props: &[],
    },
    BlockInfo {
        name: "gold_block",
        first_state: 2134,
        state_count: 1,
        default_state: 2134,
        props: &[],
    },
    BlockInfo {
        name: "iron_block",
        first_state: 2135,
        state_count: 1,
        default_state: 2135,
        props: &[],
    },
    BlockInfo {
        name: "bricks",
        first_state: 2136,
        state_count: 1,
        default_state: 2136,
        props: &[],
    },
    BlockInfo {
        name: "tnt",
        first_state: 2137,
        state_count: 2,
        default_state: 2138,
        props: &[PropInfo {
            name: "unstable",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bookshelf",
        first_state: 2139,
        state_count: 1,
        default_state: 2139,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_bookshelf",
        first_state: 2140,
        state_count: 256,
        default_state: 2203,
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
        first_state: 2396,
        state_count: 1,
        default_state: 2396,
        props: &[],
    },
    BlockInfo {
        name: "obsidian",
        first_state: 2397,
        state_count: 1,
        default_state: 2397,
        props: &[],
    },
    BlockInfo {
        name: "torch",
        first_state: 2398,
        state_count: 1,
        default_state: 2398,
        props: &[],
    },
    BlockInfo {
        name: "wall_torch",
        first_state: 2399,
        state_count: 4,
        default_state: 2399,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "fire",
        first_state: 2403,
        state_count: 512,
        default_state: 2434,
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
        first_state: 2915,
        state_count: 1,
        default_state: 2915,
        props: &[],
    },
    BlockInfo {
        name: "spawner",
        first_state: 2916,
        state_count: 1,
        default_state: 2916,
        props: &[],
    },
    BlockInfo {
        name: "creaking_heart",
        first_state: 2917,
        state_count: 9,
        default_state: 2920,
        props: &[
            PropInfo {
                name: "axis",
                values: &["x", "y", "z"],
            },
            PropInfo {
                name: "creaking",
                values: &["disabled", "dormant", "active"],
            },
        ],
    },
    BlockInfo {
        name: "oak_stairs",
        first_state: 2926,
        state_count: 80,
        default_state: 2937,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 3006,
        state_count: 24,
        default_state: 3007,
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
        first_state: 3030,
        state_count: 1296,
        default_state: 4190,
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
        first_state: 4326,
        state_count: 1,
        default_state: 4326,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_diamond_ore",
        first_state: 4327,
        state_count: 1,
        default_state: 4327,
        props: &[],
    },
    BlockInfo {
        name: "diamond_block",
        first_state: 4328,
        state_count: 1,
        default_state: 4328,
        props: &[],
    },
    BlockInfo {
        name: "crafting_table",
        first_state: 4329,
        state_count: 1,
        default_state: 4329,
        props: &[],
    },
    BlockInfo {
        name: "wheat",
        first_state: 4330,
        state_count: 8,
        default_state: 4330,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "farmland",
        first_state: 4338,
        state_count: 8,
        default_state: 4338,
        props: &[PropInfo {
            name: "moisture",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "furnace",
        first_state: 4346,
        state_count: 8,
        default_state: 4347,
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
        first_state: 4354,
        state_count: 32,
        default_state: 4355,
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
        first_state: 4386,
        state_count: 32,
        default_state: 4387,
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
        first_state: 4418,
        state_count: 32,
        default_state: 4419,
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
        first_state: 4450,
        state_count: 32,
        default_state: 4451,
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
        first_state: 4482,
        state_count: 32,
        default_state: 4483,
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
        first_state: 4514,
        state_count: 32,
        default_state: 4515,
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
        first_state: 4546,
        state_count: 32,
        default_state: 4547,
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
        first_state: 4578,
        state_count: 32,
        default_state: 4579,
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
        first_state: 4610,
        state_count: 32,
        default_state: 4611,
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
        first_state: 4642,
        state_count: 32,
        default_state: 4643,
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
        first_state: 4674,
        state_count: 64,
        default_state: 4685,
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
        first_state: 4738,
        state_count: 8,
        default_state: 4739,
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
        first_state: 4746,
        state_count: 20,
        default_state: 4747,
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
        first_state: 4766,
        state_count: 80,
        default_state: 4777,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 4846,
        state_count: 8,
        default_state: 4847,
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
        first_state: 4854,
        state_count: 8,
        default_state: 4855,
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
        first_state: 4862,
        state_count: 8,
        default_state: 4863,
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
        first_state: 4870,
        state_count: 8,
        default_state: 4871,
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
        first_state: 4878,
        state_count: 8,
        default_state: 4879,
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
        first_state: 4886,
        state_count: 8,
        default_state: 4887,
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
        first_state: 4894,
        state_count: 8,
        default_state: 4895,
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
        first_state: 4902,
        state_count: 8,
        default_state: 4903,
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
        first_state: 4910,
        state_count: 8,
        default_state: 4911,
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
        first_state: 4918,
        state_count: 8,
        default_state: 4919,
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
        first_state: 4926,
        state_count: 64,
        default_state: 4959,
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
        first_state: 4990,
        state_count: 64,
        default_state: 5023,
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
        first_state: 5054,
        state_count: 64,
        default_state: 5087,
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
        first_state: 5118,
        state_count: 64,
        default_state: 5151,
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
        first_state: 5182,
        state_count: 64,
        default_state: 5215,
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
        first_state: 5246,
        state_count: 64,
        default_state: 5279,
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
        first_state: 5310,
        state_count: 64,
        default_state: 5343,
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
        first_state: 5374,
        state_count: 64,
        default_state: 5407,
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
        first_state: 5438,
        state_count: 64,
        default_state: 5471,
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
        first_state: 5502,
        state_count: 64,
        default_state: 5535,
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
        first_state: 5566,
        state_count: 64,
        default_state: 5599,
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
        first_state: 5630,
        state_count: 64,
        default_state: 5663,
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
        first_state: 5694,
        state_count: 8,
        default_state: 5695,
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
        first_state: 5702,
        state_count: 8,
        default_state: 5703,
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
        first_state: 5710,
        state_count: 8,
        default_state: 5711,
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
        first_state: 5718,
        state_count: 8,
        default_state: 5719,
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
        first_state: 5726,
        state_count: 8,
        default_state: 5727,
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
        first_state: 5734,
        state_count: 8,
        default_state: 5735,
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
        first_state: 5742,
        state_count: 8,
        default_state: 5743,
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
        first_state: 5750,
        state_count: 8,
        default_state: 5751,
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
        first_state: 5758,
        state_count: 8,
        default_state: 5759,
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
        first_state: 5766,
        state_count: 8,
        default_state: 5767,
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
        first_state: 5774,
        state_count: 8,
        default_state: 5775,
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
        first_state: 5782,
        state_count: 8,
        default_state: 5783,
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
        first_state: 5790,
        state_count: 24,
        default_state: 5799,
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
        first_state: 5814,
        state_count: 2,
        default_state: 5815,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "iron_door",
        first_state: 5816,
        state_count: 64,
        default_state: 5827,
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
        first_state: 5880,
        state_count: 2,
        default_state: 5881,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spruce_pressure_plate",
        first_state: 5882,
        state_count: 2,
        default_state: 5883,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "birch_pressure_plate",
        first_state: 5884,
        state_count: 2,
        default_state: 5885,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "jungle_pressure_plate",
        first_state: 5886,
        state_count: 2,
        default_state: 5887,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "acacia_pressure_plate",
        first_state: 5888,
        state_count: 2,
        default_state: 5889,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cherry_pressure_plate",
        first_state: 5890,
        state_count: 2,
        default_state: 5891,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dark_oak_pressure_plate",
        first_state: 5892,
        state_count: 2,
        default_state: 5893,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pale_oak_pressure_plate",
        first_state: 5894,
        state_count: 2,
        default_state: 5895,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "mangrove_pressure_plate",
        first_state: 5896,
        state_count: 2,
        default_state: 5897,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_pressure_plate",
        first_state: 5898,
        state_count: 2,
        default_state: 5899,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_ore",
        first_state: 5900,
        state_count: 2,
        default_state: 5901,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "deepslate_redstone_ore",
        first_state: 5902,
        state_count: 2,
        default_state: 5903,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_torch",
        first_state: 5904,
        state_count: 2,
        default_state: 5904,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "redstone_wall_torch",
        first_state: 5906,
        state_count: 8,
        default_state: 5906,
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
        first_state: 5914,
        state_count: 24,
        default_state: 5923,
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
        first_state: 5938,
        state_count: 8,
        default_state: 5938,
        props: &[PropInfo {
            name: "layers",
            values: &["1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "ice",
        first_state: 5946,
        state_count: 1,
        default_state: 5946,
        props: &[],
    },
    BlockInfo {
        name: "snow_block",
        first_state: 5947,
        state_count: 1,
        default_state: 5947,
        props: &[],
    },
    BlockInfo {
        name: "cactus",
        first_state: 5948,
        state_count: 16,
        default_state: 5948,
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
        first_state: 5964,
        state_count: 1,
        default_state: 5964,
        props: &[],
    },
    BlockInfo {
        name: "sugar_cane",
        first_state: 5965,
        state_count: 16,
        default_state: 5965,
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
        first_state: 5981,
        state_count: 2,
        default_state: 5982,
        props: &[PropInfo {
            name: "has_record",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oak_fence",
        first_state: 5983,
        state_count: 32,
        default_state: 6014,
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
        first_state: 6015,
        state_count: 1,
        default_state: 6015,
        props: &[],
    },
    BlockInfo {
        name: "soul_sand",
        first_state: 6016,
        state_count: 1,
        default_state: 6016,
        props: &[],
    },
    BlockInfo {
        name: "soul_soil",
        first_state: 6017,
        state_count: 1,
        default_state: 6017,
        props: &[],
    },
    BlockInfo {
        name: "basalt",
        first_state: 6018,
        state_count: 3,
        default_state: 6019,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "polished_basalt",
        first_state: 6021,
        state_count: 3,
        default_state: 6022,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "soul_torch",
        first_state: 6024,
        state_count: 1,
        default_state: 6024,
        props: &[],
    },
    BlockInfo {
        name: "soul_wall_torch",
        first_state: 6025,
        state_count: 4,
        default_state: 6025,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "glowstone",
        first_state: 6029,
        state_count: 1,
        default_state: 6029,
        props: &[],
    },
    BlockInfo {
        name: "nether_portal",
        first_state: 6030,
        state_count: 2,
        default_state: 6030,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "z"],
        }],
    },
    BlockInfo {
        name: "carved_pumpkin",
        first_state: 6032,
        state_count: 4,
        default_state: 6032,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "jack_o_lantern",
        first_state: 6036,
        state_count: 4,
        default_state: 6036,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cake",
        first_state: 6040,
        state_count: 7,
        default_state: 6040,
        props: &[PropInfo {
            name: "bites",
            values: &["0", "1", "2", "3", "4", "5", "6"],
        }],
    },
    BlockInfo {
        name: "repeater",
        first_state: 6047,
        state_count: 64,
        default_state: 6050,
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
        first_state: 6111,
        state_count: 1,
        default_state: 6111,
        props: &[],
    },
    BlockInfo {
        name: "orange_stained_glass",
        first_state: 6112,
        state_count: 1,
        default_state: 6112,
        props: &[],
    },
    BlockInfo {
        name: "magenta_stained_glass",
        first_state: 6113,
        state_count: 1,
        default_state: 6113,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_stained_glass",
        first_state: 6114,
        state_count: 1,
        default_state: 6114,
        props: &[],
    },
    BlockInfo {
        name: "yellow_stained_glass",
        first_state: 6115,
        state_count: 1,
        default_state: 6115,
        props: &[],
    },
    BlockInfo {
        name: "lime_stained_glass",
        first_state: 6116,
        state_count: 1,
        default_state: 6116,
        props: &[],
    },
    BlockInfo {
        name: "pink_stained_glass",
        first_state: 6117,
        state_count: 1,
        default_state: 6117,
        props: &[],
    },
    BlockInfo {
        name: "gray_stained_glass",
        first_state: 6118,
        state_count: 1,
        default_state: 6118,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_stained_glass",
        first_state: 6119,
        state_count: 1,
        default_state: 6119,
        props: &[],
    },
    BlockInfo {
        name: "cyan_stained_glass",
        first_state: 6120,
        state_count: 1,
        default_state: 6120,
        props: &[],
    },
    BlockInfo {
        name: "purple_stained_glass",
        first_state: 6121,
        state_count: 1,
        default_state: 6121,
        props: &[],
    },
    BlockInfo {
        name: "blue_stained_glass",
        first_state: 6122,
        state_count: 1,
        default_state: 6122,
        props: &[],
    },
    BlockInfo {
        name: "brown_stained_glass",
        first_state: 6123,
        state_count: 1,
        default_state: 6123,
        props: &[],
    },
    BlockInfo {
        name: "green_stained_glass",
        first_state: 6124,
        state_count: 1,
        default_state: 6124,
        props: &[],
    },
    BlockInfo {
        name: "red_stained_glass",
        first_state: 6125,
        state_count: 1,
        default_state: 6125,
        props: &[],
    },
    BlockInfo {
        name: "black_stained_glass",
        first_state: 6126,
        state_count: 1,
        default_state: 6126,
        props: &[],
    },
    BlockInfo {
        name: "oak_trapdoor",
        first_state: 6127,
        state_count: 64,
        default_state: 6142,
        props: &[
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
        first_state: 6191,
        state_count: 64,
        default_state: 6206,
        props: &[
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
        first_state: 6255,
        state_count: 64,
        default_state: 6270,
        props: &[
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
        first_state: 6319,
        state_count: 64,
        default_state: 6334,
        props: &[
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
        first_state: 6383,
        state_count: 64,
        default_state: 6398,
        props: &[
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
        first_state: 6447,
        state_count: 64,
        default_state: 6462,
        props: &[
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
        first_state: 6511,
        state_count: 64,
        default_state: 6526,
        props: &[
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
        first_state: 6575,
        state_count: 64,
        default_state: 6590,
        props: &[
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
        first_state: 6639,
        state_count: 64,
        default_state: 6654,
        props: &[
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
        first_state: 6703,
        state_count: 64,
        default_state: 6718,
        props: &[
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
        first_state: 6767,
        state_count: 1,
        default_state: 6767,
        props: &[],
    },
    BlockInfo {
        name: "mossy_stone_bricks",
        first_state: 6768,
        state_count: 1,
        default_state: 6768,
        props: &[],
    },
    BlockInfo {
        name: "cracked_stone_bricks",
        first_state: 6769,
        state_count: 1,
        default_state: 6769,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_stone_bricks",
        first_state: 6770,
        state_count: 1,
        default_state: 6770,
        props: &[],
    },
    BlockInfo {
        name: "packed_mud",
        first_state: 6771,
        state_count: 1,
        default_state: 6771,
        props: &[],
    },
    BlockInfo {
        name: "mud_bricks",
        first_state: 6772,
        state_count: 1,
        default_state: 6772,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone",
        first_state: 6773,
        state_count: 1,
        default_state: 6773,
        props: &[],
    },
    BlockInfo {
        name: "infested_cobblestone",
        first_state: 6774,
        state_count: 1,
        default_state: 6774,
        props: &[],
    },
    BlockInfo {
        name: "infested_stone_bricks",
        first_state: 6775,
        state_count: 1,
        default_state: 6775,
        props: &[],
    },
    BlockInfo {
        name: "infested_mossy_stone_bricks",
        first_state: 6776,
        state_count: 1,
        default_state: 6776,
        props: &[],
    },
    BlockInfo {
        name: "infested_cracked_stone_bricks",
        first_state: 6777,
        state_count: 1,
        default_state: 6777,
        props: &[],
    },
    BlockInfo {
        name: "infested_chiseled_stone_bricks",
        first_state: 6778,
        state_count: 1,
        default_state: 6778,
        props: &[],
    },
    BlockInfo {
        name: "brown_mushroom_block",
        first_state: 6779,
        state_count: 64,
        default_state: 6779,
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
        first_state: 6843,
        state_count: 64,
        default_state: 6843,
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
        first_state: 6907,
        state_count: 64,
        default_state: 6907,
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
        first_state: 6971,
        state_count: 32,
        default_state: 7002,
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
        first_state: 7003,
        state_count: 6,
        default_state: 7006,
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
        first_state: 7009,
        state_count: 32,
        default_state: 7040,
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
        first_state: 7041,
        state_count: 1,
        default_state: 7041,
        props: &[],
    },
    BlockInfo {
        name: "melon",
        first_state: 7042,
        state_count: 1,
        default_state: 7042,
        props: &[],
    },
    BlockInfo {
        name: "attached_pumpkin_stem",
        first_state: 7043,
        state_count: 4,
        default_state: 7043,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "attached_melon_stem",
        first_state: 7047,
        state_count: 4,
        default_state: 7047,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pumpkin_stem",
        first_state: 7051,
        state_count: 8,
        default_state: 7051,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "melon_stem",
        first_state: 7059,
        state_count: 8,
        default_state: 7059,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "vine",
        first_state: 7067,
        state_count: 32,
        default_state: 7098,
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
        first_state: 7099,
        state_count: 128,
        default_state: 7226,
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
        first_state: 7227,
        state_count: 32,
        default_state: 7234,
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
        first_state: 7259,
        state_count: 80,
        default_state: 7270,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7339,
        state_count: 80,
        default_state: 7350,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7419,
        state_count: 80,
        default_state: 7430,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7499,
        state_count: 2,
        default_state: 7500,
        props: &[PropInfo {
            name: "snowy",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lily_pad",
        first_state: 7501,
        state_count: 1,
        default_state: 7501,
        props: &[],
    },
    BlockInfo {
        name: "nether_bricks",
        first_state: 7502,
        state_count: 1,
        default_state: 7502,
        props: &[],
    },
    BlockInfo {
        name: "nether_brick_fence",
        first_state: 7503,
        state_count: 32,
        default_state: 7534,
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
        first_state: 7535,
        state_count: 80,
        default_state: 7546,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7615,
        state_count: 4,
        default_state: 7615,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "enchanting_table",
        first_state: 7619,
        state_count: 1,
        default_state: 7619,
        props: &[],
    },
    BlockInfo {
        name: "brewing_stand",
        first_state: 7620,
        state_count: 8,
        default_state: 7627,
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
        first_state: 7628,
        state_count: 1,
        default_state: 7628,
        props: &[],
    },
    BlockInfo {
        name: "water_cauldron",
        first_state: 7629,
        state_count: 3,
        default_state: 7629,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "lava_cauldron",
        first_state: 7632,
        state_count: 1,
        default_state: 7632,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow_cauldron",
        first_state: 7633,
        state_count: 3,
        default_state: 7633,
        props: &[PropInfo {
            name: "level",
            values: &["1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "end_portal",
        first_state: 7636,
        state_count: 1,
        default_state: 7636,
        props: &[],
    },
    BlockInfo {
        name: "end_portal_frame",
        first_state: 7637,
        state_count: 8,
        default_state: 7641,
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
        first_state: 7645,
        state_count: 1,
        default_state: 7645,
        props: &[],
    },
    BlockInfo {
        name: "dragon_egg",
        first_state: 7646,
        state_count: 1,
        default_state: 7646,
        props: &[],
    },
    BlockInfo {
        name: "redstone_lamp",
        first_state: 7647,
        state_count: 2,
        default_state: 7648,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cocoa",
        first_state: 7649,
        state_count: 12,
        default_state: 7649,
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
        first_state: 7661,
        state_count: 80,
        default_state: 7672,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7741,
        state_count: 1,
        default_state: 7741,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_emerald_ore",
        first_state: 7742,
        state_count: 1,
        default_state: 7742,
        props: &[],
    },
    BlockInfo {
        name: "ender_chest",
        first_state: 7743,
        state_count: 8,
        default_state: 7744,
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
        first_state: 7751,
        state_count: 16,
        default_state: 7760,
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
        first_state: 7767,
        state_count: 128,
        default_state: 7894,
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
        first_state: 7895,
        state_count: 1,
        default_state: 7895,
        props: &[],
    },
    BlockInfo {
        name: "spruce_stairs",
        first_state: 7896,
        state_count: 80,
        default_state: 7907,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 7976,
        state_count: 80,
        default_state: 7987,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8056,
        state_count: 80,
        default_state: 8067,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 8136,
        state_count: 12,
        default_state: 8142,
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
        first_state: 8148,
        state_count: 1,
        default_state: 8148,
        props: &[],
    },
    BlockInfo {
        name: "cobblestone_wall",
        first_state: 8149,
        state_count: 324,
        default_state: 8152,
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
        first_state: 8473,
        state_count: 324,
        default_state: 8476,
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
        first_state: 8797,
        state_count: 1,
        default_state: 8797,
        props: &[],
    },
    BlockInfo {
        name: "potted_torchflower",
        first_state: 8798,
        state_count: 1,
        default_state: 8798,
        props: &[],
    },
    BlockInfo {
        name: "potted_oak_sapling",
        first_state: 8799,
        state_count: 1,
        default_state: 8799,
        props: &[],
    },
    BlockInfo {
        name: "potted_spruce_sapling",
        first_state: 8800,
        state_count: 1,
        default_state: 8800,
        props: &[],
    },
    BlockInfo {
        name: "potted_birch_sapling",
        first_state: 8801,
        state_count: 1,
        default_state: 8801,
        props: &[],
    },
    BlockInfo {
        name: "potted_jungle_sapling",
        first_state: 8802,
        state_count: 1,
        default_state: 8802,
        props: &[],
    },
    BlockInfo {
        name: "potted_acacia_sapling",
        first_state: 8803,
        state_count: 1,
        default_state: 8803,
        props: &[],
    },
    BlockInfo {
        name: "potted_cherry_sapling",
        first_state: 8804,
        state_count: 1,
        default_state: 8804,
        props: &[],
    },
    BlockInfo {
        name: "potted_dark_oak_sapling",
        first_state: 8805,
        state_count: 1,
        default_state: 8805,
        props: &[],
    },
    BlockInfo {
        name: "potted_pale_oak_sapling",
        first_state: 8806,
        state_count: 1,
        default_state: 8806,
        props: &[],
    },
    BlockInfo {
        name: "potted_mangrove_propagule",
        first_state: 8807,
        state_count: 1,
        default_state: 8807,
        props: &[],
    },
    BlockInfo {
        name: "potted_fern",
        first_state: 8808,
        state_count: 1,
        default_state: 8808,
        props: &[],
    },
    BlockInfo {
        name: "potted_dandelion",
        first_state: 8809,
        state_count: 1,
        default_state: 8809,
        props: &[],
    },
    BlockInfo {
        name: "potted_poppy",
        first_state: 8810,
        state_count: 1,
        default_state: 8810,
        props: &[],
    },
    BlockInfo {
        name: "potted_blue_orchid",
        first_state: 8811,
        state_count: 1,
        default_state: 8811,
        props: &[],
    },
    BlockInfo {
        name: "potted_allium",
        first_state: 8812,
        state_count: 1,
        default_state: 8812,
        props: &[],
    },
    BlockInfo {
        name: "potted_azure_bluet",
        first_state: 8813,
        state_count: 1,
        default_state: 8813,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_tulip",
        first_state: 8814,
        state_count: 1,
        default_state: 8814,
        props: &[],
    },
    BlockInfo {
        name: "potted_orange_tulip",
        first_state: 8815,
        state_count: 1,
        default_state: 8815,
        props: &[],
    },
    BlockInfo {
        name: "potted_white_tulip",
        first_state: 8816,
        state_count: 1,
        default_state: 8816,
        props: &[],
    },
    BlockInfo {
        name: "potted_pink_tulip",
        first_state: 8817,
        state_count: 1,
        default_state: 8817,
        props: &[],
    },
    BlockInfo {
        name: "potted_oxeye_daisy",
        first_state: 8818,
        state_count: 1,
        default_state: 8818,
        props: &[],
    },
    BlockInfo {
        name: "potted_cornflower",
        first_state: 8819,
        state_count: 1,
        default_state: 8819,
        props: &[],
    },
    BlockInfo {
        name: "potted_lily_of_the_valley",
        first_state: 8820,
        state_count: 1,
        default_state: 8820,
        props: &[],
    },
    BlockInfo {
        name: "potted_wither_rose",
        first_state: 8821,
        state_count: 1,
        default_state: 8821,
        props: &[],
    },
    BlockInfo {
        name: "potted_red_mushroom",
        first_state: 8822,
        state_count: 1,
        default_state: 8822,
        props: &[],
    },
    BlockInfo {
        name: "potted_brown_mushroom",
        first_state: 8823,
        state_count: 1,
        default_state: 8823,
        props: &[],
    },
    BlockInfo {
        name: "potted_dead_bush",
        first_state: 8824,
        state_count: 1,
        default_state: 8824,
        props: &[],
    },
    BlockInfo {
        name: "potted_cactus",
        first_state: 8825,
        state_count: 1,
        default_state: 8825,
        props: &[],
    },
    BlockInfo {
        name: "carrots",
        first_state: 8826,
        state_count: 8,
        default_state: 8826,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "potatoes",
        first_state: 8834,
        state_count: 8,
        default_state: 8834,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5", "6", "7"],
        }],
    },
    BlockInfo {
        name: "oak_button",
        first_state: 8842,
        state_count: 24,
        default_state: 8851,
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
        first_state: 8866,
        state_count: 24,
        default_state: 8875,
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
        first_state: 8890,
        state_count: 24,
        default_state: 8899,
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
        first_state: 8914,
        state_count: 24,
        default_state: 8923,
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
        first_state: 8938,
        state_count: 24,
        default_state: 8947,
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
        first_state: 8962,
        state_count: 24,
        default_state: 8971,
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
        first_state: 8986,
        state_count: 24,
        default_state: 8995,
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
        first_state: 9010,
        state_count: 24,
        default_state: 9019,
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
        first_state: 9034,
        state_count: 24,
        default_state: 9043,
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
        first_state: 9058,
        state_count: 24,
        default_state: 9067,
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
        first_state: 9082,
        state_count: 32,
        default_state: 9098,
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
        first_state: 9114,
        state_count: 8,
        default_state: 9115,
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
        first_state: 9122,
        state_count: 32,
        default_state: 9138,
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
        first_state: 9154,
        state_count: 8,
        default_state: 9155,
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
        first_state: 9162,
        state_count: 32,
        default_state: 9178,
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
        first_state: 9194,
        state_count: 8,
        default_state: 9195,
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
        first_state: 9202,
        state_count: 32,
        default_state: 9218,
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
        first_state: 9234,
        state_count: 8,
        default_state: 9235,
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
        first_state: 9242,
        state_count: 32,
        default_state: 9258,
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
        first_state: 9274,
        state_count: 8,
        default_state: 9275,
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
        first_state: 9282,
        state_count: 32,
        default_state: 9298,
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
        first_state: 9314,
        state_count: 8,
        default_state: 9315,
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
        first_state: 9322,
        state_count: 32,
        default_state: 9338,
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
        first_state: 9354,
        state_count: 8,
        default_state: 9355,
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
        first_state: 9362,
        state_count: 4,
        default_state: 9362,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "chipped_anvil",
        first_state: 9366,
        state_count: 4,
        default_state: 9366,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "damaged_anvil",
        first_state: 9370,
        state_count: 4,
        default_state: 9370,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "trapped_chest",
        first_state: 9374,
        state_count: 24,
        default_state: 9375,
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
        first_state: 9398,
        state_count: 16,
        default_state: 9398,
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
        first_state: 9414,
        state_count: 16,
        default_state: 9414,
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
        first_state: 9430,
        state_count: 16,
        default_state: 9431,
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
        first_state: 9446,
        state_count: 32,
        default_state: 9462,
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
        first_state: 9478,
        state_count: 1,
        default_state: 9478,
        props: &[],
    },
    BlockInfo {
        name: "nether_quartz_ore",
        first_state: 9479,
        state_count: 1,
        default_state: 9479,
        props: &[],
    },
    BlockInfo {
        name: "hopper",
        first_state: 9480,
        state_count: 10,
        default_state: 9480,
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
        first_state: 9490,
        state_count: 1,
        default_state: 9490,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_quartz_block",
        first_state: 9491,
        state_count: 1,
        default_state: 9491,
        props: &[],
    },
    BlockInfo {
        name: "quartz_pillar",
        first_state: 9492,
        state_count: 3,
        default_state: 9493,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "quartz_stairs",
        first_state: 9495,
        state_count: 80,
        default_state: 9506,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 9575,
        state_count: 24,
        default_state: 9588,
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
        first_state: 9599,
        state_count: 12,
        default_state: 9600,
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
        first_state: 9611,
        state_count: 1,
        default_state: 9611,
        props: &[],
    },
    BlockInfo {
        name: "orange_terracotta",
        first_state: 9612,
        state_count: 1,
        default_state: 9612,
        props: &[],
    },
    BlockInfo {
        name: "magenta_terracotta",
        first_state: 9613,
        state_count: 1,
        default_state: 9613,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_terracotta",
        first_state: 9614,
        state_count: 1,
        default_state: 9614,
        props: &[],
    },
    BlockInfo {
        name: "yellow_terracotta",
        first_state: 9615,
        state_count: 1,
        default_state: 9615,
        props: &[],
    },
    BlockInfo {
        name: "lime_terracotta",
        first_state: 9616,
        state_count: 1,
        default_state: 9616,
        props: &[],
    },
    BlockInfo {
        name: "pink_terracotta",
        first_state: 9617,
        state_count: 1,
        default_state: 9617,
        props: &[],
    },
    BlockInfo {
        name: "gray_terracotta",
        first_state: 9618,
        state_count: 1,
        default_state: 9618,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_terracotta",
        first_state: 9619,
        state_count: 1,
        default_state: 9619,
        props: &[],
    },
    BlockInfo {
        name: "cyan_terracotta",
        first_state: 9620,
        state_count: 1,
        default_state: 9620,
        props: &[],
    },
    BlockInfo {
        name: "purple_terracotta",
        first_state: 9621,
        state_count: 1,
        default_state: 9621,
        props: &[],
    },
    BlockInfo {
        name: "blue_terracotta",
        first_state: 9622,
        state_count: 1,
        default_state: 9622,
        props: &[],
    },
    BlockInfo {
        name: "brown_terracotta",
        first_state: 9623,
        state_count: 1,
        default_state: 9623,
        props: &[],
    },
    BlockInfo {
        name: "green_terracotta",
        first_state: 9624,
        state_count: 1,
        default_state: 9624,
        props: &[],
    },
    BlockInfo {
        name: "red_terracotta",
        first_state: 9625,
        state_count: 1,
        default_state: 9625,
        props: &[],
    },
    BlockInfo {
        name: "black_terracotta",
        first_state: 9626,
        state_count: 1,
        default_state: 9626,
        props: &[],
    },
    BlockInfo {
        name: "white_stained_glass_pane",
        first_state: 9627,
        state_count: 32,
        default_state: 9658,
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
        first_state: 9659,
        state_count: 32,
        default_state: 9690,
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
        first_state: 9691,
        state_count: 32,
        default_state: 9722,
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
        first_state: 9723,
        state_count: 32,
        default_state: 9754,
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
        first_state: 9755,
        state_count: 32,
        default_state: 9786,
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
        first_state: 9787,
        state_count: 32,
        default_state: 9818,
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
        first_state: 9819,
        state_count: 32,
        default_state: 9850,
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
        first_state: 9851,
        state_count: 32,
        default_state: 9882,
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
        first_state: 9883,
        state_count: 32,
        default_state: 9914,
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
        first_state: 9915,
        state_count: 32,
        default_state: 9946,
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
        first_state: 9947,
        state_count: 32,
        default_state: 9978,
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
        first_state: 9979,
        state_count: 32,
        default_state: 10010,
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
        first_state: 10011,
        state_count: 32,
        default_state: 10042,
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
        first_state: 10043,
        state_count: 32,
        default_state: 10074,
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
        first_state: 10075,
        state_count: 32,
        default_state: 10106,
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
        first_state: 10107,
        state_count: 32,
        default_state: 10138,
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
        first_state: 10139,
        state_count: 80,
        default_state: 10150,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10219,
        state_count: 80,
        default_state: 10230,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10299,
        state_count: 80,
        default_state: 10310,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10379,
        state_count: 80,
        default_state: 10390,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10459,
        state_count: 80,
        default_state: 10470,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10539,
        state_count: 80,
        default_state: 10550,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10619,
        state_count: 80,
        default_state: 10630,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10699,
        state_count: 1,
        default_state: 10699,
        props: &[],
    },
    BlockInfo {
        name: "barrier",
        first_state: 10700,
        state_count: 2,
        default_state: 10701,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light",
        first_state: 10702,
        state_count: 32,
        default_state: 10733,
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
        first_state: 10734,
        state_count: 64,
        default_state: 10749,
        props: &[
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
        first_state: 10798,
        state_count: 1,
        default_state: 10798,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_bricks",
        first_state: 10799,
        state_count: 1,
        default_state: 10799,
        props: &[],
    },
    BlockInfo {
        name: "dark_prismarine",
        first_state: 10800,
        state_count: 1,
        default_state: 10800,
        props: &[],
    },
    BlockInfo {
        name: "prismarine_stairs",
        first_state: 10801,
        state_count: 80,
        default_state: 10812,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10881,
        state_count: 80,
        default_state: 10892,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 10961,
        state_count: 80,
        default_state: 10972,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        name: "prismarine_brick_slab",
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
        name: "dark_prismarine_slab",
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
        name: "sea_lantern",
        first_state: 11059,
        state_count: 1,
        default_state: 11059,
        props: &[],
    },
    BlockInfo {
        name: "hay_block",
        first_state: 11060,
        state_count: 3,
        default_state: 11061,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "white_carpet",
        first_state: 11063,
        state_count: 1,
        default_state: 11063,
        props: &[],
    },
    BlockInfo {
        name: "orange_carpet",
        first_state: 11064,
        state_count: 1,
        default_state: 11064,
        props: &[],
    },
    BlockInfo {
        name: "magenta_carpet",
        first_state: 11065,
        state_count: 1,
        default_state: 11065,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_carpet",
        first_state: 11066,
        state_count: 1,
        default_state: 11066,
        props: &[],
    },
    BlockInfo {
        name: "yellow_carpet",
        first_state: 11067,
        state_count: 1,
        default_state: 11067,
        props: &[],
    },
    BlockInfo {
        name: "lime_carpet",
        first_state: 11068,
        state_count: 1,
        default_state: 11068,
        props: &[],
    },
    BlockInfo {
        name: "pink_carpet",
        first_state: 11069,
        state_count: 1,
        default_state: 11069,
        props: &[],
    },
    BlockInfo {
        name: "gray_carpet",
        first_state: 11070,
        state_count: 1,
        default_state: 11070,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_carpet",
        first_state: 11071,
        state_count: 1,
        default_state: 11071,
        props: &[],
    },
    BlockInfo {
        name: "cyan_carpet",
        first_state: 11072,
        state_count: 1,
        default_state: 11072,
        props: &[],
    },
    BlockInfo {
        name: "purple_carpet",
        first_state: 11073,
        state_count: 1,
        default_state: 11073,
        props: &[],
    },
    BlockInfo {
        name: "blue_carpet",
        first_state: 11074,
        state_count: 1,
        default_state: 11074,
        props: &[],
    },
    BlockInfo {
        name: "brown_carpet",
        first_state: 11075,
        state_count: 1,
        default_state: 11075,
        props: &[],
    },
    BlockInfo {
        name: "green_carpet",
        first_state: 11076,
        state_count: 1,
        default_state: 11076,
        props: &[],
    },
    BlockInfo {
        name: "red_carpet",
        first_state: 11077,
        state_count: 1,
        default_state: 11077,
        props: &[],
    },
    BlockInfo {
        name: "black_carpet",
        first_state: 11078,
        state_count: 1,
        default_state: 11078,
        props: &[],
    },
    BlockInfo {
        name: "terracotta",
        first_state: 11079,
        state_count: 1,
        default_state: 11079,
        props: &[],
    },
    BlockInfo {
        name: "coal_block",
        first_state: 11080,
        state_count: 1,
        default_state: 11080,
        props: &[],
    },
    BlockInfo {
        name: "packed_ice",
        first_state: 11081,
        state_count: 1,
        default_state: 11081,
        props: &[],
    },
    BlockInfo {
        name: "sunflower",
        first_state: 11082,
        state_count: 2,
        default_state: 11083,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "lilac",
        first_state: 11084,
        state_count: 2,
        default_state: 11085,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "rose_bush",
        first_state: 11086,
        state_count: 2,
        default_state: 11087,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "peony",
        first_state: 11088,
        state_count: 2,
        default_state: 11089,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "tall_grass",
        first_state: 11090,
        state_count: 2,
        default_state: 11091,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "large_fern",
        first_state: 11092,
        state_count: 2,
        default_state: 11093,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "white_banner",
        first_state: 11094,
        state_count: 16,
        default_state: 11094,
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
        first_state: 11110,
        state_count: 16,
        default_state: 11110,
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
        first_state: 11126,
        state_count: 16,
        default_state: 11126,
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
        first_state: 11142,
        state_count: 16,
        default_state: 11142,
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
        first_state: 11158,
        state_count: 16,
        default_state: 11158,
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
        first_state: 11174,
        state_count: 16,
        default_state: 11174,
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
        first_state: 11190,
        state_count: 16,
        default_state: 11190,
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
        first_state: 11206,
        state_count: 16,
        default_state: 11206,
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
        first_state: 11222,
        state_count: 16,
        default_state: 11222,
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
        first_state: 11238,
        state_count: 16,
        default_state: 11238,
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
        first_state: 11254,
        state_count: 16,
        default_state: 11254,
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
        first_state: 11270,
        state_count: 16,
        default_state: 11270,
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
        first_state: 11286,
        state_count: 16,
        default_state: 11286,
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
        first_state: 11302,
        state_count: 16,
        default_state: 11302,
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
        first_state: 11318,
        state_count: 16,
        default_state: 11318,
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
        first_state: 11334,
        state_count: 16,
        default_state: 11334,
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
        first_state: 11350,
        state_count: 4,
        default_state: 11350,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_wall_banner",
        first_state: 11354,
        state_count: 4,
        default_state: 11354,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_wall_banner",
        first_state: 11358,
        state_count: 4,
        default_state: 11358,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_wall_banner",
        first_state: 11362,
        state_count: 4,
        default_state: 11362,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_wall_banner",
        first_state: 11366,
        state_count: 4,
        default_state: 11366,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_wall_banner",
        first_state: 11370,
        state_count: 4,
        default_state: 11370,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_wall_banner",
        first_state: 11374,
        state_count: 4,
        default_state: 11374,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_wall_banner",
        first_state: 11378,
        state_count: 4,
        default_state: 11378,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_wall_banner",
        first_state: 11382,
        state_count: 4,
        default_state: 11382,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_wall_banner",
        first_state: 11386,
        state_count: 4,
        default_state: 11386,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_wall_banner",
        first_state: 11390,
        state_count: 4,
        default_state: 11390,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_wall_banner",
        first_state: 11394,
        state_count: 4,
        default_state: 11394,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_wall_banner",
        first_state: 11398,
        state_count: 4,
        default_state: 11398,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_wall_banner",
        first_state: 11402,
        state_count: 4,
        default_state: 11402,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_wall_banner",
        first_state: 11406,
        state_count: 4,
        default_state: 11406,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_wall_banner",
        first_state: 11410,
        state_count: 4,
        default_state: 11410,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_sandstone",
        first_state: 11414,
        state_count: 1,
        default_state: 11414,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_red_sandstone",
        first_state: 11415,
        state_count: 1,
        default_state: 11415,
        props: &[],
    },
    BlockInfo {
        name: "cut_red_sandstone",
        first_state: 11416,
        state_count: 1,
        default_state: 11416,
        props: &[],
    },
    BlockInfo {
        name: "red_sandstone_stairs",
        first_state: 11417,
        state_count: 80,
        default_state: 11428,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 11497,
        state_count: 6,
        default_state: 11500,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11503,
        state_count: 6,
        default_state: 11506,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11509,
        state_count: 6,
        default_state: 11512,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11515,
        state_count: 6,
        default_state: 11518,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11521,
        state_count: 6,
        default_state: 11524,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11527,
        state_count: 6,
        default_state: 11530,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11533,
        state_count: 6,
        default_state: 11536,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11539,
        state_count: 6,
        default_state: 11542,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11545,
        state_count: 6,
        default_state: 11548,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11551,
        state_count: 6,
        default_state: 11554,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11557,
        state_count: 6,
        default_state: 11560,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11563,
        state_count: 6,
        default_state: 11566,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11569,
        state_count: 6,
        default_state: 11572,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11575,
        state_count: 6,
        default_state: 11578,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11581,
        state_count: 6,
        default_state: 11584,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11587,
        state_count: 6,
        default_state: 11590,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11593,
        state_count: 6,
        default_state: 11596,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11599,
        state_count: 6,
        default_state: 11602,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11605,
        state_count: 6,
        default_state: 11608,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11611,
        state_count: 6,
        default_state: 11614,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11617,
        state_count: 6,
        default_state: 11620,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11623,
        state_count: 6,
        default_state: 11626,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11629,
        state_count: 6,
        default_state: 11632,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11635,
        state_count: 6,
        default_state: 11638,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11641,
        state_count: 6,
        default_state: 11644,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 11647,
        state_count: 1,
        default_state: 11647,
        props: &[],
    },
    BlockInfo {
        name: "smooth_sandstone",
        first_state: 11648,
        state_count: 1,
        default_state: 11648,
        props: &[],
    },
    BlockInfo {
        name: "smooth_quartz",
        first_state: 11649,
        state_count: 1,
        default_state: 11649,
        props: &[],
    },
    BlockInfo {
        name: "smooth_red_sandstone",
        first_state: 11650,
        state_count: 1,
        default_state: 11650,
        props: &[],
    },
    BlockInfo {
        name: "spruce_fence_gate",
        first_state: 11651,
        state_count: 32,
        default_state: 11658,
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
        first_state: 11683,
        state_count: 32,
        default_state: 11690,
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
        first_state: 11715,
        state_count: 32,
        default_state: 11722,
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
        first_state: 11747,
        state_count: 32,
        default_state: 11754,
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
        first_state: 11779,
        state_count: 32,
        default_state: 11786,
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
        first_state: 11811,
        state_count: 32,
        default_state: 11818,
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
        first_state: 11843,
        state_count: 32,
        default_state: 11850,
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
        first_state: 11875,
        state_count: 32,
        default_state: 11882,
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
        first_state: 11907,
        state_count: 32,
        default_state: 11914,
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
        first_state: 11939,
        state_count: 32,
        default_state: 11970,
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
        first_state: 11971,
        state_count: 32,
        default_state: 12002,
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
        first_state: 12003,
        state_count: 32,
        default_state: 12034,
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
        first_state: 12035,
        state_count: 32,
        default_state: 12066,
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
        first_state: 12067,
        state_count: 32,
        default_state: 12098,
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
        first_state: 12099,
        state_count: 32,
        default_state: 12130,
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
        first_state: 12131,
        state_count: 32,
        default_state: 12162,
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
        first_state: 12163,
        state_count: 32,
        default_state: 12194,
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
        first_state: 12195,
        state_count: 32,
        default_state: 12226,
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
        first_state: 12227,
        state_count: 64,
        default_state: 12238,
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
        first_state: 12291,
        state_count: 64,
        default_state: 12302,
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
        first_state: 12355,
        state_count: 64,
        default_state: 12366,
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
        first_state: 12419,
        state_count: 64,
        default_state: 12430,
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
        first_state: 12483,
        state_count: 64,
        default_state: 12494,
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
        first_state: 12547,
        state_count: 64,
        default_state: 12558,
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
        first_state: 12611,
        state_count: 64,
        default_state: 12622,
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
        first_state: 12675,
        state_count: 64,
        default_state: 12686,
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
        first_state: 12739,
        state_count: 64,
        default_state: 12750,
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
        first_state: 12803,
        state_count: 6,
        default_state: 12807,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "chorus_plant",
        first_state: 12809,
        state_count: 64,
        default_state: 12872,
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
        first_state: 12873,
        state_count: 6,
        default_state: 12873,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3", "4", "5"],
        }],
    },
    BlockInfo {
        name: "purpur_block",
        first_state: 12879,
        state_count: 1,
        default_state: 12879,
        props: &[],
    },
    BlockInfo {
        name: "purpur_pillar",
        first_state: 12880,
        state_count: 3,
        default_state: 12881,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "purpur_stairs",
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
        name: "end_stone_bricks",
        first_state: 12963,
        state_count: 1,
        default_state: 12963,
        props: &[],
    },
    BlockInfo {
        name: "torchflower_crop",
        first_state: 12964,
        state_count: 2,
        default_state: 12964,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1"],
        }],
    },
    BlockInfo {
        name: "pitcher_crop",
        first_state: 12966,
        state_count: 10,
        default_state: 12967,
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
        first_state: 12976,
        state_count: 2,
        default_state: 12977,
        props: &[PropInfo {
            name: "half",
            values: &["upper", "lower"],
        }],
    },
    BlockInfo {
        name: "beetroots",
        first_state: 12978,
        state_count: 4,
        default_state: 12978,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "dirt_path",
        first_state: 12982,
        state_count: 1,
        default_state: 12982,
        props: &[],
    },
    BlockInfo {
        name: "end_gateway",
        first_state: 12983,
        state_count: 1,
        default_state: 12983,
        props: &[],
    },
    BlockInfo {
        name: "repeating_command_block",
        first_state: 12984,
        state_count: 12,
        default_state: 12990,
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
        first_state: 12996,
        state_count: 12,
        default_state: 13002,
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
        first_state: 13008,
        state_count: 4,
        default_state: 13008,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "magma_block",
        first_state: 13012,
        state_count: 1,
        default_state: 13012,
        props: &[],
    },
    BlockInfo {
        name: "nether_wart_block",
        first_state: 13013,
        state_count: 1,
        default_state: 13013,
        props: &[],
    },
    BlockInfo {
        name: "red_nether_bricks",
        first_state: 13014,
        state_count: 1,
        default_state: 13014,
        props: &[],
    },
    BlockInfo {
        name: "bone_block",
        first_state: 13015,
        state_count: 3,
        default_state: 13016,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "structure_void",
        first_state: 13018,
        state_count: 1,
        default_state: 13018,
        props: &[],
    },
    BlockInfo {
        name: "observer",
        first_state: 13019,
        state_count: 12,
        default_state: 13024,
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
        first_state: 13031,
        state_count: 6,
        default_state: 13035,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_shulker_box",
        first_state: 13037,
        state_count: 6,
        default_state: 13041,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "orange_shulker_box",
        first_state: 13043,
        state_count: 6,
        default_state: 13047,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "magenta_shulker_box",
        first_state: 13049,
        state_count: 6,
        default_state: 13053,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_blue_shulker_box",
        first_state: 13055,
        state_count: 6,
        default_state: 13059,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "yellow_shulker_box",
        first_state: 13061,
        state_count: 6,
        default_state: 13065,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "lime_shulker_box",
        first_state: 13067,
        state_count: 6,
        default_state: 13071,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "pink_shulker_box",
        first_state: 13073,
        state_count: 6,
        default_state: 13077,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "gray_shulker_box",
        first_state: 13079,
        state_count: 6,
        default_state: 13083,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "light_gray_shulker_box",
        first_state: 13085,
        state_count: 6,
        default_state: 13089,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "cyan_shulker_box",
        first_state: 13091,
        state_count: 6,
        default_state: 13095,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "purple_shulker_box",
        first_state: 13097,
        state_count: 6,
        default_state: 13101,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "blue_shulker_box",
        first_state: 13103,
        state_count: 6,
        default_state: 13107,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "brown_shulker_box",
        first_state: 13109,
        state_count: 6,
        default_state: 13113,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "green_shulker_box",
        first_state: 13115,
        state_count: 6,
        default_state: 13119,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "red_shulker_box",
        first_state: 13121,
        state_count: 6,
        default_state: 13125,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "black_shulker_box",
        first_state: 13127,
        state_count: 6,
        default_state: 13131,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "east", "south", "west", "up", "down"],
        }],
    },
    BlockInfo {
        name: "white_glazed_terracotta",
        first_state: 13133,
        state_count: 4,
        default_state: 13133,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "orange_glazed_terracotta",
        first_state: 13137,
        state_count: 4,
        default_state: 13137,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "magenta_glazed_terracotta",
        first_state: 13141,
        state_count: 4,
        default_state: 13141,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_blue_glazed_terracotta",
        first_state: 13145,
        state_count: 4,
        default_state: 13145,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "yellow_glazed_terracotta",
        first_state: 13149,
        state_count: 4,
        default_state: 13149,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "lime_glazed_terracotta",
        first_state: 13153,
        state_count: 4,
        default_state: 13153,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "pink_glazed_terracotta",
        first_state: 13157,
        state_count: 4,
        default_state: 13157,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "gray_glazed_terracotta",
        first_state: 13161,
        state_count: 4,
        default_state: 13161,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "light_gray_glazed_terracotta",
        first_state: 13165,
        state_count: 4,
        default_state: 13165,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "cyan_glazed_terracotta",
        first_state: 13169,
        state_count: 4,
        default_state: 13169,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "purple_glazed_terracotta",
        first_state: 13173,
        state_count: 4,
        default_state: 13173,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "blue_glazed_terracotta",
        first_state: 13177,
        state_count: 4,
        default_state: 13177,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "brown_glazed_terracotta",
        first_state: 13181,
        state_count: 4,
        default_state: 13181,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "green_glazed_terracotta",
        first_state: 13185,
        state_count: 4,
        default_state: 13185,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "red_glazed_terracotta",
        first_state: 13189,
        state_count: 4,
        default_state: 13189,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "black_glazed_terracotta",
        first_state: 13193,
        state_count: 4,
        default_state: 13193,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "white_concrete",
        first_state: 13197,
        state_count: 1,
        default_state: 13197,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete",
        first_state: 13198,
        state_count: 1,
        default_state: 13198,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete",
        first_state: 13199,
        state_count: 1,
        default_state: 13199,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete",
        first_state: 13200,
        state_count: 1,
        default_state: 13200,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete",
        first_state: 13201,
        state_count: 1,
        default_state: 13201,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete",
        first_state: 13202,
        state_count: 1,
        default_state: 13202,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete",
        first_state: 13203,
        state_count: 1,
        default_state: 13203,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete",
        first_state: 13204,
        state_count: 1,
        default_state: 13204,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete",
        first_state: 13205,
        state_count: 1,
        default_state: 13205,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete",
        first_state: 13206,
        state_count: 1,
        default_state: 13206,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete",
        first_state: 13207,
        state_count: 1,
        default_state: 13207,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete",
        first_state: 13208,
        state_count: 1,
        default_state: 13208,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete",
        first_state: 13209,
        state_count: 1,
        default_state: 13209,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete",
        first_state: 13210,
        state_count: 1,
        default_state: 13210,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete",
        first_state: 13211,
        state_count: 1,
        default_state: 13211,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete",
        first_state: 13212,
        state_count: 1,
        default_state: 13212,
        props: &[],
    },
    BlockInfo {
        name: "white_concrete_powder",
        first_state: 13213,
        state_count: 1,
        default_state: 13213,
        props: &[],
    },
    BlockInfo {
        name: "orange_concrete_powder",
        first_state: 13214,
        state_count: 1,
        default_state: 13214,
        props: &[],
    },
    BlockInfo {
        name: "magenta_concrete_powder",
        first_state: 13215,
        state_count: 1,
        default_state: 13215,
        props: &[],
    },
    BlockInfo {
        name: "light_blue_concrete_powder",
        first_state: 13216,
        state_count: 1,
        default_state: 13216,
        props: &[],
    },
    BlockInfo {
        name: "yellow_concrete_powder",
        first_state: 13217,
        state_count: 1,
        default_state: 13217,
        props: &[],
    },
    BlockInfo {
        name: "lime_concrete_powder",
        first_state: 13218,
        state_count: 1,
        default_state: 13218,
        props: &[],
    },
    BlockInfo {
        name: "pink_concrete_powder",
        first_state: 13219,
        state_count: 1,
        default_state: 13219,
        props: &[],
    },
    BlockInfo {
        name: "gray_concrete_powder",
        first_state: 13220,
        state_count: 1,
        default_state: 13220,
        props: &[],
    },
    BlockInfo {
        name: "light_gray_concrete_powder",
        first_state: 13221,
        state_count: 1,
        default_state: 13221,
        props: &[],
    },
    BlockInfo {
        name: "cyan_concrete_powder",
        first_state: 13222,
        state_count: 1,
        default_state: 13222,
        props: &[],
    },
    BlockInfo {
        name: "purple_concrete_powder",
        first_state: 13223,
        state_count: 1,
        default_state: 13223,
        props: &[],
    },
    BlockInfo {
        name: "blue_concrete_powder",
        first_state: 13224,
        state_count: 1,
        default_state: 13224,
        props: &[],
    },
    BlockInfo {
        name: "brown_concrete_powder",
        first_state: 13225,
        state_count: 1,
        default_state: 13225,
        props: &[],
    },
    BlockInfo {
        name: "green_concrete_powder",
        first_state: 13226,
        state_count: 1,
        default_state: 13226,
        props: &[],
    },
    BlockInfo {
        name: "red_concrete_powder",
        first_state: 13227,
        state_count: 1,
        default_state: 13227,
        props: &[],
    },
    BlockInfo {
        name: "black_concrete_powder",
        first_state: 13228,
        state_count: 1,
        default_state: 13228,
        props: &[],
    },
    BlockInfo {
        name: "kelp",
        first_state: 13229,
        state_count: 26,
        default_state: 13229,
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
        first_state: 13255,
        state_count: 1,
        default_state: 13255,
        props: &[],
    },
    BlockInfo {
        name: "dried_kelp_block",
        first_state: 13256,
        state_count: 1,
        default_state: 13256,
        props: &[],
    },
    BlockInfo {
        name: "turtle_egg",
        first_state: 13257,
        state_count: 12,
        default_state: 13257,
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
        first_state: 13269,
        state_count: 3,
        default_state: 13269,
        props: &[PropInfo {
            name: "hatch",
            values: &["0", "1", "2"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_block",
        first_state: 13272,
        state_count: 1,
        default_state: 13272,
        props: &[],
    },
    BlockInfo {
        name: "dead_brain_coral_block",
        first_state: 13273,
        state_count: 1,
        default_state: 13273,
        props: &[],
    },
    BlockInfo {
        name: "dead_bubble_coral_block",
        first_state: 13274,
        state_count: 1,
        default_state: 13274,
        props: &[],
    },
    BlockInfo {
        name: "dead_fire_coral_block",
        first_state: 13275,
        state_count: 1,
        default_state: 13275,
        props: &[],
    },
    BlockInfo {
        name: "dead_horn_coral_block",
        first_state: 13276,
        state_count: 1,
        default_state: 13276,
        props: &[],
    },
    BlockInfo {
        name: "tube_coral_block",
        first_state: 13277,
        state_count: 1,
        default_state: 13277,
        props: &[],
    },
    BlockInfo {
        name: "brain_coral_block",
        first_state: 13278,
        state_count: 1,
        default_state: 13278,
        props: &[],
    },
    BlockInfo {
        name: "bubble_coral_block",
        first_state: 13279,
        state_count: 1,
        default_state: 13279,
        props: &[],
    },
    BlockInfo {
        name: "fire_coral_block",
        first_state: 13280,
        state_count: 1,
        default_state: 13280,
        props: &[],
    },
    BlockInfo {
        name: "horn_coral_block",
        first_state: 13281,
        state_count: 1,
        default_state: 13281,
        props: &[],
    },
    BlockInfo {
        name: "dead_tube_coral",
        first_state: 13282,
        state_count: 2,
        default_state: 13282,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral",
        first_state: 13284,
        state_count: 2,
        default_state: 13284,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral",
        first_state: 13286,
        state_count: 2,
        default_state: 13286,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral",
        first_state: 13288,
        state_count: 2,
        default_state: 13288,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral",
        first_state: 13290,
        state_count: 2,
        default_state: 13290,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral",
        first_state: 13292,
        state_count: 2,
        default_state: 13292,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral",
        first_state: 13294,
        state_count: 2,
        default_state: 13294,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral",
        first_state: 13296,
        state_count: 2,
        default_state: 13296,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral",
        first_state: 13298,
        state_count: 2,
        default_state: 13298,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral",
        first_state: 13300,
        state_count: 2,
        default_state: 13300,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_fan",
        first_state: 13302,
        state_count: 2,
        default_state: 13302,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_brain_coral_fan",
        first_state: 13304,
        state_count: 2,
        default_state: 13304,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_bubble_coral_fan",
        first_state: 13306,
        state_count: 2,
        default_state: 13306,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_fire_coral_fan",
        first_state: 13308,
        state_count: 2,
        default_state: 13308,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_horn_coral_fan",
        first_state: 13310,
        state_count: 2,
        default_state: 13310,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "tube_coral_fan",
        first_state: 13312,
        state_count: 2,
        default_state: 13312,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brain_coral_fan",
        first_state: 13314,
        state_count: 2,
        default_state: 13314,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bubble_coral_fan",
        first_state: 13316,
        state_count: 2,
        default_state: 13316,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "fire_coral_fan",
        first_state: 13318,
        state_count: 2,
        default_state: 13318,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "horn_coral_fan",
        first_state: 13320,
        state_count: 2,
        default_state: 13320,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "dead_tube_coral_wall_fan",
        first_state: 13322,
        state_count: 8,
        default_state: 13322,
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
        first_state: 13330,
        state_count: 8,
        default_state: 13330,
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
        first_state: 13338,
        state_count: 8,
        default_state: 13338,
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
        first_state: 13346,
        state_count: 8,
        default_state: 13346,
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
        first_state: 13354,
        state_count: 8,
        default_state: 13354,
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
        first_state: 13362,
        state_count: 8,
        default_state: 13362,
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
        first_state: 13370,
        state_count: 8,
        default_state: 13370,
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
        first_state: 13378,
        state_count: 8,
        default_state: 13378,
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
        first_state: 13386,
        state_count: 8,
        default_state: 13386,
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
        first_state: 13394,
        state_count: 8,
        default_state: 13394,
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
        first_state: 13402,
        state_count: 8,
        default_state: 13402,
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
        first_state: 13410,
        state_count: 1,
        default_state: 13410,
        props: &[],
    },
    BlockInfo {
        name: "conduit",
        first_state: 13411,
        state_count: 2,
        default_state: 13411,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "bamboo_sapling",
        first_state: 13413,
        state_count: 1,
        default_state: 13413,
        props: &[],
    },
    BlockInfo {
        name: "bamboo",
        first_state: 13414,
        state_count: 12,
        default_state: 13414,
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
        first_state: 13426,
        state_count: 1,
        default_state: 13426,
        props: &[],
    },
    BlockInfo {
        name: "void_air",
        first_state: 13427,
        state_count: 1,
        default_state: 13427,
        props: &[],
    },
    BlockInfo {
        name: "cave_air",
        first_state: 13428,
        state_count: 1,
        default_state: 13428,
        props: &[],
    },
    BlockInfo {
        name: "bubble_column",
        first_state: 13429,
        state_count: 2,
        default_state: 13429,
        props: &[PropInfo {
            name: "drag",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_granite_stairs",
        first_state: 13431,
        state_count: 80,
        default_state: 13442,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13511,
        state_count: 80,
        default_state: 13522,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13591,
        state_count: 80,
        default_state: 13602,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13671,
        state_count: 80,
        default_state: 13682,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13751,
        state_count: 80,
        default_state: 13762,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13831,
        state_count: 80,
        default_state: 13842,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13911,
        state_count: 80,
        default_state: 13922,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 13991,
        state_count: 80,
        default_state: 14002,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14071,
        state_count: 80,
        default_state: 14082,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14151,
        state_count: 80,
        default_state: 14162,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14231,
        state_count: 80,
        default_state: 14242,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14311,
        state_count: 80,
        default_state: 14322,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14391,
        state_count: 80,
        default_state: 14402,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14471,
        state_count: 80,
        default_state: 14482,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 14551,
        state_count: 6,
        default_state: 14554,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14557,
        state_count: 6,
        default_state: 14560,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14563,
        state_count: 6,
        default_state: 14566,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14569,
        state_count: 6,
        default_state: 14572,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14575,
        state_count: 6,
        default_state: 14578,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14581,
        state_count: 6,
        default_state: 14584,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14587,
        state_count: 6,
        default_state: 14590,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14593,
        state_count: 6,
        default_state: 14596,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14599,
        state_count: 6,
        default_state: 14602,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14605,
        state_count: 6,
        default_state: 14608,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14611,
        state_count: 6,
        default_state: 14614,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14617,
        state_count: 6,
        default_state: 14620,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14623,
        state_count: 6,
        default_state: 14626,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 14629,
        state_count: 324,
        default_state: 14632,
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
        first_state: 14953,
        state_count: 324,
        default_state: 14956,
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
        first_state: 15277,
        state_count: 324,
        default_state: 15280,
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
        first_state: 15601,
        state_count: 324,
        default_state: 15604,
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
        first_state: 15925,
        state_count: 324,
        default_state: 15928,
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
        first_state: 16249,
        state_count: 324,
        default_state: 16252,
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
        first_state: 16573,
        state_count: 324,
        default_state: 16576,
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
        first_state: 16897,
        state_count: 324,
        default_state: 16900,
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
        first_state: 17221,
        state_count: 324,
        default_state: 17224,
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
        first_state: 17545,
        state_count: 324,
        default_state: 17548,
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
        first_state: 17869,
        state_count: 324,
        default_state: 17872,
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
        first_state: 18193,
        state_count: 324,
        default_state: 18196,
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
        first_state: 18517,
        state_count: 324,
        default_state: 18520,
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
        first_state: 18841,
        state_count: 32,
        default_state: 18872,
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
        first_state: 18873,
        state_count: 4,
        default_state: 18873,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "barrel",
        first_state: 18877,
        state_count: 12,
        default_state: 18878,
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
        first_state: 18889,
        state_count: 8,
        default_state: 18890,
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
        first_state: 18897,
        state_count: 8,
        default_state: 18898,
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
        first_state: 18905,
        state_count: 1,
        default_state: 18905,
        props: &[],
    },
    BlockInfo {
        name: "fletching_table",
        first_state: 18906,
        state_count: 1,
        default_state: 18906,
        props: &[],
    },
    BlockInfo {
        name: "grindstone",
        first_state: 18907,
        state_count: 12,
        default_state: 18911,
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
        first_state: 18919,
        state_count: 16,
        default_state: 18922,
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
        first_state: 18935,
        state_count: 1,
        default_state: 18935,
        props: &[],
    },
    BlockInfo {
        name: "stonecutter",
        first_state: 18936,
        state_count: 4,
        default_state: 18936,
        props: &[PropInfo {
            name: "facing",
            values: &["north", "south", "west", "east"],
        }],
    },
    BlockInfo {
        name: "bell",
        first_state: 18940,
        state_count: 32,
        default_state: 18941,
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
        first_state: 18972,
        state_count: 4,
        default_state: 18975,
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
        first_state: 18976,
        state_count: 4,
        default_state: 18979,
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
        first_state: 18980,
        state_count: 32,
        default_state: 18983,
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
        first_state: 19012,
        state_count: 32,
        default_state: 19015,
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
        first_state: 19044,
        state_count: 4,
        default_state: 19044,
        props: &[PropInfo {
            name: "age",
            values: &["0", "1", "2", "3"],
        }],
    },
    BlockInfo {
        name: "warped_stem",
        first_state: 19048,
        state_count: 3,
        default_state: 19049,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_stem",
        first_state: 19051,
        state_count: 3,
        default_state: 19052,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_hyphae",
        first_state: 19054,
        state_count: 3,
        default_state: 19055,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_warped_hyphae",
        first_state: 19057,
        state_count: 3,
        default_state: 19058,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "warped_nylium",
        first_state: 19060,
        state_count: 1,
        default_state: 19060,
        props: &[],
    },
    BlockInfo {
        name: "warped_fungus",
        first_state: 19061,
        state_count: 1,
        default_state: 19061,
        props: &[],
    },
    BlockInfo {
        name: "warped_wart_block",
        first_state: 19062,
        state_count: 1,
        default_state: 19062,
        props: &[],
    },
    BlockInfo {
        name: "warped_roots",
        first_state: 19063,
        state_count: 1,
        default_state: 19063,
        props: &[],
    },
    BlockInfo {
        name: "nether_sprouts",
        first_state: 19064,
        state_count: 1,
        default_state: 19064,
        props: &[],
    },
    BlockInfo {
        name: "crimson_stem",
        first_state: 19065,
        state_count: 3,
        default_state: 19066,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_stem",
        first_state: 19068,
        state_count: 3,
        default_state: 19069,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_hyphae",
        first_state: 19071,
        state_count: 3,
        default_state: 19072,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "stripped_crimson_hyphae",
        first_state: 19074,
        state_count: 3,
        default_state: 19075,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "crimson_nylium",
        first_state: 19077,
        state_count: 1,
        default_state: 19077,
        props: &[],
    },
    BlockInfo {
        name: "crimson_fungus",
        first_state: 19078,
        state_count: 1,
        default_state: 19078,
        props: &[],
    },
    BlockInfo {
        name: "shroomlight",
        first_state: 19079,
        state_count: 1,
        default_state: 19079,
        props: &[],
    },
    BlockInfo {
        name: "weeping_vines",
        first_state: 19080,
        state_count: 26,
        default_state: 19080,
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
        first_state: 19106,
        state_count: 1,
        default_state: 19106,
        props: &[],
    },
    BlockInfo {
        name: "twisting_vines",
        first_state: 19107,
        state_count: 26,
        default_state: 19107,
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
        first_state: 19133,
        state_count: 1,
        default_state: 19133,
        props: &[],
    },
    BlockInfo {
        name: "crimson_roots",
        first_state: 19134,
        state_count: 1,
        default_state: 19134,
        props: &[],
    },
    BlockInfo {
        name: "crimson_planks",
        first_state: 19135,
        state_count: 1,
        default_state: 19135,
        props: &[],
    },
    BlockInfo {
        name: "warped_planks",
        first_state: 19136,
        state_count: 1,
        default_state: 19136,
        props: &[],
    },
    BlockInfo {
        name: "crimson_slab",
        first_state: 19137,
        state_count: 6,
        default_state: 19140,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 19143,
        state_count: 6,
        default_state: 19146,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 19149,
        state_count: 2,
        default_state: 19150,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "warped_pressure_plate",
        first_state: 19151,
        state_count: 2,
        default_state: 19152,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "crimson_fence",
        first_state: 19153,
        state_count: 32,
        default_state: 19184,
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
        first_state: 19185,
        state_count: 32,
        default_state: 19216,
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
        first_state: 19217,
        state_count: 64,
        default_state: 19232,
        props: &[
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
        first_state: 19281,
        state_count: 64,
        default_state: 19296,
        props: &[
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
        first_state: 19345,
        state_count: 32,
        default_state: 19352,
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
        first_state: 19377,
        state_count: 32,
        default_state: 19384,
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
        first_state: 19409,
        state_count: 80,
        default_state: 19420,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 19489,
        state_count: 80,
        default_state: 19500,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 19569,
        state_count: 24,
        default_state: 19578,
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
        first_state: 19593,
        state_count: 24,
        default_state: 19602,
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
        first_state: 19617,
        state_count: 64,
        default_state: 19628,
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
        first_state: 19681,
        state_count: 64,
        default_state: 19692,
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
        first_state: 19745,
        state_count: 32,
        default_state: 19746,
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
        first_state: 19777,
        state_count: 32,
        default_state: 19778,
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
        first_state: 19809,
        state_count: 8,
        default_state: 19810,
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
        first_state: 19817,
        state_count: 8,
        default_state: 19818,
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
        first_state: 19825,
        state_count: 4,
        default_state: 19826,
        props: &[PropInfo {
            name: "mode",
            values: &["save", "load", "corner", "data"],
        }],
    },
    BlockInfo {
        name: "jigsaw",
        first_state: 19829,
        state_count: 12,
        default_state: 19839,
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
        first_state: 19841,
        state_count: 9,
        default_state: 19841,
        props: &[PropInfo {
            name: "level",
            values: &["0", "1", "2", "3", "4", "5", "6", "7", "8"],
        }],
    },
    BlockInfo {
        name: "target",
        first_state: 19850,
        state_count: 16,
        default_state: 19850,
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
        first_state: 19866,
        state_count: 24,
        default_state: 19866,
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
        first_state: 19890,
        state_count: 24,
        default_state: 19890,
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
        first_state: 19914,
        state_count: 1,
        default_state: 19914,
        props: &[],
    },
    BlockInfo {
        name: "honeycomb_block",
        first_state: 19915,
        state_count: 1,
        default_state: 19915,
        props: &[],
    },
    BlockInfo {
        name: "netherite_block",
        first_state: 19916,
        state_count: 1,
        default_state: 19916,
        props: &[],
    },
    BlockInfo {
        name: "ancient_debris",
        first_state: 19917,
        state_count: 1,
        default_state: 19917,
        props: &[],
    },
    BlockInfo {
        name: "crying_obsidian",
        first_state: 19918,
        state_count: 1,
        default_state: 19918,
        props: &[],
    },
    BlockInfo {
        name: "respawn_anchor",
        first_state: 19919,
        state_count: 5,
        default_state: 19919,
        props: &[PropInfo {
            name: "charges",
            values: &["0", "1", "2", "3", "4"],
        }],
    },
    BlockInfo {
        name: "potted_crimson_fungus",
        first_state: 19924,
        state_count: 1,
        default_state: 19924,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_fungus",
        first_state: 19925,
        state_count: 1,
        default_state: 19925,
        props: &[],
    },
    BlockInfo {
        name: "potted_crimson_roots",
        first_state: 19926,
        state_count: 1,
        default_state: 19926,
        props: &[],
    },
    BlockInfo {
        name: "potted_warped_roots",
        first_state: 19927,
        state_count: 1,
        default_state: 19927,
        props: &[],
    },
    BlockInfo {
        name: "lodestone",
        first_state: 19928,
        state_count: 1,
        default_state: 19928,
        props: &[],
    },
    BlockInfo {
        name: "blackstone",
        first_state: 19929,
        state_count: 1,
        default_state: 19929,
        props: &[],
    },
    BlockInfo {
        name: "blackstone_stairs",
        first_state: 19930,
        state_count: 80,
        default_state: 19941,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20010,
        state_count: 324,
        default_state: 20013,
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
        first_state: 20334,
        state_count: 6,
        default_state: 20337,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 20340,
        state_count: 1,
        default_state: 20340,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_bricks",
        first_state: 20341,
        state_count: 1,
        default_state: 20341,
        props: &[],
    },
    BlockInfo {
        name: "cracked_polished_blackstone_bricks",
        first_state: 20342,
        state_count: 1,
        default_state: 20342,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_polished_blackstone",
        first_state: 20343,
        state_count: 1,
        default_state: 20343,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_brick_slab",
        first_state: 20344,
        state_count: 6,
        default_state: 20347,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 20350,
        state_count: 80,
        default_state: 20361,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20430,
        state_count: 324,
        default_state: 20433,
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
        first_state: 20754,
        state_count: 1,
        default_state: 20754,
        props: &[],
    },
    BlockInfo {
        name: "polished_blackstone_stairs",
        first_state: 20755,
        state_count: 80,
        default_state: 20766,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 20835,
        state_count: 6,
        default_state: 20838,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 20841,
        state_count: 2,
        default_state: 20842,
        props: &[PropInfo {
            name: "powered",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "polished_blackstone_button",
        first_state: 20843,
        state_count: 24,
        default_state: 20852,
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
        first_state: 20867,
        state_count: 324,
        default_state: 20870,
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
        first_state: 21191,
        state_count: 1,
        default_state: 21191,
        props: &[],
    },
    BlockInfo {
        name: "cracked_nether_bricks",
        first_state: 21192,
        state_count: 1,
        default_state: 21192,
        props: &[],
    },
    BlockInfo {
        name: "quartz_bricks",
        first_state: 21193,
        state_count: 1,
        default_state: 21193,
        props: &[],
    },
    BlockInfo {
        name: "candle",
        first_state: 21194,
        state_count: 16,
        default_state: 21197,
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
        first_state: 21210,
        state_count: 16,
        default_state: 21213,
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
        first_state: 21226,
        state_count: 16,
        default_state: 21229,
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
        first_state: 21242,
        state_count: 16,
        default_state: 21245,
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
        first_state: 21258,
        state_count: 16,
        default_state: 21261,
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
        first_state: 21274,
        state_count: 16,
        default_state: 21277,
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
        first_state: 21290,
        state_count: 16,
        default_state: 21293,
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
        first_state: 21306,
        state_count: 16,
        default_state: 21309,
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
        first_state: 21322,
        state_count: 16,
        default_state: 21325,
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
        first_state: 21338,
        state_count: 16,
        default_state: 21341,
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
        first_state: 21354,
        state_count: 16,
        default_state: 21357,
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
        first_state: 21370,
        state_count: 16,
        default_state: 21373,
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
        first_state: 21386,
        state_count: 16,
        default_state: 21389,
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
        first_state: 21402,
        state_count: 16,
        default_state: 21405,
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
        first_state: 21418,
        state_count: 16,
        default_state: 21421,
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
        first_state: 21434,
        state_count: 16,
        default_state: 21437,
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
        first_state: 21450,
        state_count: 16,
        default_state: 21453,
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
        first_state: 21466,
        state_count: 2,
        default_state: 21467,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "white_candle_cake",
        first_state: 21468,
        state_count: 2,
        default_state: 21469,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "orange_candle_cake",
        first_state: 21470,
        state_count: 2,
        default_state: 21471,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "magenta_candle_cake",
        first_state: 21472,
        state_count: 2,
        default_state: 21473,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_blue_candle_cake",
        first_state: 21474,
        state_count: 2,
        default_state: 21475,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "yellow_candle_cake",
        first_state: 21476,
        state_count: 2,
        default_state: 21477,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "lime_candle_cake",
        first_state: 21478,
        state_count: 2,
        default_state: 21479,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pink_candle_cake",
        first_state: 21480,
        state_count: 2,
        default_state: 21481,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "gray_candle_cake",
        first_state: 21482,
        state_count: 2,
        default_state: 21483,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "light_gray_candle_cake",
        first_state: 21484,
        state_count: 2,
        default_state: 21485,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "cyan_candle_cake",
        first_state: 21486,
        state_count: 2,
        default_state: 21487,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "purple_candle_cake",
        first_state: 21488,
        state_count: 2,
        default_state: 21489,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "blue_candle_cake",
        first_state: 21490,
        state_count: 2,
        default_state: 21491,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "brown_candle_cake",
        first_state: 21492,
        state_count: 2,
        default_state: 21493,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "green_candle_cake",
        first_state: 21494,
        state_count: 2,
        default_state: 21495,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "red_candle_cake",
        first_state: 21496,
        state_count: 2,
        default_state: 21497,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "black_candle_cake",
        first_state: 21498,
        state_count: 2,
        default_state: 21499,
        props: &[PropInfo {
            name: "lit",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "amethyst_block",
        first_state: 21500,
        state_count: 1,
        default_state: 21500,
        props: &[],
    },
    BlockInfo {
        name: "budding_amethyst",
        first_state: 21501,
        state_count: 1,
        default_state: 21501,
        props: &[],
    },
    BlockInfo {
        name: "amethyst_cluster",
        first_state: 21502,
        state_count: 12,
        default_state: 21511,
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
        first_state: 21514,
        state_count: 12,
        default_state: 21523,
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
        first_state: 21526,
        state_count: 12,
        default_state: 21535,
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
        first_state: 21538,
        state_count: 12,
        default_state: 21547,
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
        first_state: 21550,
        state_count: 1,
        default_state: 21550,
        props: &[],
    },
    BlockInfo {
        name: "tuff_slab",
        first_state: 21551,
        state_count: 6,
        default_state: 21554,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 21557,
        state_count: 80,
        default_state: 21568,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 21637,
        state_count: 324,
        default_state: 21640,
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
        first_state: 21961,
        state_count: 1,
        default_state: 21961,
        props: &[],
    },
    BlockInfo {
        name: "polished_tuff_slab",
        first_state: 21962,
        state_count: 6,
        default_state: 21965,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 21968,
        state_count: 80,
        default_state: 21979,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 22048,
        state_count: 324,
        default_state: 22051,
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
        first_state: 22372,
        state_count: 1,
        default_state: 22372,
        props: &[],
    },
    BlockInfo {
        name: "tuff_bricks",
        first_state: 22373,
        state_count: 1,
        default_state: 22373,
        props: &[],
    },
    BlockInfo {
        name: "tuff_brick_slab",
        first_state: 22374,
        state_count: 6,
        default_state: 22377,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 22380,
        state_count: 80,
        default_state: 22391,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 22460,
        state_count: 324,
        default_state: 22463,
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
        first_state: 22784,
        state_count: 1,
        default_state: 22784,
        props: &[],
    },
    BlockInfo {
        name: "calcite",
        first_state: 22785,
        state_count: 1,
        default_state: 22785,
        props: &[],
    },
    BlockInfo {
        name: "tinted_glass",
        first_state: 22786,
        state_count: 1,
        default_state: 22786,
        props: &[],
    },
    BlockInfo {
        name: "powder_snow",
        first_state: 22787,
        state_count: 1,
        default_state: 22787,
        props: &[],
    },
    BlockInfo {
        name: "sculk_sensor",
        first_state: 22788,
        state_count: 96,
        default_state: 22789,
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
        first_state: 22884,
        state_count: 384,
        default_state: 22885,
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
        first_state: 23268,
        state_count: 1,
        default_state: 23268,
        props: &[],
    },
    BlockInfo {
        name: "sculk_vein",
        first_state: 23269,
        state_count: 128,
        default_state: 23396,
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
        first_state: 23397,
        state_count: 2,
        default_state: 23398,
        props: &[PropInfo {
            name: "bloom",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "sculk_shrieker",
        first_state: 23399,
        state_count: 8,
        default_state: 23406,
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
        first_state: 23407,
        state_count: 1,
        default_state: 23407,
        props: &[],
    },
    BlockInfo {
        name: "exposed_copper",
        first_state: 23408,
        state_count: 1,
        default_state: 23408,
        props: &[],
    },
    BlockInfo {
        name: "weathered_copper",
        first_state: 23409,
        state_count: 1,
        default_state: 23409,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_copper",
        first_state: 23410,
        state_count: 1,
        default_state: 23410,
        props: &[],
    },
    BlockInfo {
        name: "copper_ore",
        first_state: 23411,
        state_count: 1,
        default_state: 23411,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_copper_ore",
        first_state: 23412,
        state_count: 1,
        default_state: 23412,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper",
        first_state: 23413,
        state_count: 1,
        default_state: 23413,
        props: &[],
    },
    BlockInfo {
        name: "weathered_cut_copper",
        first_state: 23414,
        state_count: 1,
        default_state: 23414,
        props: &[],
    },
    BlockInfo {
        name: "exposed_cut_copper",
        first_state: 23415,
        state_count: 1,
        default_state: 23415,
        props: &[],
    },
    BlockInfo {
        name: "cut_copper",
        first_state: 23416,
        state_count: 1,
        default_state: 23416,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_chiseled_copper",
        first_state: 23417,
        state_count: 1,
        default_state: 23417,
        props: &[],
    },
    BlockInfo {
        name: "weathered_chiseled_copper",
        first_state: 23418,
        state_count: 1,
        default_state: 23418,
        props: &[],
    },
    BlockInfo {
        name: "exposed_chiseled_copper",
        first_state: 23419,
        state_count: 1,
        default_state: 23419,
        props: &[],
    },
    BlockInfo {
        name: "chiseled_copper",
        first_state: 23420,
        state_count: 1,
        default_state: 23420,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_chiseled_copper",
        first_state: 23421,
        state_count: 1,
        default_state: 23421,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_chiseled_copper",
        first_state: 23422,
        state_count: 1,
        default_state: 23422,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_chiseled_copper",
        first_state: 23423,
        state_count: 1,
        default_state: 23423,
        props: &[],
    },
    BlockInfo {
        name: "waxed_chiseled_copper",
        first_state: 23424,
        state_count: 1,
        default_state: 23424,
        props: &[],
    },
    BlockInfo {
        name: "oxidized_cut_copper_stairs",
        first_state: 23425,
        state_count: 80,
        default_state: 23436,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23505,
        state_count: 80,
        default_state: 23516,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23585,
        state_count: 80,
        default_state: 23596,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23665,
        state_count: 80,
        default_state: 23676,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23745,
        state_count: 6,
        default_state: 23748,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 23751,
        state_count: 6,
        default_state: 23754,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 23757,
        state_count: 6,
        default_state: 23760,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 23763,
        state_count: 6,
        default_state: 23766,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 23769,
        state_count: 1,
        default_state: 23769,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_copper",
        first_state: 23770,
        state_count: 1,
        default_state: 23770,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_copper",
        first_state: 23771,
        state_count: 1,
        default_state: 23771,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_copper",
        first_state: 23772,
        state_count: 1,
        default_state: 23772,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper",
        first_state: 23773,
        state_count: 1,
        default_state: 23773,
        props: &[],
    },
    BlockInfo {
        name: "waxed_weathered_cut_copper",
        first_state: 23774,
        state_count: 1,
        default_state: 23774,
        props: &[],
    },
    BlockInfo {
        name: "waxed_exposed_cut_copper",
        first_state: 23775,
        state_count: 1,
        default_state: 23775,
        props: &[],
    },
    BlockInfo {
        name: "waxed_cut_copper",
        first_state: 23776,
        state_count: 1,
        default_state: 23776,
        props: &[],
    },
    BlockInfo {
        name: "waxed_oxidized_cut_copper_stairs",
        first_state: 23777,
        state_count: 80,
        default_state: 23788,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23857,
        state_count: 80,
        default_state: 23868,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 23937,
        state_count: 80,
        default_state: 23948,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24017,
        state_count: 80,
        default_state: 24028,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 24097,
        state_count: 6,
        default_state: 24100,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24103,
        state_count: 6,
        default_state: 24106,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24109,
        state_count: 6,
        default_state: 24112,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24115,
        state_count: 6,
        default_state: 24118,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 24121,
        state_count: 64,
        default_state: 24132,
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
        first_state: 24185,
        state_count: 64,
        default_state: 24196,
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
        first_state: 24249,
        state_count: 64,
        default_state: 24260,
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
        first_state: 24313,
        state_count: 64,
        default_state: 24324,
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
        first_state: 24377,
        state_count: 64,
        default_state: 24388,
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
        first_state: 24441,
        state_count: 64,
        default_state: 24452,
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
        first_state: 24505,
        state_count: 64,
        default_state: 24516,
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
        first_state: 24569,
        state_count: 64,
        default_state: 24580,
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
        first_state: 24633,
        state_count: 64,
        default_state: 24648,
        props: &[
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
        first_state: 24697,
        state_count: 64,
        default_state: 24712,
        props: &[
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
        first_state: 24761,
        state_count: 64,
        default_state: 24776,
        props: &[
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
        first_state: 24825,
        state_count: 64,
        default_state: 24840,
        props: &[
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
        first_state: 24889,
        state_count: 64,
        default_state: 24904,
        props: &[
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
        first_state: 24953,
        state_count: 64,
        default_state: 24968,
        props: &[
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
        first_state: 25017,
        state_count: 64,
        default_state: 25032,
        props: &[
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
        first_state: 25081,
        state_count: 64,
        default_state: 25096,
        props: &[
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
        first_state: 25145,
        state_count: 2,
        default_state: 25146,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "exposed_copper_grate",
        first_state: 25147,
        state_count: 2,
        default_state: 25148,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "weathered_copper_grate",
        first_state: 25149,
        state_count: 2,
        default_state: 25150,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "oxidized_copper_grate",
        first_state: 25151,
        state_count: 2,
        default_state: 25152,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_copper_grate",
        first_state: 25153,
        state_count: 2,
        default_state: 25154,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_exposed_copper_grate",
        first_state: 25155,
        state_count: 2,
        default_state: 25156,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_weathered_copper_grate",
        first_state: 25157,
        state_count: 2,
        default_state: 25158,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "waxed_oxidized_copper_grate",
        first_state: 25159,
        state_count: 2,
        default_state: 25160,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "copper_bulb",
        first_state: 25161,
        state_count: 4,
        default_state: 25164,
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
        first_state: 25165,
        state_count: 4,
        default_state: 25168,
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
        first_state: 25169,
        state_count: 4,
        default_state: 25172,
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
        first_state: 25173,
        state_count: 4,
        default_state: 25176,
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
        first_state: 25177,
        state_count: 4,
        default_state: 25180,
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
        first_state: 25181,
        state_count: 4,
        default_state: 25184,
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
        first_state: 25185,
        state_count: 4,
        default_state: 25188,
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
        first_state: 25189,
        state_count: 4,
        default_state: 25192,
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
        first_state: 25193,
        state_count: 24,
        default_state: 25212,
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
        first_state: 25217,
        state_count: 20,
        default_state: 25222,
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
        first_state: 25237,
        state_count: 1,
        default_state: 25237,
        props: &[],
    },
    BlockInfo {
        name: "cave_vines",
        first_state: 25238,
        state_count: 52,
        default_state: 25239,
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
        first_state: 25290,
        state_count: 2,
        default_state: 25291,
        props: &[PropInfo {
            name: "berries",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "spore_blossom",
        first_state: 25292,
        state_count: 1,
        default_state: 25292,
        props: &[],
    },
    BlockInfo {
        name: "azalea",
        first_state: 25293,
        state_count: 1,
        default_state: 25293,
        props: &[],
    },
    BlockInfo {
        name: "flowering_azalea",
        first_state: 25294,
        state_count: 1,
        default_state: 25294,
        props: &[],
    },
    BlockInfo {
        name: "moss_carpet",
        first_state: 25295,
        state_count: 1,
        default_state: 25295,
        props: &[],
    },
    BlockInfo {
        name: "pink_petals",
        first_state: 25296,
        state_count: 16,
        default_state: 25296,
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
        first_state: 25312,
        state_count: 1,
        default_state: 25312,
        props: &[],
    },
    BlockInfo {
        name: "big_dripleaf",
        first_state: 25313,
        state_count: 32,
        default_state: 25314,
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
        first_state: 25345,
        state_count: 8,
        default_state: 25346,
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
        first_state: 25353,
        state_count: 16,
        default_state: 25356,
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
        first_state: 25369,
        state_count: 2,
        default_state: 25370,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "rooted_dirt",
        first_state: 25371,
        state_count: 1,
        default_state: 25371,
        props: &[],
    },
    BlockInfo {
        name: "mud",
        first_state: 25372,
        state_count: 1,
        default_state: 25372,
        props: &[],
    },
    BlockInfo {
        name: "deepslate",
        first_state: 25373,
        state_count: 3,
        default_state: 25374,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "cobbled_deepslate",
        first_state: 25376,
        state_count: 1,
        default_state: 25376,
        props: &[],
    },
    BlockInfo {
        name: "cobbled_deepslate_stairs",
        first_state: 25377,
        state_count: 80,
        default_state: 25388,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 25457,
        state_count: 6,
        default_state: 25460,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 25463,
        state_count: 324,
        default_state: 25466,
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
        first_state: 25787,
        state_count: 1,
        default_state: 25787,
        props: &[],
    },
    BlockInfo {
        name: "polished_deepslate_stairs",
        first_state: 25788,
        state_count: 80,
        default_state: 25799,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 25868,
        state_count: 6,
        default_state: 25871,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 25874,
        state_count: 324,
        default_state: 25877,
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
        first_state: 26198,
        state_count: 1,
        default_state: 26198,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_tile_stairs",
        first_state: 26199,
        state_count: 80,
        default_state: 26210,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 26279,
        state_count: 6,
        default_state: 26282,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 26285,
        state_count: 324,
        default_state: 26288,
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
        first_state: 26609,
        state_count: 1,
        default_state: 26609,
        props: &[],
    },
    BlockInfo {
        name: "deepslate_brick_stairs",
        first_state: 26610,
        state_count: 80,
        default_state: 26621,
        props: &[
            PropInfo {
                name: "facing",
                values: &["north", "south", "west", "east"],
            },
            PropInfo {
                name: "half",
                values: &["top", "bottom"],
            },
            PropInfo {
                name: "shape",
                values: &[
                    "straight",
                    "inner_left",
                    "inner_right",
                    "outer_left",
                    "outer_right",
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
        first_state: 26690,
        state_count: 6,
        default_state: 26693,
        props: &[
            PropInfo {
                name: "type",
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
        first_state: 26696,
        state_count: 324,
        default_state: 26699,
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
        first_state: 27020,
        state_count: 1,
        default_state: 27020,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_bricks",
        first_state: 27021,
        state_count: 1,
        default_state: 27021,
        props: &[],
    },
    BlockInfo {
        name: "cracked_deepslate_tiles",
        first_state: 27022,
        state_count: 1,
        default_state: 27022,
        props: &[],
    },
    BlockInfo {
        name: "infested_deepslate",
        first_state: 27023,
        state_count: 3,
        default_state: 27024,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "smooth_basalt",
        first_state: 27026,
        state_count: 1,
        default_state: 27026,
        props: &[],
    },
    BlockInfo {
        name: "raw_iron_block",
        first_state: 27027,
        state_count: 1,
        default_state: 27027,
        props: &[],
    },
    BlockInfo {
        name: "raw_copper_block",
        first_state: 27028,
        state_count: 1,
        default_state: 27028,
        props: &[],
    },
    BlockInfo {
        name: "raw_gold_block",
        first_state: 27029,
        state_count: 1,
        default_state: 27029,
        props: &[],
    },
    BlockInfo {
        name: "potted_azalea_bush",
        first_state: 27030,
        state_count: 1,
        default_state: 27030,
        props: &[],
    },
    BlockInfo {
        name: "potted_flowering_azalea_bush",
        first_state: 27031,
        state_count: 1,
        default_state: 27031,
        props: &[],
    },
    BlockInfo {
        name: "ochre_froglight",
        first_state: 27032,
        state_count: 3,
        default_state: 27033,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "verdant_froglight",
        first_state: 27035,
        state_count: 3,
        default_state: 27036,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "pearlescent_froglight",
        first_state: 27038,
        state_count: 3,
        default_state: 27039,
        props: &[PropInfo {
            name: "axis",
            values: &["x", "y", "z"],
        }],
    },
    BlockInfo {
        name: "frogspawn",
        first_state: 27041,
        state_count: 1,
        default_state: 27041,
        props: &[],
    },
    BlockInfo {
        name: "reinforced_deepslate",
        first_state: 27042,
        state_count: 1,
        default_state: 27042,
        props: &[],
    },
    BlockInfo {
        name: "decorated_pot",
        first_state: 27043,
        state_count: 16,
        default_state: 27052,
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
        first_state: 27059,
        state_count: 48,
        default_state: 27104,
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
        first_state: 27107,
        state_count: 12,
        default_state: 27113,
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
        first_state: 27119,
        state_count: 32,
        default_state: 27123,
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
        first_state: 27151,
        state_count: 2,
        default_state: 27152,
        props: &[PropInfo {
            name: "waterlogged",
            values: &["true", "false"],
        }],
    },
    BlockInfo {
        name: "pale_moss_block",
        first_state: 27153,
        state_count: 1,
        default_state: 27153,
        props: &[],
    },
    BlockInfo {
        name: "pale_moss_carpet",
        first_state: 27154,
        state_count: 162,
        default_state: 27154,
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
        first_state: 27316,
        state_count: 2,
        default_state: 27316,
        props: &[PropInfo {
            name: "tip",
            values: &["true", "false"],
        }],
    },
];
