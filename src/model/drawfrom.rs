use super::Color;

#[derive(Clone, Copy)]
pub enum DrawFrom {
    Deck,
    Discard { color: Color },
}
