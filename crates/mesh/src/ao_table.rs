//! @generated из ModelBlockRenderer.AdjacencyInfo (1.21.11); не править вручную.
//! Размеры: SizeInfo индексы 0..12 = DOWN,UP,NORTH,SOUTH,WEST,EAST,FLIP_*.
use ezzmc_assets::model::Direction::{self, *};

pub struct Adjacency {
    pub corners: [Direction; 4],
    pub non_cubic_weight: bool,
    pub vert_weights: [[u8; 8]; 4],
}

pub static ADJACENCY: [Adjacency; 6] = [
    Adjacency {
        corners: [West, East, North, South],
        non_cubic_weight: true,
        vert_weights: [
            [10, 3, 10, 9, 4, 9, 4, 3],
            [10, 2, 10, 8, 4, 8, 4, 2],
            [11, 2, 11, 8, 5, 8, 5, 2],
            [11, 3, 11, 9, 5, 9, 5, 3],
        ],
    },
    Adjacency {
        corners: [East, West, North, South],
        non_cubic_weight: true,
        vert_weights: [
            [5, 3, 5, 9, 11, 9, 11, 3],
            [5, 2, 5, 8, 11, 8, 11, 2],
            [4, 2, 4, 8, 10, 8, 10, 2],
            [4, 3, 4, 9, 10, 9, 10, 3],
        ],
    },
    Adjacency {
        corners: [Up, Down, East, West],
        non_cubic_weight: true,
        vert_weights: [
            [1, 10, 1, 4, 7, 4, 7, 10],
            [1, 11, 1, 5, 7, 5, 7, 11],
            [0, 11, 0, 5, 6, 5, 6, 11],
            [0, 10, 0, 4, 6, 4, 6, 10],
        ],
    },
    Adjacency {
        corners: [West, East, Down, Up],
        non_cubic_weight: true,
        vert_weights: [
            [1, 10, 7, 10, 7, 4, 1, 4],
            [0, 10, 6, 10, 6, 4, 0, 4],
            [0, 11, 6, 11, 6, 5, 0, 5],
            [1, 11, 7, 11, 7, 5, 1, 5],
        ],
    },
    Adjacency {
        corners: [Up, Down, North, South],
        non_cubic_weight: true,
        vert_weights: [
            [1, 3, 1, 9, 7, 9, 7, 3],
            [1, 2, 1, 8, 7, 8, 7, 2],
            [0, 2, 0, 8, 6, 8, 6, 2],
            [0, 3, 0, 9, 6, 9, 6, 3],
        ],
    },
    Adjacency {
        corners: [Down, Up, North, South],
        non_cubic_weight: true,
        vert_weights: [
            [6, 3, 6, 9, 0, 9, 0, 3],
            [6, 2, 6, 8, 0, 8, 0, 2],
            [7, 2, 7, 8, 1, 8, 1, 2],
            [7, 3, 7, 9, 1, 9, 1, 3],
        ],
    },
];
