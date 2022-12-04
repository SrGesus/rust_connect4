use macroquad::prelude::*;
use async_trait::async_trait;

use crate::consts::*;
use crate::board::{Board, Slot};



#[async_trait]
pub trait Graphics {
    async fn draw_table(&self);
}

#[async_trait]
impl Graphics for Board {
    async fn draw_table(&self) {
        let side = screen_width().min(screen_height());
        let width_offset = (screen_width() - side) / 2.0;
        let height_offset = (screen_height() - side) / 2.0;
        for i in 1..COLUMNS as u32 {
            draw_line(
                side * i as f32 / COLUMNS + width_offset,
                height_offset,
                side * i as f32 / COLUMNS + width_offset,
                side + height_offset,
                5.0,
                BLACK,
            );
        }
        for i in 1..ROWS as u32 {
            draw_line(
                width_offset,
                side * i as f32 / ROWS + height_offset,
                side + width_offset,
                side * i as f32 / ROWS + height_offset,
                5.0,
                BLACK,
            );
        }
        draw_rectangle_lines(
            width_offset,
            height_offset,
            side,
            side,
            10.0,
            BLACK,
        );


        let radius = (side / ROWS).min(side / COLUMNS) / 2.0 - 10.0;
        let width_offset = width_offset - side / COLUMNS / 2.0;
        let height_offset = height_offset - side / ROWS / 2.0;


        for i in 0..ROWS as usize {
            for j in 0..COLUMNS as usize {
                match self[i][j] {
                    Slot::Empty => continue,
                    Slot::Red => draw_circle(
                            side * (j + 1) as f32 / COLUMNS + width_offset,
                            side * (i + 1) as f32 / ROWS + height_offset,
                            radius,
                            RED
                        ),
                    Slot::Yellow => draw_circle(
                        side * (j + 1) as f32 / COLUMNS + width_offset,
                        side * (i + 1) as f32 / ROWS + height_offset,
                        radius,
                        YELLOW
                    ),
                }
            }
        }
    }
}