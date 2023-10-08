
use std::fmt;

#[derive(Clone,Copy,Eq,Hash,PartialEq,PartialOrd,Ord)]
pub enum Color {
    Undefined,
    Yellow,
    Blue,
    White,
    Green,
    Red,
}

impl Color {
    pub fn all() -> impl Iterator<Item = Color> {
        return vec![
            Color::Yellow,
            Color::Blue,
            Color::White,
            Color::Green,
            Color::Red,
        ].into_iter()
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
            Color::Undefined => "U",
        };
        write!(f, "{}", letter)
    }
}
