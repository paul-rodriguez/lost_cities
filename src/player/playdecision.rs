use crate::error::Error;
use crate::model::{Card, DrawFrom, Game, PlayTo};

pub struct PlayDecision {
    pub card: Card,
    pub playTo: PlayTo,
    pub drawFrom: DrawFrom,
}

impl PlayDecision {
    pub fn applyTo(&self, game: Game) -> Result<Game, Error> {
        game.play(self.card, self.playTo, self.drawFrom)
    }
}
