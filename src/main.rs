extern crate nalgebra as na;

use nalgebra::base::Vector2;
use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    open: bool,
}

impl Tile {
    fn new(x: i32, y: i32) -> Self {
        Tile { x, y, open: false }
    }
}

fn main() {
    let screen_size: i32 = 800;
    let gap: i32 = 10;
    let no_of_rows: i32 = 10;
    let box_size: i32 = (screen_size - (no_of_rows + 1) * gap) / 10;

    let (mut rl, thread) = raylib::init().size(800, 800).title("Bugsweeper").build();

    rl.set_target_fps(60);

    let crimson = Color::from_hex("DC143C").unwrap();
    let grey = Color::from_hex("000333").unwrap();

    let mut board: [[Tile; 10]; 10] = [[Tile::new(0, 0); 10]; 10];

    for i in 0..(no_of_rows as usize) {
        for j in 0..(no_of_rows as usize) {
            board[i][j].x = i as i32 * (box_size + gap) as i32 + gap;
            board[i][j].y = j as i32 * (box_size + gap) as i32 + gap;
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for row in board.iter() {
            for tile in row {
                let x = tile.x;
                let y = tile.y;

                match tile.open {
                    true => d.draw_rectangle(x, y, box_size, box_size, crimson),
                    false => d.draw_rectangle(x, y, box_size, box_size, Color::BLACK),
                }
            }
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = d.get_mouse_position();
            let mouse_x = mouse_pos.x as i32;
            let mouse_y = mouse_pos.y as i32;

            for i in 0..(no_of_rows as usize) {
                let row = board[i];

                for j in 0..(row.len()) {
                    let tile = row[j];
                    if mouse_x > tile.x
                        && mouse_x < tile.x + box_size
                        && mouse_y > tile.y
                        && mouse_y < tile.y + box_size
                    {
                        board[i][j].open = !board[i][j].open;
                    }
                }
            }
        }

        d.draw_text(
            format!("FPS: {:?}", d.get_mouse_position()).as_str(),
            16,
            16,
            20,
            Color::BLACK,
        );
    }
}
