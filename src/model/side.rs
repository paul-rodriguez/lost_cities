#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Side {
    Up,
    Down,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Side::Up => Side::Down,
            Side::Down => Side::Up,
        }
    }
}
