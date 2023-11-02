
use std::fmt;
use std::vec::Vec;

#[derive(Clone,Copy,Eq,Hash,PartialEq,PartialOrd)]
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
    N10
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
            Value::N10 => "1",
        };
        write!(f, "{}", letter)
    }
}
