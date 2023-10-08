
use itertools::Itertools;
use std::rc::Rc;
use std::collections::BTreeMap;
use super::Card;
use super::Color;
use super::Side;
use super::Value;
use super::Expedition;
use std::{cmp,fmt};

#[derive(Clone)]
pub struct HalfBoard {
    side: Side,
    expeditions: Rc<BTreeMap<Color, Expedition>>
}

impl HalfBoard {
    pub fn new(side: Side) -> HalfBoard {
        let expeditions = Color::all()
            .map(|c| (c, Expedition::new(c)))
            .collect();
        HalfBoard {side, expeditions: Rc::new(expeditions)}
    }

    pub fn with(&self, card: Card) -> Option<HalfBoard> {
        let exp = self.exp(card.color);
        let newExp = exp.with(card)?;

        let mut newHalf = Rc::clone(&self.expeditions);
        Rc::make_mut(&mut newHalf).insert(newExp.color, newExp);
        Some(HalfBoard {
            side: self.side,
            expeditions: newHalf,
        })
    }

    pub fn canAccept(&self, card: Card) -> bool {
        self.exp(card.color).canAccept(&card)
    }

    pub fn exp(&self, color: Color) -> &Expedition {
        self.expeditions.get(&color).unwrap()
    }

    fn displayColLen(&self) -> usize {
        let n = self.expeditions.values().map(|e| e.nbCards()).max().unwrap();
        cmp::max(1, n)
    }
}

impl fmt::Display for HalfBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let orderRange = 0..self.displayColLen();
        let lines = orderRange.map(|n|
            Color::all()
                .map(|c| match self.expeditions.get(&c).unwrap().nth(n.into()) {
                        Some(card) => format!("{}", card),
                        None => "  ".to_string(),
                    })
                .join(" | "))
            .join("\n");
        write!(f, "{}", lines)
    }
}
