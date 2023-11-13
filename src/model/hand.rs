use super::{Card, Deck, Error, Side};
use anyhow;
use bit_set::BitSet;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn literal(side: Side, ids: &[u8]) -> anyhow::Result<Hand> {
        if ids.len() > Hand::SIZE {
            anyhow::bail!("Too many cards");
        }
        let empty = BitSet::with_capacity(Card::DECK_SIZE);
        let bits: anyhow::Result<BitSet> = ids.iter().fold(Ok(empty), |b, c| {
            let mut r = b?.clone();
            r.insert((*c).into());
            Ok(r)
        });
        Ok(Hand { side, cards: bits? })
    }

    pub fn toSet(&self) -> HashSet<Card> {
        self.cards
            .iter()
            .map(|n| Card::fromId(n.try_into().unwrap()))
            .collect()
    }

    pub fn toVec(&self) -> Vec<Card> {
        self.cards
            .iter()
            .sorted()
            .map(|n| Card::fromId(n.try_into().unwrap()))
            .collect()
    }

    pub fn take(&self, card: Card) -> Result<Hand, Error> {
        let mut newCards = self.cards.clone();
        let wasPresent = newCards.remove(card.toId().into());
        if wasPresent {
            Ok(Hand {
                side: self.side,
                cards: newCards,
            })
        } else {
            Err(Error::CardNotFound { card })
        }
    }

    pub fn with(&self, card: Card) -> Result<Hand, Error> {
        if self.cards.len() >= Self::SIZE {
            return Err(Error::HandFull);
        }

        let mut newCards = self.cards.clone();
        let ok = newCards.insert(card.toId().into());
        if ok {
            Ok(Hand {
                side: self.side,
                cards: newCards,
            })
        } else {
            Err(Error::DuplicateCard { card })
        }
    }

    pub fn side(&self) -> Side {
        self.side
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cardsStr = self
            .toVec()
            .into_iter()
            .map(|c| format!("{}", c))
            .join(", ");
        write!(f, "{}", cardsStr)
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
        assert_eq!(newDeck.remainingCards(), (Card::DECK_SIZE - Hand::SIZE));
        assert_eq!(format!("{}", hand), "Y5, B2, B4, W7, GB, RB, R5, R7");
    }

    #[test]
    fn test_take() {
        let mut rng = testRng();
        let d = Deck::new(&mut rng);
        let (hand, _) = Hand::new(Side::Up, &d).unwrap();
        let hand2 = hand.take(Card::fromId(50)).unwrap();
        assert_eq!(
            hand2.take(Card::fromId(50)),
            Err(Error::CardNotFound {
                card: Card::fromId(50)
            })
        );
        assert_eq!(
            hand2.take(Card::fromId(20)),
            Err(Error::CardNotFound {
                card: Card::fromId(20)
            })
        );
    }

    #[test]
    fn test_add() {
        let mut rng = testRng();
        let d = Deck::new(&mut rng);
        let (hand, _) = Hand::new(Side::Up, &d).unwrap();
        assert_eq!(hand.with(Card::fromId(0)), Err(Error::HandFull));
        let hand2 = hand.take(Card::fromId(50)).unwrap();
        assert_eq!(
            hand2.with(Card::fromId(15)),
            Err(Error::DuplicateCard {
                card: Card::fromId(15)
            })
        );
        assert_eq!(hand2.with(Card::fromId(50)), Ok(hand));
    }
}
