use super::{Color, Value};
use crate::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    id: u8,
}

impl Card {
    pub const DECK_SIZE: usize = 60;
    pub const MAX_ID: u8 = 60;

    pub fn fromId(id: u8) -> Card {
        if !Self::isValidId(id) {
            panic!()
        }
        Card { id }
    }

    pub fn color(&self) -> Color {
        Color::fromId(self.id / 12).unwrap()
    }

    pub fn value(&self) -> Value {
        Value::fromId(self.id)
    }

    pub fn set() -> Box<dyn Iterator<Item = Card>> {
        Box::new((0..Self::MAX_ID).map(|id| Card { id }))
    }

    pub fn canBeStackedOn(&self, other: Card) -> bool {
        if other.color() == self.color() {
            self.value().canBeStackedOn(other.value())
        } else {
            false
        }
    }

    fn isValidId(id: u8) -> bool {
        id < Self::MAX_ID
    }

    pub fn toId(self) -> u8 {
        self.id
    }

    /// Cards obtained this way aren't meant to be used as part of a Deck.
    ///
    /// There are three identical copies of each bet card, and this function can only return one of
    /// them.
    /// Unless there's a very good reason not to, Card::fromId should be used instead.
    fn new(color: Color, value: Value) -> Card {
        let colorComponent: u8 = Value::familySize() * color as u8;
        Card::fromId(colorComponent + value.prototypeId())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color(), self.value())
    }
}

impl std::str::FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.get(0..1).ok_or(Error::CannotParseCard {
            cause: Box::new(Error::InputTooShort),
        })?;
        let second = s.get(1..2).ok_or(Error::CannotParseCard {
            cause: Box::new(Error::InputTooShort),
        })?;

        let color: Color = match first.parse() {
            Ok(c) => c,
            Err(e) => return Err(Error::CannotParseCard { cause: Box::new(e) }),
        };

        let value: Value = match second.parse() {
            Ok(v) => v,
            Err(e) => return Err(Error::CannotParseCard { cause: Box::new(e) }),
        };

        Ok(Self::new(color, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let card = Card::new(Color::Blue, Value::Bet);
        assert_eq!(card.color(), Color::Blue);
        assert_eq!(card.value(), Value::Bet);
        assert_eq!(card, Card::fromId(12));
    }

    #[test]
    fn test_fromStr() {
        let r3card: Card = "r3".parse().unwrap();
        assert_eq!(r3card, Card::fromId(52));
        let wbcard: Card = "wb".parse().unwrap();
        assert_eq!(wbcard, Card::fromId(24));
    }
}
