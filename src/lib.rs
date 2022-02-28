pub mod pathfinding;
pub mod style;

pub mod consts {
    pub const MAX_BOARD_WIDTH: usize = 8;
    pub const MAX_BOARD_LENGTH: usize = 8;
    pub const GRID_BUTTON_SPACING: u16 = 12;
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
    pub point_a: Option<Cell>,
    pub point_b: Option<Cell>,
}
