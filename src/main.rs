use rand::Rng;
use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
enum EntityType {
    Bug,
    Mine,
    Empty,
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
    state: TileState,
    entity: EntityType,
}

impl Tile {
    fn new(x: i32, y: i32) -> Self {
        Tile {
            x,
            y,
            state: TileState::Closed,
            entity: EntityType::Empty,
        }
    }
}

fn main() {
    let gap: i32 = 10;
    let no_of_rows: i32 = 10;
    let screen_size: i32 = 800;
    let box_size: i32 = (screen_size - (no_of_rows + 1) * gap) / 10;
    let font_size: i32 = 30;

    let no_of_bugs: i32 = 5;
    let no_of_mines: i32 = 5;

    let mut rng = rand::thread_rng();
    let (mut rl, thread) = raylib::init().size(800, 800).title("Bugsweeper").build();

    rl.set_target_fps(60);

    let crimson = Color::from_hex("DC143C").unwrap();
    let grey = Color::from_hex("333333").unwrap();

    // Initialize board
    let mut board: [[Tile; 10]; 10] = [[Tile::new(0, 0); 10]; 10];

    for i in 0..(no_of_rows as usize) {
        for j in 0..(no_of_rows as usize) {
            board[i][j].x = i as i32 * (box_size + gap) as i32 + gap;
            board[i][j].y = j as i32 * (box_size + gap) as i32 + gap;
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

        for i in 0..(no_of_rows as usize) {
            let row = board[i];

            for j in 0..(row.len()) {
                let tile = row[j];
                if mouse_x > tile.x
                    && mouse_x < tile.x + box_size
                    && mouse_y > tile.y
                    && mouse_y < tile.y + box_size
                {
                    match board[i][j].state {
                        TileState::Opened => {}
                        _ => board[i][j].state = intended_state,
                    }
                } else {
                    match board[i][j].state {
                        TileState::Opened => {}
                        _ => board[i][j].state = TileState::Closed,
                    }
                }
            }
        }

        for row in board.iter() {
            for tile in row {
                let x = tile.x;
                let y = tile.y;

                match tile.state {
                    TileState::Opened => {
                        match tile.entity {
                            EntityType::Bug => {
                                d.draw_rectangle(
                                    x,
                                    y,
                                    box_size,
                                    box_size,
                                    crimson,
                                );
                            }
                            EntityType::Mine => {
                                d.draw_rectangle(
                                    x,
                                    y,
                                    box_size,
                                    box_size,
                                    Color::GOLD,
                                );
                            }
                            EntityType::Empty => {
                                d.draw_rectangle(
                                    x,
                                    y,
                                    box_size,
                                    box_size,
                                    Color::GREEN,
                                );
                            }
                        }
                    },
                    TileState::Closed => d.draw_rectangle(x, y, box_size, box_size, Color::BLACK),
                    TileState::Active => d.draw_rectangle(x, y, box_size, box_size, grey),
                }

                d.draw_text(format!("{}", (x + y) % 7).as_str(), x + (box_size - font_size) / 2 + 8, y + (box_size - font_size) / 2 + 4, font_size, Color::WHITE);
            }
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            intended_state = TileState::Closed;
        }
    }
}
