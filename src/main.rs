use astar::*;
use iced::{window, Sandbox, Settings, Column, Row, Text, button, Button, Length, slider, Slider};
use rand;

const WINDOW_SIZE_X: u32 = 1280;
const WINDOW_SIZE_Y: u32 = 720;
fn main() -> iced::Result {
    AStarUI::run(Settings {
        window: window::Settings {
            size: (WINDOW_SIZE_X, WINDOW_SIZE_Y),
            ..Default::default()
        },
        ..Default::default()
    })
}

enum AppState {
    Playing {
        generate_button: button::State,
        find_button: button::State,
        slider: slider::State,
    },
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Playing {
            generate_button: button::State::new(),
            find_button: button::State::new(),
            slider: slider::State::new(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum PointType {
    PointA,
    PointB,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CreateGame,
    FindPath,
    PlacePoint(usize, usize),
    GameSizeChange(u16),
}

struct AStarUI {
    game: GameData,
    game_size: u16,
    new_game_size: u16,
    board: Vec<Vec<button::State>>,
    point: PointType,
    found_path: Vec<Cell>,
    state: AppState,
}

impl Sandbox for AStarUI {
    type Message = Message;

    fn new() -> AStarUI {
        AStarUI {
            game: GameData {
                board: (0..consts::MAX_BOARD_HEIGHT).map(|y| {
                    (0..consts::MAX_BOARD_WIDTH).map(|x| {
                        Cell {
                            x,
                            y,
                            solid: false,
                        }
                    }).collect()
                }).collect(),
                point_a: None,
                point_b: None,
                board_size_x: consts::MAX_BOARD_WIDTH as i32,
                board_size_y: consts::MAX_BOARD_HEIGHT as i32,
            },
            board: (0..consts::MAX_BOARD_HEIGHT).map(|_| {
                (0..consts::MAX_BOARD_WIDTH).map(|_| {
                    button::State::new()
                }).collect()
            }).collect(),
            game_size: 20,
            new_game_size: 20,
            point: PointType::PointA,
            found_path: Vec::new(),
            state: AppState::default(),
        }
    }

    fn title(&self) -> String {
        String::from("A Star Pathfinding")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CreateGame => {
                self.game_size = self.new_game_size;
                self.game = GameData {
                    board: (0..self.game_size).map(|y| {
                        (0..self.game_size).map(|x| {
                            Cell {
                                x: x.into(),
                                y: y.into(),
                                solid: rand::random(),
                            }
                        }).collect()
                    }).collect(),
                    point_a: None,
                    point_b: None,
                    board_size_x: self.game_size as i32,
                    board_size_y: self.game_size as i32,
                };
                self.board = (0..self.game_size).map(|_| {
                    (0..self.game_size).map(|_| {
                        button::State::new()
                    }).collect()
                }).collect();
                self.found_path.clear();
            },
            Message::FindPath => {
                let point_a = match &self.game.point_a {
                    Some(point) => point,
                    None => return,
                };
                let point_b = match &self.game.point_b {
                    Some(point) => point,
                    None => return,
                };
                let found_path = pathfinding::find_path(&self.game, &point_a, &point_b);
                match found_path {
                    None => return,
                    Some(path) => {
                        self.found_path = path;
                    }
                }
            },
            Message::PlacePoint(x, y) => {
                if self.game.board[y][x].solid { return; }
                self.found_path.clear();
                let new_point = Cell {
                    x,
                    y,
                    solid: false
                };
                if self.point == PointType::PointA {
                    self.game.point_a = Some(new_point);
                    self.point = PointType::PointB;
                } else {
                    self.game.point_b = Some(new_point);
                    self.point = PointType::PointA;
                }
            },
            Message::GameSizeChange(new_size) => {
                self.new_game_size = new_size;
            },
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let point_a = &self.game.point_a;
        let point_b = &self.game.point_b;
        let button_size = (WINDOW_SIZE_Y as f32 / self.game.board_size_y as f32) as u16;
        let board = self.board
            .iter_mut()
            .enumerate()
            .fold(Row::new(), |row, (y, v)| {
                row.push(
                    v.iter_mut()
                        .enumerate()
                        .fold(Column::new(), |clm, (x, btn)| {
                            let button_style = match self.game.board[y][x].solid {
                                true => { astar::style::Button::Solid }
                                false => { astar::style::Button::Empty }
                            };

                            let button_style = match point_a {
                                Some(val) => {
                                    if val.x == x && val.y == y {
                                        astar::style::Button::Point
                                    } else {
                                        button_style
                                    }
                                },
                                None => button_style,
                            };

                            let button_style = match point_b {
                                Some(val) => {
                                    if val.x == x && val.y == y {
                                        astar::style::Button::Point
                                    } else {
                                        button_style
                                    }
                                },
                                None => button_style,
                            };

                            let button_style = if self.found_path.contains(&self.game.board[y][x]) { astar::style::Button::Path }
                            else { button_style };

                            clm.push(
                                Button::new(btn,
                                            Text::new(" ")
                                )
                                .on_press(Message::PlacePoint(x, y))
                                .height(Length::Units(button_size))
                                .width(Length::Units(button_size))
                                .min_width(32)
                                .min_height(32)
                                .style(button_style)
                            )
                        })
                )
            });


        match &mut self.state {
            AppState::Playing { generate_button, find_button, slider } => {
                Row::new()
                    .push(Column::new()
                        .push(Text::new("To generate a new map,\nselect map size using the slider\n and click 'Generate'\n\n\nThen select two tiles (A and B)\n and click 'Find Path'.\n"))
                        .push(Slider::new(
                                slider,
                                4..=42,
                                self.new_game_size,
                                Message::GameSizeChange,
                         ))
                        .push(Button::new(generate_button, Text::new("Generate"))
                               .on_press(Message::CreateGame))
                        .push(Button::new(find_button, Text::new("Find Path"))
                               .on_press(Message::FindPath))
                        .push(Text::new(format!("Point A: {:?},\nPoint B: {:?}",
                                                 self.game.point_a,
                                                 self.game.point_b)))
                        .max_width(300)
                        .spacing(16)
                    )
                    .push(board)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(iced::Align::Center)
                    .into()
            },
        }
    }
}
