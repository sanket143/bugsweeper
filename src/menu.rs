use raylib::core::math::Rectangle;
use raylib::prelude::*;
use std::ffi::CString;

use crate::types::GameState;

pub fn menu_loop(game_state: &mut GameState, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let top = 110.0;
    let left = 20.0;
    let width = 100.0;
    let height = 30.0;
    let padding = 10.0;

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);

    d.draw_text(
        "BUGSWEEPER",
        left as i32,
        left as i32 - 10,
        100,
        Color::from_hex("DC143C").unwrap(),
    );

    let start = d.gui_button(
        Rectangle {
            x: left,
            y: top,
            width,
            height,
        },
        Some(CString::new("Start").unwrap().as_ref()),
    );

    let how_to_play = d.gui_button(
        Rectangle {
            x: left,
            y: top + height + padding,
            width,
            height,
        },
        Some(CString::new("How to play").unwrap().as_ref()),
    );

    let quit = d.gui_button(
        Rectangle {
            x: left,
            y: top + 2.0 * height + 2.0 * padding,
            width,
            height,
        },
        Some(CString::new("Quit").unwrap().as_ref()),
    );

    if start {
        *game_state = GameState::Start;
    } else if how_to_play {
        *game_state = GameState::HowToPlay;
    } else if quit {
        *game_state = GameState::Quit;
    }
}
