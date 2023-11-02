use super::{Card, Color};
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

    pub fn top(&self, color: Color) -> Option<Card> {
        let pile = self.colorPile(color);
        pile.last().copied()
    }

    pub fn take(&self, color: Color) -> Option<(Card, DiscardPile)> {
        let mut newPile = self.colorPile(color);
        let card = Rc::make_mut(&mut newPile).pop()?;
        let mut newPiles = self.piles.clone();
        newPiles.insert(color, newPile);
        Some((card, DiscardPile { piles: newPiles }))
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
        assert_eq!(p.top(Color::Red), None)
    }

    #[test]
    fn test_takeEmpty() {
        let p = DiscardPile::empty();
        assert_eq!(p.take(Color::Red), None)
    }

    #[test]
    fn test_takeOk() {
        let p = DiscardPile::empty().with(Card::fromId(0));
        let expected = Some((Card::fromId(0), DiscardPile::empty()));
        assert_eq!(p.take(Color::Yellow), expected)
    }
}
