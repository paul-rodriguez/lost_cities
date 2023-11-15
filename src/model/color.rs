use crate::error::Error;
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

impl std::str::FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let character = s.chars().nth(0).ok_or(Error::CannotParseColor {
            cause: Box::new(Error::InputTooShort),
        })?;

        match character {
            'Y' | 'y' => Ok(Color::Yellow),
            'B' | 'b' => Ok(Color::Blue),
            'W' | 'w' => Ok(Color::White),
            'G' | 'g' => Ok(Color::Green),
            'R' | 'r' => Ok(Color::Red),
            default => Err(Error::CannotParseColor {
                cause: Box::new(Error::UnexpectedCharacter { c: character }),
            }),
        }
    }
}
