use m2s2_math::Vector2i32;
use rand::Rng;

use crate::grid::{self, Grid, GRID_HEIGHT, GRID_WIDTH};
use crate::piece::{BlockType, Piece};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    grid: Grid,
    current_piece: Option<Piece>,
    next_piece: Piece,
    score: u32,
    level: u32,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let next_piece = Game::generate_piece();
        Game {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT),
            current_piece: None,
            next_piece,
            score: 0,
            level: 1,
            state: GameState::Playing,
        }
    }

    pub fn spawn_piece(&mut self) {
        let mut new_piece = self.next_piece;
        let initial_spawn_position_x = (self.grid.width() / 2) as i32;
        let initial_spawn_position_y = 0;

        new_piece.pos = Vector2i32::new(initial_spawn_position_x, initial_spawn_position_y);
        new_piece.rot_state = 0;

        self.current_piece = Some(new_piece);
        self.next_piece = Game::generate_piece();
    }

    fn generate_piece() -> Piece {
        let mut rng = rand::thread_rng();
        let piece_type =
            BlockType::from_usize(rng.gen_range(0..7)).expect("Invalid piece type index");

        let initial_piece_position = Vector2i32::new(GRID_WIDTH as i32 + 2, 2);
        Piece::new(piece_type, initial_piece_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new() {
        let game = Game::new();
        assert_eq!(game.grid.width(), grid::GRID_WIDTH);
        assert_eq!(game.grid.height(), grid::GRID_HEIGHT);
        assert_eq!(game.score, 0);
        assert_eq!(game.level, 1);
        assert_eq!(game.state, GameState::Playing);
        assert!(game.current_piece.is_none());
    }

    #[test]
    fn test_game_spawn_piece() {
        let mut game = Game::new();
        assert!(game.current_piece.is_none());
        game.spawn_piece();
        assert!(game.current_piece.is_some());
        let current_piece = game.current_piece.unwrap();
        assert_eq!(current_piece.pos[0], (grid::GRID_WIDTH / 2) as i32);
        assert_eq!(current_piece.pos[1], 0);
        assert_eq!(current_piece.rot_state, 0);
        assert!(game.next_piece.block_type != BlockType::Empty);
    }
}
