mod game;
mod game_over;
mod how_to_play;
mod menu;
mod types;

use game::game_loop;
use how_to_play::how_to_play_loop;
use menu::menu_loop;
use types::GameState;

fn main() {
    let mut game_state = GameState::Menu;
    let (mut rl, thread) = raylib::init().size(800, 800).title("Bugsweeper").build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        match game_state {
            GameState::Menu => menu_loop(&mut game_state, &mut rl, &thread),
            GameState::Start => game_loop(&mut game_state, &mut rl, &thread),
            GameState::HowToPlay => how_to_play_loop(&mut game_state, &mut rl, &thread),
            GameState::Quit => break,
            _ => {}
        }
    }
}
