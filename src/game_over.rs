use raylib::prelude::*;
use std::ffi::CString;

use crate::types::GameState;

pub fn game_over_loop(game_state: &mut GameState, d: &mut RaylibDrawHandle) {
    // GAME OVER
    // Play again
    // Main menu

    let mut bg_color = Color::from_hex("333333").unwrap();
    bg_color.a = 230;

    d.draw_rectangle(0, 0, 800, 800, bg_color);

    if matches!(game_state, GameState::GameOver) {
        d.draw_text(
            "GAME OVER!",
            85,
            300,
            100,
            Color::from_hex("DC143C").unwrap(),
        );
    } else if matches!(game_state, GameState::GameComplete) {
        d.draw_text(
            "YOU WON!",
            170,
            300,
            100,
            Color::LIME,
        );
    }

    let play_again = d.gui_button(
        Rectangle {
            x: 240.0,
            y: 400.0,
            width: 110.0,
            height: 40.0,
        },
        Some(CString::new("Play again!").unwrap().as_ref()),
    );

    let main_menu = d.gui_button(
        Rectangle {
            x: 448.0,
            y: 400.0,
            width: 110.0,
            height: 40.0,
        },
        Some(CString::new("Main menu").unwrap().as_ref()),
    );

    if play_again {
        *game_state = GameState::PlayAgain;
    }

    if main_menu {
        *game_state = GameState::Menu;
    }
}
