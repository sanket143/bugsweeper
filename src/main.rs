use raylib::consts::KeyboardKey;

mod game;
mod types;

use game::game_loop;
use types::GameState;

fn main() {
    let mut game_state = GameState::Menu;
    let (mut rl, thread) = raylib::init().size(800, 800).title("Bugsweeper").build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        game_loop(&mut game_state, &mut rl, &thread);

        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            game_state = GameState::Quit;

            println!("Quitting game");
            break;
        }

        println!("{:?}", game_state);
        match game_state {
            GameState::Quit => break,
            _ => {}
        }
    }
}
