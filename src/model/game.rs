use super::{Board, Card, Deck, Hand, Side};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    board: Board,
    deck: Deck,
    hands: HashMap<Side, Hand>,
    turn: Side,
}

impl Game {
    pub fn new(rng: &mut impl Rng) -> Game {
        let (up, deck0) = Hand::new(Side::Up, &Deck::new(rng)).unwrap();
        let (down, deck1) = Hand::new(Side::Down, &deck0).unwrap();
        Game {
            board: Board::new(),
            deck: deck1,
            hands: maplit::hashmap! {
                Side::Up => up,
                Side::Down => down,
            },
            turn: Side::Up,
        }
    }

    pub fn hand(&self, side: Side) -> Hand {
        self.hands.get(&side).unwrap().clone()
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn deck(&self) -> Deck {
        self.deck.clone()
    }

    pub fn turn(&self) -> Side {
        self.turn
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
        let game = Game::new(&mut rng);
        assert_eq!(game.board(), Board::new());
        let expectedUp = Hand::literal(
            Side::Up,
            &[
                Card::fromId(6),
                Card::fromId(15),
                Card::fromId(17),
                Card::fromId(32),
                Card::fromId(37),
                Card::fromId(50),
                Card::fromId(54),
                Card::fromId(56),
            ],
        )
        .unwrap();
        assert_eq!(game.hand(Side::Up), expectedUp);
        let expectedDown = Hand::literal(
            Side::Down,
            &[
                Card::fromId(0),
                Card::fromId(3),
                Card::fromId(16),
                Card::fromId(26),
                Card::fromId(29),
                Card::fromId(40),
                Card::fromId(42),
                Card::fromId(52),
            ],
        )
        .unwrap();
        assert_eq!(game.hand(Side::Down), expectedDown);
        let expectedDeck = Deck::literal(&[
            44, 34, 2, 4, 45, 47, 1, 10, 14, 57, 9, 36, 27, 21, 20, 33, 19, 8, 43, 51, 35, 18, 31,
            30, 58, 48, 24, 7, 38, 46, 41, 39, 25, 28, 59, 55, 13, 11, 22, 23, 5, 53, 12, 49,
        ]);
        assert_eq!(game.deck(), expectedDeck);
        assert_eq!(game.turn(), Side::Up);
    }
}
