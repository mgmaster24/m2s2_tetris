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

    pub fn get_cell_data(&self) -> Vec<i32> {
        let mut cell_data = Vec::new();
        for cell in &self.cells {
            cell_data.push(cell.block_type.into_i32());
        }

        cell_data
    }

    #[inline]
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

    pub fn clear_lines(&mut self) -> u32 {
        // Find lines to clear
        let mut lines_to_clear = Vec::new();
        for y in (0..self.height).rev() {
            let mut row_is_full = true;
            for x in 0..self.width {
                if self
                    .get_cell(x, y)
                    .is_none_or(|cell| cell.block_type == BlockType::Empty)
                {
                    row_is_full = false;
                    break;
                }
            }

            if row_is_full {
                lines_to_clear.push(y);
            }
        }

        let num_lines_cleared = lines_to_clear.len() as u32;
        if num_lines_cleared > 0 {
            let mut new_cells: Vec<Cell> = vec![
                Cell {
                    block_type: BlockType::Empty
                };
                self.cells.len()
            ];
            let mut y_write = self.height - 1;
            for y_read in (0..self.height).rev() {
                if !lines_to_clear.contains(&y_read) {
                    for x in 0..self.width {
                        let old_idx = self.xy_to_index(x, y_read).unwrap();
                        let new_idx = self.xy_to_index(x, y_write).unwrap();
                        new_cells[new_idx] = self.cells[old_idx];
                    }
                    y_write -= 1
                }
            }

            self.cells = new_cells;
        }

        num_lines_cleared
    }

    fn empty_cell(&mut self, idx: usize) {
        self.cells[idx] = Cell {
            block_type: BlockType::Empty,
        };
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
