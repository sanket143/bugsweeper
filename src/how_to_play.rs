use raylib::prelude::*;
use std::ffi::CString;

use crate::types::GameState;

pub fn how_to_play_loop(game_state: &mut GameState, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    let box_size = 69;
    let left = 50;
    let top = 40;
    let top_padding = 10;
    let left_padding = 20;
    let font_size = 20;

    d.clear_background(Color::WHITE);

    d.draw_text("How to play!", left, top, 40, Color::BLACK);

    // Closed tile
    d.draw_rectangle(left, top + 70, box_size, box_size, Color::BLACK);
    d.draw_text(
        "Closed tile! You can click on it to open.",
        left + box_size + left_padding,
        top + 55 + box_size / 2,
        font_size,
        Color::BLACK,
    );

    // Opened and safe tile with number
    d.draw_rectangle(
        left,
        top + 70 + top_padding + box_size,
        box_size,
        box_size,
        Color::LIME,
    );
    d.draw_text(
        "Opened tile! The count on the tile shows the number of\nentities around that tile. Max 8.",
        left + box_size + left_padding,
        box_size + top + 55 + box_size / 2,
        font_size,
        Color::BLACK,
    );

    // The bug
    d.draw_rectangle(
        left,
        top + 70 + top_padding * 2 + box_size * 2,
        box_size,
        box_size,
        Color::from_hex("dc143c").unwrap(),
    );
    d.draw_text(
        "The BUG! The can move around in the closed tiles, and\nopening that tile will kill them.",
        left + box_size + left_padding,
        box_size * 2 + top_padding + top + 55 + box_size / 2,
        font_size,
        Color::BLACK,
    );

    // The mine
    d.draw_rectangle(
        left,
        top + 70 + top_padding * 3 + box_size * 3,
        box_size,
        box_size,
        Color::GOLD,
    );
    d.draw_text(
        "The MINE! Opening a tile with a mine will kill you.\nHence, finishing the game.",
        left + box_size + left_padding,
        box_size * 3 + top_padding * 2 + top + 55 + box_size / 2,
        font_size,
        Color::BLACK,
    );

    d.draw_text("Bugs can move around the grid. Every time we open a\ntile, the bug will move one tile to the left/right or top/bottom\nor both (i.e. diagnoal)\n\nUse this information to distinguish between mines and bugs.\nThe GOAL is to kill all the bugs and avoid all the mines by clearing\nall the empty fields.", left, box_size * 4 + top_padding * 3 + top + 55 + box_size / 2, font_size, Color::BLACK);

    let back = d.gui_button(
        Rectangle {
            x: left as f32,
            y: 700.0,
            width: 100.0,
            height: 30.0,
        },
        Some(CString::new("Back").unwrap().as_ref()),
    );

    if back {
        *game_state = GameState::Menu;
    }

    if d.is_key_pressed(KeyboardKey::KEY_Q) {
        *game_state = GameState::Quit;
    }
}
