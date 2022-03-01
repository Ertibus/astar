pub mod pathfinding;
pub mod style;

pub mod consts {
    pub const MAX_BOARD_HEIGHT: usize = 20;
    pub const MAX_BOARD_WIDTH: usize = 20;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub solid: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            solid: false,
        }
    }
}

#[derive(Default)]
pub struct GameData {
    pub board: Vec<Vec<Cell>>,
    pub board_size_x: i32,
    pub board_size_y: i32,
    pub point_a: Option<Cell>,
    pub point_b: Option<Cell>,
}
