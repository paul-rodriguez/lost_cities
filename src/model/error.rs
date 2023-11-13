use super::Card;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("card not found: {card}")]
    CardNotFound { card: Card },
    #[error("hand already full")]
    HandFull,
    #[error("card already in hand: {card}")]
    DuplicateCard { card: Card },
    #[error("the game is over")]
    GameOver,
    #[error("expedition cannot accept card {card}")]
    CannotAccept { card: Card },
    #[error("the discard pile was empty")]
    DiscardPileEmpty,
}
