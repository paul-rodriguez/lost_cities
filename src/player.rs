mod keyboardplayer;
mod playdecision;

use self::playdecision::PlayDecision;
use crate::error::Error;
use crate::model::Game;

pub trait Player {
    fn makeDecision(&mut self, game: &Game) -> Result<PlayDecision, Error>;
}
