
use std::fmt;
use itertools;
use std::vec::Vec;
use super::{Color, Value};

#[derive(Clone,Eq,Hash,PartialEq)]
pub struct Card {
    pub color: super::Color,
    id: u8,
}

impl Card {
    pub fn new(color: Color, id: u8) -> Card {
        if !Value::validId(id) {
            panic!()
        }
        Card{color, id}
    }

    pub fn value(&self) -> Value {
        Value::fromId(self.id)
    }

    pub fn set() -> Vec<Card> {
        itertools::iproduct!(
            super::Color::all(),
            0..Value::familySize())
            .map(|(color, id)|
                Card{color, id: id.try_into().unwrap()})
            .collect()
    }

    pub fn canBeStackedOn(&self, other: Card) -> bool {
        if other.color == self.color {
            self.value().canBeStackedOn(other.value())
        } else {
            false
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color, self.value())
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        card = Card{
            color: Color::White,
            value: Value::Bet,
        }
    }
}
