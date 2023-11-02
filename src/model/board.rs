
use std::rc::Rc;
use std::collections::BTreeMap;

use crate::model::Halfboard;
use super::Card;
use super::Color;
use super::Side;
use std::fmt;


pub struct Board {
    discard: Rc<BTreeMap<Color, Vec<Card>>>,
    halves: Rc<BTreeMap<Side, Halfboard>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nDiscard\n{}",
            self.halves.get(&Side::Up).unwrap(),
            self.halves.get(&Side::Down).unwrap())
    }
}

impl Board {
    pub fn new() -> Board {
        Board{
            discard: Rc::new(maplit::btreemap!{}),
            halves: Rc::new(maplit::btreemap!{
                Side::Up => Halfboard::new(Side::Up),
                Side::Down => Halfboard::new(Side::Down),
            }),
        }
    }

    pub fn scoreCard(self, side: Side, card: Card) -> Option<Board> {
        let newHalf = self.half(side).with(card)?;
        let mut newHalves = Rc::clone(&self.halves);
        Rc::make_mut(&mut newHalves).insert(side, newHalf);
        Some(Board {
            discard: Rc::clone(&self.discard),
            halves: newHalves,
        })
    }

    fn half(&self, side: Side) -> &Halfboard {
        return self.halves.get(&side).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boardDisplayEmpty() {
        let b = Board::new();
        assert_eq!(format!("{}", b), "   |    |    |    |   \nDiscard\n   |    |    |    |   ")
    }

    #[test]
    fn test_boardDisplayOneCard() {
        let b = Board::new().scoreCard(Side::Up, Card::fromId(45)).unwrap();
        assert_eq!(format!("{}", b), "   |    |    | G8 |   \nDiscard\n   |    |    |    |   ")
    }

    #[test]
    fn test_boardDisplayTwoCardDown() {
        let b = Board::new()
            .scoreCard(Side::Down, Card::fromId(20)).unwrap()
            .scoreCard(Side::Down, Card::fromId(21)).unwrap();
        assert_eq!(format!("{}", b), "   |    |    |    |   \nDiscard\n   | B7 |    |    |   \n   | B8 |    |    |   ")
    }

    #[test]
    fn test_boardDisplayTwoCardUp() {
        let b = Board::new()
            .scoreCard(Side::Up, Card::fromId(56)).unwrap()
            .scoreCard(Side::Up, Card::fromId(57)).unwrap();
        assert_eq!(format!("{}", b), "   |    |    |    | R8\n   |    |    |    | R7\nDiscard\n   |    |    |    |   ")
    }


    #[test]
    fn test_boardScoreCard() {
        let result = Board::new().scoreCard(Side::Up, Card::fromId(5));
        match result {
            Some(_) => (),
            None => panic!()
        }
    }
}
