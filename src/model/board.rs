use std::collections::BTreeMap;
use std::rc::Rc;

use super::{Card, DiscardPile, PlayTo, Side};
use crate::error::Error;
use crate::model::Halfboard;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    discard: Rc<DiscardPile>,
    halves: Rc<BTreeMap<Side, Halfboard>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Up:\n{}\nDiscard:\n{}\nDown:\n{}",
            self.halves.get(&Side::Up).unwrap(),
            self.discard,
            self.halves.get(&Side::Down).unwrap()
        )
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            discard: Rc::new(DiscardPile::empty()),
            halves: Rc::new(maplit::btreemap! {
                Side::Up => Halfboard::new(Side::Up),
                Side::Down => Halfboard::new(Side::Down),
            }),
        }
    }

    pub fn play(self, side: Side, card: Card, playTo: PlayTo) -> Result<Board, Error> {
        match playTo {
            PlayTo::Discard => Ok(self.discardCard(card)),
            PlayTo::Expedition => self.scoreCard(side, card),
        }
    }

    pub fn scoreCard(self, side: Side, card: Card) -> Result<Board, Error> {
        let newHalf = self.half(side).with(card)?;
        let mut newHalves = Rc::clone(&self.halves);
        Rc::make_mut(&mut newHalves).insert(side, newHalf);
        Ok(Board {
            discard: Rc::clone(&self.discard),
            halves: newHalves,
        })
    }

    pub fn discardCard(self, card: Card) -> Board {
        let newDiscardPile = self.discard.with(card);
        Board {
            discard: Rc::new(newDiscardPile),
            halves: Rc::clone(&self.halves),
        }
    }

    pub fn take(self, card: Card) -> Result<Board, Error> {
        let newDiscard = self.discard.take(card)?;
        Ok(Board {
            discard: Rc::new(newDiscard),
            halves: self.halves,
        })
    }

    pub fn discardPile(&self) -> &DiscardPile {
        &self.discard
    }

    fn half(&self, side: Side) -> &Halfboard {
        return self.halves.get(&side).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Color;

    #[test]
    fn test_boardDisplayEmpty() {
        let b = Board::new();
        assert_eq!(
            format!("{}", b),
            "Up:\n   |    |    |    |   \nDiscard:\n |  |  |  | \nDown:\n   |    |    |    |   "
        )
    }

    #[test]
    fn test_boardDisplayOneCard() {
        let b = Board::new().scoreCard(Side::Up, Card::fromId(45)).unwrap();
        assert_eq!(
            format!("{}", b),
            "Up:\n   |    |    | G8 |   \nDiscard:\n |  |  |  | \nDown:\n   |    |    |    |   "
        )
    }

    #[test]
    fn test_boardDisplayTwoCardDown() {
        let b = Board::new()
            .scoreCard(Side::Down, Card::fromId(20))
            .unwrap()
            .scoreCard(Side::Down, Card::fromId(21))
            .unwrap();
        assert_eq!(
            format!("{}", b),
            "Up:\n   |    |    |    |   \nDiscard:\n |  |  |  | \nDown:\n   | B7 |    |    |   \n   | B8 |    |    |   ")
    }

    #[test]
    fn test_boardDisplayTwoCardUp() {
        let b = Board::new()
            .scoreCard(Side::Up, Card::fromId(56))
            .unwrap()
            .scoreCard(Side::Up, Card::fromId(57))
            .unwrap();
        assert_eq!(
            format!("{}", b),
            "Up:\n   |    |    |    | R8\n   |    |    |    | R7\nDiscard:\n |  |  |  | \nDown:\n   |    |    |    |   ")
    }

    #[test]
    fn test_boardScoreCard() {
        let result = Board::new().scoreCard(Side::Up, Card::fromId(5));
        assert_eq!(
            result.unwrap().half(Side::Up).exp(Color::Yellow).top(),
            Some(Card::fromId(5))
        )
    }

    #[test]
    fn test_discardOk() {
        let b = Board::new().discardCard(Card::fromId(40));
        assert_eq!(
            format!("{}", b),
            "Up:\n   |    |    |    |   \nDiscard:\n |  |  | G3 | \nDown:\n   |    |    |    |   "
        )
    }
}
