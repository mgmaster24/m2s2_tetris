use m2s2_math::Vector2i32;
use rand::Rng;

use crate::grid::{Grid, GRID_HEIGHT, GRID_WIDTH};
use crate::piece::{BlockType, Piece, RotationDirection};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Pause,
    Resume,
    MoveRight,
    MoveLeft,
    MoveDown,
    HardDrop,
    RotateCW,
    RotateCCW,
}

#[derive(Debug, Clone)]
pub struct Game {
    grid: Grid,
    current_piece: Option<Piece>,
    next_piece: Piece,
    score: u32,
    level: u32,
    state: GameState,
    gravity_timer: f64,
    gravity_interval: f64,
    lines_cleared: u32,
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
            gravity_timer: 0.0,
            gravity_interval: 1.0,
            lines_cleared: 0,
        }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn grav_timer(&self) -> f64 {
        self.gravity_timer
    }

    pub fn grav_int(&self) -> f64 {
        self.gravity_interval
    }

    pub fn grid_width(&self) -> i32 {
        self.grid.width() as i32
    }

    pub fn grid_height(&self) -> i32 {
        self.grid.height() as i32
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn curr_piece(&self) -> Option<Piece> {
        self.current_piece
    }

    pub fn next_piece(&self) -> Piece {
        self.next_piece
    }

    pub fn get_grid_data(&self) -> Vec<i32> {
        self.grid.get_cell_data()
    }

    pub fn get_piece_abs_positions(&self, piece: &Piece) -> [Vector2i32; 4] {
        piece.absolute_block_positions()
    }

    pub fn spawn_piece(&mut self) -> bool {
        let mut new_piece = self.next_piece;
        let initial_spawn_position_x = (self.grid.width() / 2) as i32 - 1;
        let initial_spawn_position_y = 0;

        new_piece.pos = Vector2i32::new(initial_spawn_position_x, initial_spawn_position_y);
        new_piece.rot_state = 0;

        if self.check_piece_collision(&new_piece, new_piece.pos, new_piece.rot_state) {
            self.state = GameState::GameOver;
            return false;
        }

        self.current_piece = Some(new_piece);
        self.next_piece = Game::generate_piece();

        true
    }

    pub fn try_move(&mut self, delta: Vector2i32) -> bool {
        if self.state != GameState::Playing {
            return false;
        }

        if let Some(current_piece) = &self.current_piece {
            let potential_position = current_piece.pos + delta;
            if !self.check_piece_collision(
                current_piece,
                potential_position,
                current_piece.rot_state,
            ) {
                self.current_piece.as_mut().unwrap().pos = potential_position;
                return true;
            }
        }

        false
    }

    pub fn try_rotate(&mut self, direction: RotationDirection) -> bool {
        if self.state != GameState::Playing {
            return false;
        }

        if let Some(current_piece) = &self.current_piece {
            let next_rotation = current_piece.calculate_next_rotation_state(direction);
            if !self.check_piece_collision(current_piece, current_piece.pos, next_rotation) {
                self.current_piece.as_mut().unwrap().rot_state = next_rotation;
                return true;
            }
        }

        false
    }

    pub fn handle_input(&mut self, action: Action) {
        match action {
            Action::Pause => {
                if self.state == GameState::Playing {
                    self.state = GameState::Paused;
                }
            }
            Action::Resume => {
                if self.state == GameState::Playing {
                    return;
                }

                if self.state == GameState::Paused {
                    self.state = GameState::Playing;
                    return;
                }

                if self.state == GameState::GameOver {
                    self.reset();
                }
            }
            Action::MoveLeft => {
                self.try_move(Vector2i32::new(-1, 0));
            }
            Action::MoveRight => {
                self.try_move(Vector2i32::new(1, 0));
            }
            Action::MoveDown => {
                if !self.try_move_down() {
                    self.lock_piece();
                }
            }
            Action::HardDrop => {
                let mut rows_dropped = 0;
                while self.try_move_down() {
                    rows_dropped += 1;
                }

                if rows_dropped > 0 {
                    self.update_score(0, rows_dropped);
                }

                self.lock_piece();
            }
            Action::RotateCW => {
                self.try_rotate(RotationDirection::Clockwise);
            }
            Action::RotateCCW => {
                self.try_rotate(RotationDirection::CounterClockwise);
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) -> bool {
        if self.state != GameState::Playing || self.current_piece.is_none() {
            println!("Current piece is none?");
            return false;
        }

        self.gravity_timer += delta_time;
        if self.gravity_timer >= self.gravity_interval {
            self.gravity_timer = 0.0;
            if !self.try_move_down() {
                self.lock_piece();
            }
        }

        true
        // add logic for increasing level and speed
    }

    pub fn reset(&mut self) {
        *self = Game::new();
        self.spawn_piece();
    }

    fn generate_piece() -> Piece {
        let mut rng = rand::rng();
        let piece_type =
            BlockType::from_usize(rng.random_range(0..7)).expect("Invalid piece type index");

        let initial_piece_position = Vector2i32::new(GRID_WIDTH as i32 + 2, 2);
        Piece::new(piece_type, initial_piece_position)
    }

    fn check_piece_collision(
        &self,
        piece: &Piece,
        potential_position: Vector2i32,
        potential_rotation_state: usize,
    ) -> bool {
        let potential_block_position =
            piece.absolute_block_positions_at(potential_position, potential_rotation_state);

        for block_pos in &potential_block_position {
            if self.grid.is_coord_occupied(*block_pos) {
                return true;
            }
        }

        false
    }

    fn lock_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            let absolute_positions = piece.absolute_block_positions();
            for block_pos in &absolute_positions {
                if self.grid.is_coord_within_bounds(*block_pos) {
                    if let Some(cell) = self
                        .grid
                        .get_cell_mut(block_pos[0] as usize, block_pos[1] as usize)
                    {
                        cell.block_type = piece.block_type;
                    }
                } else {
                    self.state = GameState::GameOver;
                    return;
                }
            }
            let lines_cleared = self.clear_lines();
            self.update_score(lines_cleared, 0);
            self.spawn_piece();
        }
    }

    fn clear_lines(&mut self) -> u32 {
        self.grid.clear_lines()
    }

    fn update_score(&mut self, lines_cleared: u32, rows_dropped: u32) {
        if lines_cleared == 0 && rows_dropped == 0 {
            return;
        }

        let score_per_line = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };

        let line_score = score_per_line * self.level;
        self.score += line_score + rows_dropped;
        self.lines_cleared += lines_cleared;
        self.level = (self.lines_cleared / 10) + 1;
    }

    fn try_move_down(&mut self) -> bool {
        self.try_move(Vector2i32::new(0, 1))
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new() {
        let game = Game::new();
        assert_eq!(game.grid.width(), GRID_WIDTH);
        assert_eq!(game.grid.height(), GRID_HEIGHT);
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
        assert_eq!(current_piece.pos[0], (GRID_WIDTH / 2) as i32);
        assert_eq!(current_piece.pos[1], 0);
        assert_eq!(current_piece.rot_state, 0);
        assert!(game.next_piece.block_type != BlockType::Empty);
    }
}
