use super::Color;

pub enum DrawFrom {
    Deck,
    Discard { color: Color },
}
