use crate::error::Error;
use std::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub enum Value {
    Bet,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
}

impl Value {
    pub fn canBeStackedOn(&self, other: Value) -> bool {
        self >= &other
    }

    pub fn family() -> Vec<Value> {
        vec![
            Value::Bet,
            Value::Bet,
            Value::Bet,
            Value::N2,
            Value::N3,
            Value::N4,
            Value::N5,
            Value::N6,
            Value::N7,
            Value::N8,
            Value::N9,
            Value::N10,
        ]
    }

    pub fn familySize() -> u8 {
        12
    }

    pub fn fromId(id: u8) -> Value {
        Value::family()[<u8 as Into<usize>>::into(id % Self::familySize())]
    }

    /// This does not respect the bijection between individual cards and ids.
    ///
    /// This function should only be used in order to get a card that can serve to match value and
    /// color rather than id.
    pub fn prototypeId(self) -> u8 {
        Self::family()
            .iter()
            .position(|v| *v == self)
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match self {
            Value::Bet => "B",
            Value::N2 => "2",
            Value::N3 => "3",
            Value::N4 => "4",
            Value::N5 => "5",
            Value::N6 => "6",
            Value::N7 => "7",
            Value::N8 => "8",
            Value::N9 => "9",
            Value::N10 => "0",
        };
        write!(f, "{}", letter)
    }
}

impl std::str::FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let character = s.chars().nth(0).ok_or(Error::CannotParseValue {
            cause: Box::new(Error::InputTooShort),
        })?;

        match character {
            'B' | 'b' => Ok(Value::Bet),
            '2' => Ok(Value::N2),
            '3' => Ok(Value::N3),
            '4' => Ok(Value::N4),
            '5' => Ok(Value::N5),
            '6' => Ok(Value::N6),
            '7' => Ok(Value::N7),
            '8' => Ok(Value::N8),
            '9' => Ok(Value::N9),
            '0' => Ok(Value::N10),
            default => Err(Error::CannotParseValue {
                cause: Box::new(Error::UnexpectedCharacter { c: character }),
            }),
        }
    }
}
