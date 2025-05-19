use godot::{classes::InputEvent, prelude::*};
use m2s2_math::Vector2i32;
use tetris_game_logic::{Action, Game, GameState, Piece};

struct TetrisExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TetrisExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct TetrisGameNode {
    base: Base<Node>,
    game: Game,
}

#[godot_api]
impl INode for TetrisGameNode {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Initializing Tetris game node");
        Self {
            base,
            game: Game::new(),
        }
    }

    fn ready(&mut self) {
        let spawned = self.spawn_piece();
        if !spawned {
            godot_print!("Piece not spawned");
        }
    }

    fn process(&mut self, delta: f64) {
        self.process(delta);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if *self.game.state() == GameState::GameOver {
            if event.is_action_pressed("ui_accept") {
                self.game.handle_input(Action::Resume);
            }

            return;
        }

        let action = if event.is_action_pressed("move_left") {
            Some(Action::MoveLeft)
        } else if event.is_action_pressed("move_right") {
            Some(Action::MoveRight)
        } else if event.is_action_pressed("move_down") {
            Some(Action::MoveDown)
        } else if event.is_action_pressed("hard_drop") {
            Some(Action::HardDrop)
        } else if event.is_action_pressed("rotate_clockwise") {
            Some(Action::RotateCW)
        } else if event.is_action_pressed("rotate_counter_clockwise") {
            Some(Action::RotateCCW)
        } else if event.is_action_pressed("ui_cancel") {
            if *self.game.state() == GameState::Playing {
                Some(Action::Pause)
            } else if *self.game.state() == GameState::Paused {
                Some(Action::Resume)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(action) = action {
            self.game.handle_input(action);
        }
    }
}

#[godot_api]
impl TetrisGameNode {
    #[func]
    fn get_game_state(&self) -> i32 {
        match *self.game.state() {
            GameState::Playing => 0,
            GameState::Paused => 1,
            GameState::GameOver => 2,
        }
    }

    #[func]
    fn get_level(&self) -> u32 {
        self.game.level()
    }

    #[func]
    fn get_score(&self) -> u32 {
        self.game.score()
    }

    #[func]
    fn spawn_piece(&mut self) -> bool {
        self.game.spawn_piece()
    }

    #[func]
    fn reset_game(&mut self) {
        self.game.reset();
    }

    #[func]
    fn get_grid_width(&self) -> i32 {
        self.game.grid_width()
    }

    #[func]
    fn get_grid_height(&self) -> i32 {
        self.game.grid_height()
    }

    #[func]
    fn get_grid_data(&self) -> Array<i32> {
        let mut array = Array::new();
        for bt in self.game.get_grid_data() {
            array.push(bt);
        }

        array
    }

    #[func]
    fn get_current_piece_positions(&self) -> Array<Vector2> {
        if let Some(current_piece) = &self.game.curr_piece() {
            return self.get_piece_positions(current_piece);
        }
        Array::new()
    }

    #[func]
    fn get_current_piece_block_type(&self) -> i32 {
        if let Some(cp) = self.game.curr_piece() {
            return self.get_piece_block_type(&cp);
        }
        -1
    }

    #[func]
    fn get_next_piece_positions(&self) -> Array<Vector2> {
        self.get_piece_positions(&self.game.next_piece())
    }

    #[func]
    fn get_next_piece_block_type(&self) -> i32 {
        self.get_piece_block_type(&self.game.next_piece())
    }

    #[func]
    fn process(&mut self, delta: f64) {
        godot_print!(
            "Game update called. State: {:?}, Timer: {}, Interval: {}",
            self.game.state(),
            self.game.grav_timer(),
            self.game.grav_int()
        );
        let was_updated = self.game.update(delta);
        if !was_updated {
            godot_print!("Not updated")
        }
    }

    fn get_piece_block_type(&self, piece: &Piece) -> i32 {
        piece.block_type.into_i32()
    }

    fn get_piece_positions(&self, piece: &Piece) -> Array<Vector2> {
        let mut array = Array::new();
        for pos in self.game.get_piece_abs_positions(piece) {
            array.push(TetrisGameNode::vector2i32_to_vector2(&pos));
        }

        array
    }

    fn vector2i32_to_vector2(vector: &Vector2i32) -> Vector2 {
        Vector2::new(vector[0] as f32, vector[1] as f32)
    }
}
