use iced::{ button, Background, Color };

pub const EMPTY: Color = Color::from_rgb(
    0xaf as f32 / 255.0,
    0xaf as f32 / 255.0,
    0xaf as f32 / 255.0,
);
pub const SOLID: Color = Color::from_rgb(
    0x02 as f32 / 255.0,
    0x02 as f32 / 255.0,
    0x0b as f32 / 255.0,
);
pub const POINT_A: Color = Color::from_rgb(
    0x44 as f32 / 255.0,
    0xa1 as f32 / 255.0,
    0xa0 as f32 / 255.0,
);
pub const POINT_B: Color = Color::from_rgb(
    0xe6 as f32 / 255.0,
    0xaa as f32 / 255.0,
    0x68 as f32 / 255.0,
);
pub const OPEN: Color = Color::from_rgb(
    0x7f as f32 / 255.0,
    0xb0 as f32 / 255.0,
    0x69 as f32 / 255.0,
);
pub const CLOSED: Color = Color::from_rgb(
    0x57 as f32 / 255.0,
    0x00 as f32 / 255.0,
    0x00 as f32 / 255.0,
);
pub const PATH: Color = Color::from_rgb(
    0x26 as f32 / 255.0,
    0x54 as f32 / 255.0,
    0x7c as f32 / 255.0,
);

pub enum Button {
    Empty,
    Solid,
    Open,
    Closed,
    Path,
    Point,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        let (background, text_color) = match self {
            Button::Empty => (Some(EMPTY), Color::WHITE),
            Button::Solid => (Some(SOLID), Color::WHITE),
            Button::Open => (Some(OPEN), Color::WHITE),
            Button::Closed => (Some(CLOSED), Color::WHITE),
            Button::Path => (Some(PATH), Color::WHITE),
            Button::Point => (Some(POINT_A), Color::WHITE),
        };
        button::Style {
            text_color,
            background: background.map(Background::Color),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        let background = match self {
            Button::Empty => Some(Color::from_rgb(0.5, 0.5, 0.5)),
            Button::Solid => Some(SOLID),
            Button::Open => Some(Color::from_rgb(0.5, 0.5, 0.5)),
            Button::Closed => Some(CLOSED),
            Button::Path => Some(Color::from_rgb(0.5, 0.5, 0.5)),
            Button::Point => Some(Color::from_rgb(0.5, 0.5, 0.5)),
        };
        button::Style {
                background: background.map(Background::Color),
                ..active
            }
    }
}
