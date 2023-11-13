use super::{Board, Card, Color, Deck, DrawFrom, Error, Hand, PlayTo, Side};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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

    pub fn isOver(&self) -> bool {
        self.deck().isEmpty()
    }

    /// Executes the whole turn of a player
    ///
    /// A card is played to an expedition or a discard pile.
    /// Then a card is drawn from the deck or a discard pile.
    ///
    /// In the implementation, an intermediate Game instance is created to represent the
    /// intermediate step, but that Game is not accessible.
    pub fn play(&self, card: Card, playTo: PlayTo, drawFrom: DrawFrom) -> Result<Game, Error> {
        if self.isOver() {
            return Err(Error::GameOver);
        }
        let afterPlay = self.playFromHand(card, playTo)?;
        match drawFrom {
            DrawFrom::Discard { color } => afterPlay.drawFromDiscard(color),
            DrawFrom::Deck => afterPlay.drawFromDeck(),
        }
    }

    fn handMapWith(&self, hand: Hand) -> HashMap<Side, Hand> {
        let mut newHands = self.hands.clone();
        newHands.insert(hand.side(), hand);
        newHands
    }

    fn playFromHand(&self, card: Card, playTo: PlayTo) -> Result<Game, Error> {
        let handAfterPlay = self.hand(self.turn).take(card)?;
        let boardAfterPlay = self.board().play(self.turn, card, playTo)?;
        Ok(Game {
            board: boardAfterPlay,
            deck: self.deck.clone(),
            hands: self.handMapWith(handAfterPlay),
            turn: self.turn,
        })
    }

    fn drawFromDiscard(&self, color: Color) -> Result<Game, Error> {
        let top = self.board().discardPile().top(color)?;
        let newBoard = self.board().take(top)?;
        let newHand = self.hand(self.turn).with(top)?;
        Ok(Game {
            board: newBoard,
            deck: self.deck.clone(),
            hands: self.handMapWith(newHand),
            turn: self.turn,
        })
    }

    fn drawFromDeck(&self) -> Result<Game, Error> {
        let (card, newDeck) = match self.deck.take() {
            Some((c, d)) => (c, d),
            None => return Err(Error::GameOver),
        };
        let newHand = self.hand(self.turn).with(card)?;
        Ok(Game {
            board: self.board.clone(),
            deck: newDeck,
            hands: self.handMapWith(newHand),
            turn: self.turn,
        })
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
        let expectedUp = Hand::literal(Side::Up, &[6, 15, 17, 32, 37, 50, 54, 56]).unwrap();
        assert_eq!(game.hand(Side::Up), expectedUp);
        let expectedDown = Hand::literal(Side::Down, &[0, 3, 16, 26, 29, 40, 42, 52]).unwrap();
        assert_eq!(game.hand(Side::Down), expectedDown);
        let expectedDeck = Deck::literal(&[
            44, 34, 2, 4, 45, 47, 1, 10, 14, 57, 9, 36, 27, 21, 20, 33, 19, 8, 43, 51, 35, 18, 31,
            30, 58, 48, 24, 7, 38, 46, 41, 39, 25, 28, 59, 55, 13, 11, 22, 23, 5, 53, 12, 49,
        ]);
        assert_eq!(game.deck(), expectedDeck);
        assert_eq!(game.turn(), Side::Up);
    }

    #[test]
    fn test_playNormal() {
        let mut rng = testRng();
        let game = Game::new(&mut rng)
            .play(Card::fromId(15), PlayTo::Expedition, DrawFrom::Deck)
            .unwrap();
        let expectedBoard = Board::new().scoreCard(Side::Up, Card::fromId(15)).unwrap();
        assert_eq!(game.board(), expectedBoard);
        let expectedUp = Hand::literal(Side::Up, &[6, 17, 32, 37, 44, 50, 54, 56]).unwrap();
        assert_eq!(game.hand(Side::Up), expectedUp);
        let expectedDown = Hand::literal(Side::Down, &[0, 3, 16, 26, 29, 40, 42, 52]).unwrap();
        assert_eq!(game.hand(Side::Down), expectedDown);
        let expectedDeck = Deck::literal(&[
            34, 2, 4, 45, 47, 1, 10, 14, 57, 9, 36, 27, 21, 20, 33, 19, 8, 43, 51, 35, 18, 31, 30,
            58, 48, 24, 7, 38, 46, 41, 39, 25, 28, 59, 55, 13, 11, 22, 23, 5, 53, 12, 49,
        ]);
        assert_eq!(game.deck(), expectedDeck);
        assert_eq!(game.turn(), Side::Up);
    }

    #[test]
    fn test_playToDiscard() {
        let mut rng = testRng();
        let game = Game::new(&mut rng)
            .play(Card::fromId(15), PlayTo::Discard, DrawFrom::Deck)
            .unwrap();
        let expectedBoard = Board::new().discardCard(Card::fromId(15));
        assert_eq!(game.board(), expectedBoard);
        let expectedUp = Hand::literal(Side::Up, &[6, 17, 32, 37, 44, 50, 54, 56]).unwrap();
        assert_eq!(game.hand(Side::Up), expectedUp);
        let expectedDown = Hand::literal(Side::Down, &[0, 3, 16, 26, 29, 40, 42, 52]).unwrap();
        assert_eq!(game.hand(Side::Down), expectedDown);
        let expectedDeck = Deck::literal(&[
            34, 2, 4, 45, 47, 1, 10, 14, 57, 9, 36, 27, 21, 20, 33, 19, 8, 43, 51, 35, 18, 31, 30,
            58, 48, 24, 7, 38, 46, 41, 39, 25, 28, 59, 55, 13, 11, 22, 23, 5, 53, 12, 49,
        ]);
        assert_eq!(game.deck(), expectedDeck);
        assert_eq!(game.turn(), Side::Up);
    }

    #[test]
    fn test_playAndTakeBack() {
        let mut rng = testRng();
        let game = Game::new(&mut rng)
            .play(
                Card::fromId(15),
                PlayTo::Discard,
                DrawFrom::Discard { color: Color::Blue },
            )
            .unwrap();
        let expectedBoard = Board::new();
        assert_eq!(game.board(), expectedBoard);
        let expectedUp = Hand::literal(Side::Up, &[6, 15, 17, 32, 37, 50, 54, 56]).unwrap();
        assert_eq!(game.hand(Side::Up), expectedUp);
        let expectedDown = Hand::literal(Side::Down, &[0, 3, 16, 26, 29, 40, 42, 52]).unwrap();
        assert_eq!(game.hand(Side::Down), expectedDown);
        let expectedDeck = Deck::literal(&[
            44, 34, 2, 4, 45, 47, 1, 10, 14, 57, 9, 36, 27, 21, 20, 33, 19, 8, 43, 51, 35, 18, 31,
            30, 58, 48, 24, 7, 38, 46, 41, 39, 25, 28, 59, 55, 13, 11, 22, 23, 5, 53, 12, 49,
        ]);
        assert_eq!(game.deck(), expectedDeck);
        assert_eq!(game.turn(), Side::Up);
    }

    #[test]
    fn test_playGameOver() {
        let game = Game {
            board: Board::new(),
            deck: Deck::literal(&[]),
            hands: maplit::hashmap! {
                Side::Up => Hand::literal(Side::Up, &[6, 15, 17, 32, 37, 50, 54, 56]).unwrap(),
                Side::Down => Hand::literal(Side::Down, &[0, 3, 16, 26, 29, 40, 42, 52]).unwrap(),
            },
            turn: Side::Down,
        };
        let result = game.play(Card::fromId(29), PlayTo::Expedition, DrawFrom::Deck);
        assert_eq!(result, Err(Error::GameOver))
    }

    #[test]
    fn test_playCannotTakeFromEmptyDiscard() {
        let mut rng = testRng();
        let result = Game::new(&mut rng).play(
            Card::fromId(15),
            PlayTo::Expedition,
            DrawFrom::Discard { color: Color::Red },
        );
        assert_eq!(result, Err(Error::DiscardPileEmpty))
    }
}
