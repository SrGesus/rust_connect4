use macroquad::prelude::*;

mod consts;
mod graphics;
mod game;
mod board;

use game::*;

#[macroquad::main("Connect4")]
async fn main() {

    let mut game = Game::new();

    loop {
        clear_background(WHITE);

        game.run().await;

        next_frame().await
    }
}