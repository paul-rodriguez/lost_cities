use super::{Card, Color, Expedition, Side};
use crate::error::Error;
use core::ops::Range;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::{cmp, fmt};

#[derive(Clone, Debug, PartialEq)]
pub struct Halfboard {
    side: Side,
    expeditions: Rc<BTreeMap<Color, Expedition>>,
}

impl Halfboard {
    pub fn new(side: Side) -> Halfboard {
        let expeditions = Color::all().map(|c| (c, Expedition::new(c))).collect();
        Halfboard {
            side,
            expeditions: Rc::new(expeditions),
        }
    }

    pub fn with(&self, card: Card) -> Result<Halfboard, Error> {
        let exp = self.exp(card.color());
        let newExp = exp.with(card)?;

        let mut newHalf = Rc::clone(&self.expeditions);
        Rc::make_mut(&mut newHalf).insert(newExp.color(), newExp);
        Ok(Halfboard {
            side: self.side,
            expeditions: newHalf,
        })
    }

    pub fn canAccept(&self, card: Card) -> bool {
        self.exp(card.color()).canAccept(card)
    }

    pub fn exp(&self, color: Color) -> &Expedition {
        self.expeditions.get(&color).unwrap()
    }

    fn displayColLen(&self) -> usize {
        let n = self
            .expeditions
            .values()
            .map(|e| e.nbCards())
            .max()
            .unwrap();
        cmp::max(1, n)
    }
}

impl fmt::Display for Halfboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let baseRange: Range<usize> = 0..self.displayColLen();
        let orderRange: Box<dyn Iterator<Item = usize>> = match self.side {
            Side::Down => Box::new(baseRange),
            Side::Up => Box::new(baseRange.rev()),
        };
        let lines = orderRange
            .map(|n| {
                Color::all()
                    .map(|c| match self.expeditions.get(&c).unwrap().nth(n) {
                        Some(card) => format!("{}", card),
                        None => "  ".to_string(),
                    })
                    .join(" | ")
            })
            .join("\n");
        write!(f, "{}", lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canAccept() {
        let h = Halfboard::new(Side::Up).with(Card::fromId(7)).unwrap();
        assert!(h.canAccept(Card::fromId(8)));
        assert!(!h.canAccept(Card::fromId(0)));
    }
}
