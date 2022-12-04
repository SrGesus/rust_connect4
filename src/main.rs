use macroquad::prelude::*;

mod graphics;
mod consts;

use graphics::Graphics;
const ROWS: usize = consts::ROWS as usize;
const COLUMNS: usize = consts::COLUMNS as usize;


#[macroquad::main("Connect4")]
async fn main() {
    let mut board = Board::new();
    let mut player = Slot::Red;

    loop {

        if player_turn(&mut board, player).await {
            println!("Player played!");
            player = player.next_player();
        }

        clear_background(WHITE);
        board.draw_table().await;
        next_frame().await
    }
}

// Returns whether a player finished their turn
async fn player_turn(board: &mut Board, player: Slot) -> bool {
    if is_mouse_button_pressed(MouseButton::Left) {
        let side = screen_width().min(screen_height());
        let width_offset = (screen_width() - side) / 2.0;
        let (mouse_x, _) = mouse_position();
        println!("Mouse position: {}; left bound: {}, right bound: {}", mouse_x, width_offset, screen_width() - width_offset);
        if width_offset < mouse_x && mouse_x < screen_width() - width_offset {
            let column = ((mouse_x - width_offset) / side * COLUMNS as f32) as usize;
            println!("Columns number: {}", column);
            match board.insert(column, player) {
                Ok(_) => return true,
                Err(_) => return false
            }
        }
    }
    false
}

#[derive(Debug)]
struct FullColumn;

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    Red,
    Yellow,
}

impl Slot {
    fn is_empty(self) -> bool {
        self == Slot::Empty
    }
    fn next_player(self) -> Slot {
        match self {
            Slot::Yellow => Slot::Red,
            Slot::Red => Slot::Yellow,
            _ => panic!()
        }
    }
}

type Board = [[Slot; COLUMNS]; ROWS];

trait BoardTrait {
    fn new() -> Self;
    fn insert(&mut self, column: usize, player: Slot) -> Result<(), FullColumn>;
}

impl BoardTrait for Board {
    fn new() -> Self {
        [[Slot::Empty; COLUMNS]; ROWS]
    }

    fn insert(&mut self, column: usize, player: Slot) -> Result<(), FullColumn> {
        for i in (0..ROWS).rev() {
            if self[i][column].is_empty() {
                self[i][column] = player;
                return Ok(());
            }
        }
        Err(FullColumn)
    }
}
