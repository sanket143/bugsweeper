#[derive(Debug, Copy, Clone)]
pub enum EntityType {
    Bug,
    Mine,
    Empty,
}

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    Menu,
    Start,
    Playing,
    HowToPlay,
    GameOver,
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub enum TileState {
    Active,
    Opened,
    Closed,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub value: u8,
    pub row_index: usize,
    pub col_index: usize,
    pub state: TileState,
    pub entity: EntityType,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Self {
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
