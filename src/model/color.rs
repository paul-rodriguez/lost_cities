use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Primitive, Ord)]
pub enum Color {
    Yellow = 0,
    Blue = 1,
    White = 2,
    Green = 3,
    Red = 4,
}

impl Color {
    pub fn fromId(id: u8) -> Option<Color> {
        Color::from_u8(id)
    }

    pub fn all() -> impl Iterator<Item = Color> {
        vec![
            Color::Yellow,
            Color::Blue,
            Color::White,
            Color::Green,
            Color::Red,
        ]
        .into_iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match self {
            Color::Yellow => "Y",
            Color::Blue => "B",
            Color::White => "W",
            Color::Green => "G",
            Color::Red => "R",
        };
        write!(f, "{}", letter)
    }
}
