
use std::fmt;
use std::vec::Vec;
use super::{Color, Value};

#[derive(Clone,Copy,Eq,Hash,PartialEq)]
pub struct Card {
    id: u8,
}

impl Card {
    pub fn fromId(id: u8) -> Card {
        if !Self::isValidId(id) {
            panic!()
        }
        Card{id}
    }

    pub fn color(&self) -> Color {
        Color::fromId(self.id / 12).unwrap()
    }

    pub fn value(&self) -> Value {
        Value::fromId(self.id)
    }

    pub fn set() -> Vec<Card> {
        (0..Self::maxId())
            .map(|id| Card{id})
            .collect()
    }

    pub fn canBeStackedOn(&self, other: Card) -> bool {
        if other.color() == self.color() {
            self.value().canBeStackedOn(other.value())
        } else {
            false
        }
    }

    fn isValidId(id: u8) -> bool {
        id < Self::maxId()
    }

    fn maxId() -> u8 {
        60
    }
}



impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color(), self.value())
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
