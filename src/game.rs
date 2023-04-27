use crate::consts;
use macroquad::prelude::*;
use std::time::Instant;

use crate::graphics::*;
use crate::board::*;

// const ROWS: usize = consts::ROWS as usize;
const COLUMNS: usize = consts::COLUMNS as usize;

pub struct Game {
    board: Board,
    player: Slot,
    state: GameState,
    outcome: Outcome,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::new();
        let player = Slot::Red;
        let state = GameState::Playing;
        let outcome = Outcome::None;
        Game {
            board,
            player,
            state,
            outcome,
        }
    }

    pub async fn run(&mut self) {
        match self.state {
            GameState::Playing => {
                if self.player_turn().await {
                    println!("Player played!");
                    self.player = self.player.next_player();
                    let now = Instant::now();
                    self.outcome = self.board.get_outcome();
                    println!("Time passed: {:.2?}", now.elapsed());
                    if !self.board.get_outcome().is_none() {
                        self.state = GameState::GameOver;
                    }
                }
                self.board.draw_table().await;
            }
            GameState::GameOver => self.game_over().await,
        }
    }

    async fn game_over(&mut self) {
        self.board.draw_table().await;
        let text = format!("{:?} Won!", self.player.next_player());
        let center = get_text_center(
            text.as_ref(),
            None,
            (screen_height() / 3.0) as u16,
            1.0,
            0.0,
        );

        draw_text(
            text.as_ref(),
            screen_width() / 2.0 - center.x,
            screen_height() / 2.0 - center.y,
            screen_height() / 3.0,
            BLACK,
        );

        self.outcome.draw_score_line().await;
        if is_mouse_button_pressed(MouseButton::Left) {
            self.board.clear();
            self.state = GameState::Playing;
        }
    }

    async fn player_turn(&mut self) -> bool {
        if is_mouse_button_pressed(MouseButton::Left) {
            let side = screen_width().min(screen_height());
            let width_offset = (screen_width() - side) / 2.0;
            let (mouse_x, _) = mouse_position();
            println!(
                "Mouse position: {}; left bound: {}, right bound: {}",
                mouse_x,
                width_offset,
                screen_width() - width_offset
            );

            // If the mouse is within the table, insert a disk on the column the mouse is in
            if width_offset < mouse_x && mouse_x < screen_width() - width_offset {
                let column = ((mouse_x - width_offset) / side * COLUMNS as f32) as usize;
                println!("Columns number: {}", column);
                match self.board.insert(column, self.player) {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }
        false
    }
}

pub enum GameState {
    Playing,
    GameOver,
}
