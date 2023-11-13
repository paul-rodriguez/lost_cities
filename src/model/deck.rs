use super::Card;
use rand::seq::SliceRandom;
use rand::Rng;
use rpds::Stack;

#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
    cards: Stack<Card>,
}

impl Deck {
    pub fn take(&self) -> Option<(Card, Deck)> {
        let card = self.cards.peek()?;
        let newCards = self.cards.pop()?;
        Some((*card, Deck { cards: newCards }))
    }

    pub fn new(rng: &mut impl Rng) -> Deck {
        let mut cardSlice: Vec<_> = Card::set().collect();
        cardSlice.shuffle(rng);
        let cards: Stack<Card> = cardSlice.into_iter().collect();
        Deck { cards }
    }

    pub fn remainingCards(&self) -> usize {
        self.cards.size()
    }

    pub fn isEmpty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn literal(ids: &[u8]) -> Deck {
        let cards = ids.iter().map(|i| Card::fromId(*i)).collect();
        Deck { cards }
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
    fn test_takeOk() {
        let mut rng = testRng();
        let d = Deck::new(&mut rng);
        let (card, _) = d.take().unwrap();
        assert_eq!(card, Card::fromId(50));
    }

    #[test]
    fn test_takeToEmpty() {
        let mut rng = testRng();
        let mut deck = Deck::new(&mut rng);
        for n in 0..Card::DECK_SIZE {
            assert_eq!(deck.remainingCards(), (Card::DECK_SIZE - n));
            let (_, newDeck) = deck.take().unwrap();
            deck = newDeck;
        }
        let result = deck.take();
        assert_eq!(result, None);
    }
}
