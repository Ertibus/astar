use astar::*;
use iced::{Sandbox, Settings, Column, Row, Text, button, Button, Length};
use rand;

fn main() -> iced::Result {
    AStar::run(Settings::default())
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
}

struct AStar {
    game: GameData,
    board: Vec<Vec<button::State>>,
    point: PointType,
    generate_button: button::State,
    find_button: button::State,
    found_path: Vec<Cell>,
}

impl Sandbox for AStar {
    type Message = Message;

    fn new() -> AStar {
        AStar {
            game: GameData {
                board: (0..consts::MAX_BOARD_LENGTH).map(|y| {
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
            },
            board: (0..consts::MAX_BOARD_LENGTH).map(|_| {
                (0..consts::MAX_BOARD_WIDTH).map(|_| {
                    button::State::new()
                }).collect()
            }).collect(),
            point: PointType::PointA,
            generate_button: button::State::new(),
            find_button: button::State::new(),
            found_path: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("A Star Pathfinding")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::CreateGame => {
                self.game = GameData {
                    board: (0..consts::MAX_BOARD_LENGTH).map(|y| {
                        (0..consts::MAX_BOARD_WIDTH).map(|x| {
                            Cell {
                                x,
                                y,
                                solid: rand::random(),
                            }
                        }).collect()
                    }).collect(),
                    point_a: None,
                    point_b: None,
                };
                self.found_path.clear();
            },
            Message::FindPath => {
                if self.game.point_a == None || self.game.point_b == None {
                    return;
                }
                let found_path = pathfinding::find_path(&self.game);
                match found_path {
                    None => return,
                    Some(path) => {
                        self.found_path = path;
                    }
                }
            },
            Message::PlacePoint(x, y) => {
                if self.game.board[x][y].solid { return; }
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
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let point_a = &self.game.point_a;
        let point_b = &self.game.point_b;

        let board = self.board
            .iter_mut()
            .enumerate()
            .fold(Column::new(), |clm, (y, v)| {
                clm.push(
                    v.iter_mut()
                        .enumerate()
                        .fold(Row::new(), |row, (x, btn)| {
                            let button_style = match self.game.board[x][y].solid {
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

                            let button_style = if self.found_path.contains(&self.game.board[x][y]) { astar::style::Button::Path }
                            else { button_style };

                            row.push(
                                Button::new(btn,
                                            Text::new("")
                                )
                                .on_press(Message::PlacePoint(x, y))
                                .height(Length::Units(72))
                                .width(Length::Units(72))
                                .style(button_style)
                            )
                        })
                )
            });

        let control = Column::new()
            .push(Text::new("To start a new instance,\n click 'Generate'\n\nThen select two tiles (A and B)\n and click 'Find Path'.\n")
                  .vertical_alignment(iced::VerticalAlignment::Center))
            .push(Row::new()
                  .push(Button::new(&mut self.generate_button, Text::new("Generate"))
                        .on_press(Message::CreateGame))
                  .push(Button::new(&mut self.find_button, Text::new("Find Path"))
                        .on_press(Message::FindPath))
                  .spacing(16)
                  )
            .push(Text::new(format!("Point A: {:?},\nPoint B: {:?},\nPath: {:?}", self.game.point_a, self.game.point_b, self.found_path)))
            .align_items(iced::Align::Center)
            .spacing(16);


        Row::new()
            .push(control)
            .push(Row::new().push(board))
            .spacing(16)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Align::Center)
            .into()
    }
}

