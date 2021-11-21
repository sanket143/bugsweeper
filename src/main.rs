use rand::Rng;
use raylib::prelude::*;
use std::cmp::{max, min};

const GAP: i32 = 10;
const NO_OF_ROWS: i32 = 10;
const SCREEN_SIZE: i32 = 800;
const BOX_SIZE: i32 = (SCREEN_SIZE - (NO_OF_ROWS + 1) * GAP) / 10;
const FONT_SIZE: i32 = 30;

#[derive(Debug, Copy, Clone)]
enum EntityType {
    Bug,
    Mine,
    Empty,
}

#[derive(Debug, Copy, Clone)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Debug, Copy, Clone)]
enum TileState {
    Active,
    Opened,
    Closed,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    value: u8,
    row_index: usize,
    col_index: usize,
    state: TileState,
    entity: EntityType,
}

impl Tile {
    fn new(x: i32, y: i32) -> Self {
        Tile {
            x,
            y,
            value: 0,
            row_index: 0,
            col_index: 0,
            state: TileState::Closed,
            entity: EntityType::Empty,
        }
    }
}


fn board(d: &RaylibDrawHandle) {

}

fn main() {
    let game_state = GameState::Menu;

    let no_of_bugs: i32 = 5;
    let no_of_mines: i32 = 5;

    let mut rng = rand::thread_rng();
    let (mut rl, thread) = raylib::init().size(800, 800).title("Bugsweeper").build();

    rl.set_target_fps(60);

    let crimson = Color::from_hex("DC143C").unwrap();
    let grey = Color::from_hex("333333").unwrap();

    // Initialize board
    let mut board: [[Tile; 10]; 10] = [[Tile::new(0, 0); 10]; 10];

    for i in 0..(NO_OF_ROWS as usize) {
        for j in 0..(NO_OF_ROWS as usize) {
            board[i][j].row_index = j;
            board[i][j].col_index = i;
            board[i][j].x = i as i32 * (BOX_SIZE + GAP) as i32 + GAP;
            board[i][j].y = j as i32 * (BOX_SIZE + GAP) as i32 + GAP;
        }
    }

    // Add bugs
    for _ in 0..(no_of_bugs) {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);

        board[x][y].entity = EntityType::Bug;
    }

    // Add mines
    for _ in 0..(no_of_mines) {
        let x = rng.gen_range(0..10);
        let y = rng.gen_range(0..10);

        board[x][y].entity = EntityType::Mine;
    }

    let mut intended_state = TileState::Closed;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut move_bugs = false;

        d.clear_background(Color::WHITE);

        let mouse_pos = d.get_mouse_position();
        let mouse_x = mouse_pos.x as i32;
        let mouse_y = mouse_pos.y as i32;

        // if left click is pressed && mouse is over a tile 'A' then tile 'A' should turn to 'grey'
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            intended_state = TileState::Active;
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            intended_state = TileState::Opened;
        }

        for i in 0..(NO_OF_ROWS as usize) {
            let row = board[i];

            for j in 0..(row.len()) {
                let tile = row[j];
                if mouse_x > tile.x
                    && mouse_x < tile.x + BOX_SIZE
                    && mouse_y > tile.y
                    && mouse_y < tile.y + BOX_SIZE
                {
                    match board[i][j].state {
                        TileState::Opened => {}
                        TileState::Active => {
                            if matches!(intended_state, TileState::Opened) {
                                move_bugs = true;
                            }
                            board[i][j].state = intended_state
                        }
                        TileState::Closed => board[i][j].state = intended_state,
                    }
                } else {
                    match board[i][j].state {
                        TileState::Opened => {}
                        TileState::Active => board[i][j].state = TileState::Closed,
                        TileState::Closed => {
                            board[i][j].state = TileState::Closed;
                        }
                    }
                }
            }
        }

        if move_bugs {
            for i in 0..(NO_OF_ROWS as usize) {
                for j in 0..(NO_OF_ROWS as usize) {
                    let tile = board[j][i];
                    if matches!(tile.entity, EntityType::Bug) && matches!(tile.state, TileState::Closed) {
                        let x = tile.row_index;
                        let y = tile.col_index;

                        let x_next = (x as i32 + rng.gen_range(0..3) - 1) as i32;
                        let y_next = (y as i32 + rng.gen_range(0..3) - 1) as i32;

                        let x_next_index = min(max(x_next, 0), NO_OF_ROWS - 1) as usize;
                        let y_next_index = min(max(y_next, 0), NO_OF_ROWS - 1) as usize;

                        if matches!(board[x_next_index][y_next_index].entity, EntityType::Empty)
                            && matches!(board[x_next_index][y_next_index].state, TileState::Closed)
                        {
                            board[x_next_index][y_next_index].entity = EntityType::Bug;
                            board[j][i].entity = EntityType::Empty;
                        }
                    }
                }
            }
        }

        for i in 0..(NO_OF_ROWS as usize) {
            let row = board[i];

            for j in 0..(row.len()) {
                let mut tile = row[j];

                let x = tile.x;
                let y = tile.y;

                match tile.state {
                    TileState::Opened => {
                        match tile.entity {
                            EntityType::Bug => {
                                d.draw_rectangle(x, y, BOX_SIZE, BOX_SIZE, crimson);
                            }
                            EntityType::Mine => {
                                d.draw_rectangle(x, y, BOX_SIZE, BOX_SIZE, Color::GOLD);
                            }
                            EntityType::Empty => {
                                d.draw_rectangle(x, y, BOX_SIZE, BOX_SIZE, Color::GREEN);
                            }
                        }

                        let grounds_x = max(0 as i8, tile.row_index as i8 - 1);
                        let grounds_y = max(0 as i8, tile.col_index as i8 - 1);
                        let grounds_x_max = min(NO_OF_ROWS as i8, tile.row_index as i8 + 2);
                        let grounds_y_max = min(NO_OF_ROWS as i8, tile.col_index as i8 + 2);

                        tile.value = 0;

                        for i in grounds_x..grounds_x_max {
                            for j in grounds_y..grounds_y_max {
                                match board[j as usize][i as usize].entity {
                                    EntityType::Bug => tile.value += 1,
                                    EntityType::Mine => tile.value += 1,
                                    EntityType::Empty => {}
                                }
                            }
                        }
                    }
                    TileState::Closed => d.draw_rectangle(x, y, BOX_SIZE, BOX_SIZE, Color::BLACK),
                    TileState::Active => d.draw_rectangle(x, y, BOX_SIZE, BOX_SIZE, grey),
                }

                match tile.entity {
                    EntityType::Empty => {
                        if matches!(tile.state, TileState::Opened) {
                            d.draw_text(
                                format!("{}", tile.value).as_str(),
                                x + (BOX_SIZE - FONT_SIZE) / 2 + 8,
                                y + (BOX_SIZE - FONT_SIZE) / 2 + 4,
                                FONT_SIZE,
                                Color::WHITE,
                            )
                        }
                    }
                    _ => {}
                }
            }
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            intended_state = TileState::Closed;
        }
    }
}
