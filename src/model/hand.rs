use super::{Card, Deck, Side};
use bit_set::BitSet;
use std::collections::HashSet;
use std::rc::Rc;

pub struct Hand {
    side: Side,
    cards: Rc<BitSet>,
}

impl Hand {
    pub fn new(side: Side, deck: &Deck) -> Option<(Hand, Deck)> {
        let mut cards = BitSet::with_capacity(Card::maxId().into());
        let mut newDeck = deck.clone();
        for _ in 0..8 {
            let (card, nextDeck) = newDeck.take()?;
            newDeck = nextDeck;
            cards.insert(card.toId().into());
        }
        Some((
            Hand {
                side,
                cards: Rc::new(cards),
            },
            newDeck,
        ))
    }

    pub fn toSet(&self) -> HashSet<Card> {
        self.cards
            .iter()
            .map(|n| Card::fromId(n.try_into().unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;

    fn fixedRng() -> Xoshiro256StarStar {
        Xoshiro256StarStar::seed_from_u64(41025)
    }

    #[test]
    fn test_new() {
        let mut rng = fixedRng();
        let d = Deck::new(&mut rng);
        let (hand, newDeck) = Hand::new(Side::Up, &d).unwrap();
        let expected = HashSet::from([
            Card::fromId(6),
            Card::fromId(17),
            Card::fromId(15),
            Card::fromId(50),
            Card::fromId(56),
            Card::fromId(37),
            Card::fromId(54),
            Card::fromId(32),
        ]);
        assert_eq!(hand.toSet(), expected);
    }
}
