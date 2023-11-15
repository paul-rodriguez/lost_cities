use crate::model::Card;
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
    #[error("cannot parse card: {cause}")]
    CannotParseCard { cause: Box<Error> },
    #[error("cannot parse color: {cause}")]
    CannotParseColor { cause: Box<Error> },
    #[error("cannot parse value: {cause}")]
    CannotParseValue { cause: Box<Error> },
    #[error("the input was too short")]
    InputTooShort,
    #[error("unexpected character: {c}")]
    UnexpectedCharacter { c: char },
    #[error("cannot play: {cause}")]
    CannotPlay { cause: Box<Error> },
    #[error("I/O error: {cause}")]
    IOError { cause: String },
}

impl Error {
    pub fn boxed(self, boxType: BoxType) -> Error {
        match boxType {
            BoxType::CannotParseCard => Error::CannotParseCard {
                cause: Box::new(self),
            },
            BoxType::CannotParseColor => Error::CannotParseColor {
                cause: Box::new(self),
            },
            BoxType::CannotParseValue => Error::CannotParseValue {
                cause: Box::new(self),
            },
            BoxType::CannotPlay => Error::CannotPlay {
                cause: Box::new(self),
            },
        }
    }
}

pub trait Boxable<T> {
    fn boxed(self, boxType: BoxType) -> Result<T, Error>;
}

impl<T> Boxable<T> for Result<T, Error> {
    fn boxed(self, boxType: BoxType) -> Result<T, Error> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.boxed(boxType)),
        }
    }
}

pub enum BoxType {
    CannotParseCard,
    CannotParseColor,
    CannotParseValue,
    CannotPlay,
}
