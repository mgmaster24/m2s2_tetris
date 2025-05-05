use crate::templates;
use m2s2_math::Vector2i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    I, // Cyan
    J, // Blue
    L, // Orange
    O, // Yellow
    S, // Green
    T, // Purple
    Z, // Red
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceShapeTemplate {
    block_type: BlockType,
    rotations: [[Vector2i32; 4]; 4],
    pivot_offset: Vector2i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub block_type: BlockType,
    pub pos: Vector2i32,
    pub rot_state: usize,
    template: &'static PieceShapeTemplate,
}

impl Piece {
    pub fn new(block_type: BlockType, position: Vector2i32) -> Self {
        let template = templates::SHAPE_TEMPLATES
            .iter()
            .find(|&template| template.block_type == block_type)
            .expect("Unknown Block Type");

        Piece {
            block_type,
            pos: position,
            rot_state: 0,
            template,
        }
    }

    pub fn absolute_block_positions(&self) -> [Vector2i32; 4] {
        self.absolute_block_positions_at(self.pos, self.rot_state)
    }

    pub fn absolute_block_positions_at(
        &self,
        position: Vector2i32,
        rotation_state: usize,
    ) -> [Vector2i32; 4] {
        let relative_positions = self.template.rotations[rotation_state];
        let mut absolute_positions = [Vector2i32::new(0, 0); 4];
        for i in 0..4 {
            absolute_positions[i] = position + relative_positions[i];
        }

        absolute_positions
    }

    pub fn absolut_pivot_position(&self) -> Vector2i32 {
        self.pos + self.template.pivot_offset
    }

    fn relative_block_positions_at_rotation(&self, rotation_state: usize) -> [Vector2i32; 4] {
        self.template.rotations[rotation_state]
    }
}

impl PieceShapeTemplate {
    pub const fn new(
        block_type: BlockType,
        rotations: [[Vector2i32; 4]; 4],
        pivot_offset: Vector2i32,
    ) -> Self {
        PieceShapeTemplate {
            block_type,
            rotations,
            pivot_offset,
        }
    }
}

impl BlockType {
    pub fn from_usize(index: usize) -> Option<Self> {
        match index {
            0 => Some(BlockType::I),
            1 => Some(BlockType::J),
            2 => Some(BlockType::L),
            3 => Some(BlockType::O),
            4 => Some(BlockType::S),
            5 => Some(BlockType::T),
            6 => Some(BlockType::Z),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_SQUARE_SHAPE: PieceShapeTemplate = PieceShapeTemplate {
        block_type: BlockType::O,
        pivot_offset: Vector2i32::new(0, 0),
        rotations: [
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
        ],
    };

    const MOCK_LINE_SHAPE: PieceShapeTemplate = PieceShapeTemplate {
        block_type: BlockType::I,
        pivot_offset: Vector2i32::new(0, 0),
        rotations: [
            [
                Vector2i32::new(0, 0),
                Vector2i32::new(1, 0),
                Vector2i32::new(2, 0),
                Vector2i32::new(3, 0),
            ],
            [
                Vector2i32::new(0, 0),
                Vector2i32::new(0, 1),
                Vector2i32::new(0, 2),
                Vector2i32::new(0, 3),
            ],
            [
                Vector2i32::new(0, 0),
                Vector2i32::new(1, 0),
                Vector2i32::new(2, 0),
                Vector2i32::new(3, 0),
            ],
            [
                Vector2i32::new(0, 0),
                Vector2i32::new(0, 1),
                Vector2i32::new(0, 2),
                Vector2i32::new(0, 3),
            ],
        ],
    };

    #[test]
    fn test_get_block_type_from_usize() {
        assert_eq!(BlockType::from_usize(0), Some(BlockType::I));
        assert_eq!(BlockType::from_usize(1), Some(BlockType::J));
        assert_eq!(BlockType::from_usize(2), Some(BlockType::L));
        assert_eq!(BlockType::from_usize(3), Some(BlockType::O));
        assert_eq!(BlockType::from_usize(4), Some(BlockType::S));
        assert_eq!(BlockType::from_usize(5), Some(BlockType::T));
        assert_eq!(BlockType::from_usize(6), Some(BlockType::Z));
        assert_eq!(BlockType::from_usize(7), None);
    }

    #[test]
    fn test_piece_absolute_block_position() {
        let piece_o = Piece {
            block_type: BlockType::O,
            pos: Vector2i32::new(5, 10),
            rot_state: 0,
            template: &MOCK_SQUARE_SHAPE,
        };

        let abs_pos_o = piece_o.absolute_block_positions();
        assert_eq!(abs_pos_o[0], Vector2i32::new(5, 10));
        assert_eq!(abs_pos_o[1], Vector2i32::new(6, 10));
        assert_eq!(abs_pos_o[2], Vector2i32::new(5, 11));
        assert_eq!(abs_pos_o[3], Vector2i32::new(6, 11));

        let mut piece_i = Piece {
            block_type: BlockType::I,
            pos: Vector2i32::new(0, 0),
            rot_state: 0,
            template: &MOCK_LINE_SHAPE,
        };

        let mut abs_pos_i = piece_i.absolute_block_positions();
        assert_eq!(abs_pos_i[0], Vector2i32::new(0, 0));
        assert_eq!(abs_pos_i[1], Vector2i32::new(1, 0));
        assert_eq!(abs_pos_i[2], Vector2i32::new(2, 0));
        assert_eq!(abs_pos_i[3], Vector2i32::new(3, 0));

        piece_i.pos = Vector2i32::new(5, 5);
        piece_i.rot_state = 1;

        abs_pos_i = piece_i.absolute_block_positions();
        assert_eq!(abs_pos_i[0], Vector2i32::new(5, 5));
        assert_eq!(abs_pos_i[1], Vector2i32::new(5, 6));
        assert_eq!(abs_pos_i[2], Vector2i32::new(5, 7));
        assert_eq!(abs_pos_i[3], Vector2i32::new(5, 8));
    }
}
