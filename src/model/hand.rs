use super::{Card, Deck, Side};
use bit_set::BitSet;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Hand {
    side: Side,
    cards: BitSet,
}

impl Hand {
    pub const SIZE: usize = 8;

    pub fn new(side: Side, deck: &Deck) -> Option<(Hand, Deck)> {
        let mut cards = BitSet::with_capacity(Card::DECK_SIZE);
        let mut newDeck = deck.clone();
        for _ in 0..Self::SIZE {
            let (card, nextDeck) = newDeck.take()?;
            newDeck = nextDeck;
            cards.insert(card.toId().into());
        }
        Some((Hand { side, cards }, newDeck))
    }

    pub fn toSet(&self) -> HashSet<Card> {
        self.cards
            .iter()
            .map(|n| Card::fromId(n.try_into().unwrap()))
            .collect()
    }

    pub fn take(&self, card: Card) -> Option<Hand> {
        let mut newCards = self.cards.clone();
        let wasPresent = newCards.remove(card.toId().into());
        if wasPresent {
            Some(Hand {
                side: self.side,
                cards: newCards,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_xoshiro::rand_core::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;

    fn testRng() -> Xoshiro256StarStar {
        Xoshiro256StarStar::seed_from_u64(41025)
    }

    #[test]
    fn test_new() {
        let mut rng = testRng();
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
        assert_eq!(newDeck.remainingCards(), (Card::DECK_SIZE - Hand::SIZE))
    }

    #[test]
    fn test_take() {
        let mut rng = testRng();
        let d = Deck::new(&mut rng);
        let (hand, _) = Hand::new(Side::Up, &d).unwrap();
        let hand2 = hand.take(Card::fromId(50)).unwrap();
        assert_eq!(hand2.take(Card::fromId(50)), None);
        assert_eq!(hand2.take(Card::fromId(20)), None);
    }
}
