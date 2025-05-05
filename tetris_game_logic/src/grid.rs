use m2s2_math::Vector2i32;

use crate::piece::BlockType;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub block_type: BlockType,
}

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells = vec![
            Cell {
                block_type: BlockType::Empty
            };
            size
        ];
        Grid {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn xy_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.xy_to_index(x, y)
            .and_then(|index| self.cells.get(index))
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.xy_to_index(x, y)
            .and_then(|index| self.cells.get_mut(index))
    }

    pub fn is_cell_occupied(&self, x: i32, y: i32) -> bool {
        if !self.is_within_bounds(x, y) {
            return true;
        }

        let index = self
            .xy_to_index(x as usize, y as usize)
            .expect("Index calculation failed abter bounds check");
        self.cells[index].block_type != BlockType::Empty
    }

    pub fn is_within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn is_coord_within_bounds(&self, coord: Vector2i32) -> bool {
        self.is_within_bounds(coord[0], coord[1])
    }

    pub fn is_coord_occupied(&self, coord: Vector2i32) -> bool {
        self.is_cell_occupied(coord[0], coord[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
        assert_eq!(grid.width(), GRID_WIDTH);
        assert_eq!(grid.height(), GRID_HEIGHT);
        assert_eq!(grid.cells.len(), GRID_WIDTH * GRID_HEIGHT);
        for cell in grid.cells.iter() {
            assert_eq!(cell.block_type, BlockType::Empty);
        }
    }

    #[test]
    fn test_grid_xy_to_index() {
        let grid = Grid::new(10, 20);
        assert_eq!(grid.xy_to_index(0, 0), Some(0));
        assert_eq!(grid.xy_to_index(9, 0), Some(9));
        assert_eq!(grid.xy_to_index(0, 1), Some(10));
        assert_eq!(grid.xy_to_index(9, 19), Some(10 * 19 + 9));
        assert_eq!(grid.xy_to_index(10, 0), None);
        assert_eq!(grid.xy_to_index(0, 20), None);
    }

    #[test]
    fn test_grid_get_cell() {
        let mut grid = Grid::new(5, 5);
        assert_eq!(
            grid.get_cell(0, 0).map(|c| c.block_type),
            Some(BlockType::Empty)
        );
        assert_eq!(
            grid.get_cell(4, 4).map(|c| c.block_type),
            Some(BlockType::Empty)
        );
        assert_eq!(grid.get_cell(5, 0), None);

        if let Some(cell) = grid.get_cell_mut(2, 2) {
            cell.block_type = BlockType::I;
        }

        assert_eq!(
            grid.get_cell(2, 2).map(|c| c.block_type),
            Some(BlockType::I)
        );
    }

    #[test]
    fn test_grid_is_cell_occupied_within_bounds() {
        let mut grid = Grid::new(5, 5);
        assert!(!grid.is_cell_occupied(0, 0));

        if let Some(cell) = grid.get_cell_mut(2, 2) {
            cell.block_type = BlockType::L;
        }

        assert!(grid.is_within_bounds(2, 2));
    }

    #[test]
    fn test_grid_is_cell_occupied_out_of_bounds() {
        let grid = Grid::new(5, 5);
        assert!(grid.is_cell_occupied(-1, 0));
        assert!(grid.is_cell_occupied(5, 0));
    }

    #[test]
    fn test_grid_is_within_bounds() {
        let grid = Grid::new(10, 20);
        assert!(grid.is_within_bounds(0, 0));
        assert!(grid.is_within_bounds(9, 19));
        assert!(grid.is_within_bounds(5, 10));

        assert!(!grid.is_within_bounds(-1, 0));
        assert!(!grid.is_within_bounds(10, 0));
        assert!(!grid.is_within_bounds(0, -1));
        assert!(!grid.is_within_bounds(0, 20));
        assert!(!grid.is_within_bounds(10, 20));
    }
}
