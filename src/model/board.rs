
use std::rc::Rc;
use itertools::Itertools;
use std::collections::BTreeMap;
use super::HalfBoard;
use super::Card;
use super::Color;
use super::Side;
use super::Value;
use super::Expedition;
use std::fmt;


pub struct Board {
    discard: Rc<BTreeMap<Color, Vec<Card>>>,
    halves: Rc<BTreeMap<Side, HalfBoard>>,
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
                Side::Up => HalfBoard::new(Side::Up),
                Side::Down => HalfBoard::new(Side::Down),
            }),
        }
    }

    pub fn scoreCard(&self, side: Side, card: Card) -> Option<Board> {
        let newHalf = self.half(side).with(card)?;
        let mut newHalves = Rc::clone(&self.halves);
        Rc::make_mut(&mut newHalves).insert(side, newHalf);
        Some(Board {
            discard: Rc::clone(&self.discard),
            halves: newHalves,
        })
    }

    fn half(&self, side: Side) -> &HalfBoard {
        return self.halves.get(&side).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boardDisplay() {
        let b = Board::new();
        assert_eq!(format!("{}", b), "")
    }
}
