
use std::rc::Rc;
use super::Card;
use super::Color;

#[derive(Clone)]
pub struct Expedition {
    pub color: Color,
    cards: Rc<Vec<Card>>,
}

impl Expedition {
    pub fn new(color: Color) -> Expedition {
        Expedition {
            color,
            cards: Rc::new(vec![]),
        }
    }

    pub fn with(&self, card: Card) -> Option<Expedition> {
        if !self.canAccept(&card) {
            None
        } else {
            let mut v = Rc::clone(&self.cards);
            Rc::make_mut(&mut v).push(card);
            Some(Expedition {
                color: self.color,
                cards: v,
            })
        }
    }

    pub fn canAccept(&self, card: &Card) -> bool {
        if card.color == self.color {
            match self.top() {
                Some(t) => card.canBeStackedOn(t),
                None => true,
            }
        } else {
            false
        }
    }

    pub fn has(&self, card: Card) -> bool {
        self.cards.contains(&card)
    }

    pub fn nth(&self, n: usize) -> Option<Card> {
        self.cards.get(n).cloned()
    }

    pub fn top(&self) -> Option<Card> {
        self.cards.last().cloned()
    }

    pub fn nbCards(&self) -> usize {
        self.cards.len()
    }
}

