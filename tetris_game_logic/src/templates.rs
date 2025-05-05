use crate::piece::{BlockType, PieceShapeTemplate};
use m2s2_math::Vector2i32;

const O_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
    ],
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
    ],
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
    ],
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
    ],
];

const I_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(3, 1),
    ],
    [
        Vector2i32::new(2, 0),
        Vector2i32::new(2, 1),
        Vector2i32::new(2, 2),
        Vector2i32::new(2, 3),
    ],
    [
        Vector2i32::new(0, 2),
        Vector2i32::new(1, 2),
        Vector2i32::new(2, 2),
        Vector2i32::new(3, 2),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
        Vector2i32::new(1, 3),
    ],
];

const J_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(2, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
    ],
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(2, 2),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(0, 2),
        Vector2i32::new(1, 2),
    ],
];

const L_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(2, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
        Vector2i32::new(2, 2),
    ],
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(0, 2),
    ],
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
    ],
];

const S_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(2, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
    ],
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
    ],
    [
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(0, 2),
        Vector2i32::new(1, 2),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(0, 2),
    ],
];

const T_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(1, 0),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
        Vector2i32::new(2, 1),
    ],
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(1, 2),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
        Vector2i32::new(0, 1),
    ],
];

const Z_SHAPE_ROTATIONS: [[Vector2i32; 4]; 4] = [
    [
        Vector2i32::new(0, 0),
        Vector2i32::new(1, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
    ],
    [
        Vector2i32::new(2, 0),
        Vector2i32::new(1, 1),
        Vector2i32::new(2, 1),
        Vector2i32::new(1, 2),
    ],
    [
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(1, 2),
        Vector2i32::new(2, 2),
    ],
    [
        Vector2i32::new(1, 0),
        Vector2i32::new(0, 1),
        Vector2i32::new(1, 1),
        Vector2i32::new(0, 2),
    ],
];

const PIVOT_ORIGIN: Vector2i32 = Vector2i32::new(0, 0);
const PIVOT_1X1: Vector2i32 = Vector2i32::new(1, 1);

pub const SHAPE_TEMPLATES: [PieceShapeTemplate; 7] = [
    PieceShapeTemplate::new(BlockType::I, I_SHAPE_ROTATIONS, PIVOT_1X1),
    PieceShapeTemplate::new(BlockType::J, J_SHAPE_ROTATIONS, PIVOT_1X1),
    PieceShapeTemplate::new(BlockType::L, L_SHAPE_ROTATIONS, PIVOT_1X1),
    PieceShapeTemplate::new(BlockType::O, O_SHAPE_ROTATIONS, PIVOT_ORIGIN),
    PieceShapeTemplate::new(BlockType::S, S_SHAPE_ROTATIONS, PIVOT_1X1),
    PieceShapeTemplate::new(BlockType::T, T_SHAPE_ROTATIONS, PIVOT_1X1),
    PieceShapeTemplate::new(BlockType::Z, Z_SHAPE_ROTATIONS, PIVOT_1X1),
];
