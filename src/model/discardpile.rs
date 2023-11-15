use super::{Card, Color};
use crate::error::Error;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct DiscardPile {
    piles: BTreeMap<Color, Rc<Vec<Card>>>,
}

impl DiscardPile {
    pub fn empty() -> DiscardPile {
        let emptyPiles: BTreeMap<Color, Rc<Vec<Card>>> =
            Color::all().map(|color| (color, Rc::new(vec![]))).collect();

        DiscardPile { piles: emptyPiles }
    }

    pub fn with(&self, card: Card) -> DiscardPile {
        let color = card.color();
        let mut newPile = Rc::clone(self.piles.get(&color).unwrap());
        Rc::make_mut(&mut newPile).push(card);
        let mut newPiles = self.piles.clone();
        newPiles.insert(color, newPile);
        DiscardPile { piles: newPiles }
    }

    pub fn top(&self, color: Color) -> Result<Card, Error> {
        let pile = self.colorPile(color);
        match pile.last().copied() {
            None => Err(Error::DiscardPileEmpty),
            Some(card) => Ok(card),
        }
    }

    pub fn take(&self, card: Card) -> Result<DiscardPile, Error> {
        let mut newPile = self.colorPile(card.color());
        match Rc::make_mut(&mut newPile).pop() {
            None => return Err(Error::CardNotFound { card }),
            Some(_) => (),
        };
        let mut newPiles = self.piles.clone();
        newPiles.insert(card.color(), newPile);
        Ok(DiscardPile { piles: newPiles })
    }

    fn colorPile(&self, color: Color) -> Rc<Vec<Card>> {
        Rc::clone(self.piles.get(&color).unwrap())
    }
}

impl fmt::Display for DiscardPile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = Color::all()
            .map(|color| {
                self.colorPile(color)
                    .iter()
                    .map(|card| format!("{}", card))
                    .join(",")
            })
            .join(" | ");
        write!(f, "{}", line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displayEmpty() {
        let p = DiscardPile::empty();
        assert_eq!(format!("{}", p), " |  |  |  | ")
    }

    #[test]
    fn test_displayOneCard() {
        let p = DiscardPile::empty().with(Card::fromId(32));
        assert_eq!(format!("{}", p), " |  | W7 |  | ")
    }

    #[test]
    fn test_displayTwoCards() {
        let p = DiscardPile::empty()
            .with(Card::fromId(0))
            .with(Card::fromId(1));
        assert_eq!(format!("{}", p), "YB,YB |  |  |  | ")
    }

    #[test]
    fn test_topEmpty() {
        let p = DiscardPile::empty();
        assert_eq!(p.top(Color::Red), Err(Error::DiscardPileEmpty))
    }

    #[test]
    fn test_takeEmpty() {
        let p = DiscardPile::empty();
        assert_eq!(
            p.take(Card::fromId(39)),
            Err(Error::CardNotFound {
                card: Card::fromId(39)
            })
        )
    }

    #[test]
    fn test_takeOk() {
        let p = DiscardPile::empty().with(Card::fromId(0));
        let expected = Ok(DiscardPile::empty());
        assert_eq!(p.take(Card::fromId(0)), expected)
    }
}
