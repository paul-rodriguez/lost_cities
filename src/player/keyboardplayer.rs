use super::{PlayDecision, Player};
use crate::error::{BoxType, Boxable, Error};
use crate::model::{Card, Game};
use std::io;
use std::io::Read;
use std::vec::Vec;

pub struct KeyboardPlayer {
    input: Box<dyn io::Read>,
    output: Box<dyn io::Write>,
}

impl KeyboardPlayer {
    pub fn new(input: Box<dyn io::Read>, output: Box<dyn io::Write>) -> KeyboardPlayer {
        KeyboardPlayer {
            input: input,
            output: output,
        }
    }

    fn readn<const SIZE: usize>(&mut self) -> Result<String, Error> {
        let mut buf = [0; SIZE];
        match self.input.read_exact(&mut buf) {
            Ok(_) => (),
            Err(e) => {
                return Err(Error::IOError {
                    cause: e.to_string(),
                })
            }
        };
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}

impl Player for KeyboardPlayer {
    fn makeDecision(&mut self, game: &Game) -> Result<PlayDecision, Error> {
        let errType = BoxType::CannotPlay;

        let playTo = todo!();
        let drawFrom = todo!();
        let cardStr = self.readn::<2>().boxed(errType)?;
        let card = cardStr.parse().boxed(errType)?;

        Ok(PlayDecision {
            card,
            playTo,
            drawFrom,
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
        let player = KeyboardPlayer::new(Box::new(io::stdin()), Box::new(io::stdout()));
    }

    #[test]
    fn test_makeDecision() {
        let mut rng = testRng();
        let game = Game::new(&mut rng);
        let fakeIn = Box::new(io::Cursor::new("y5e\rd"));
        let mut outBuf = Vec::<u8>::new();
        let fakeOut = Box::new(io::Cursor::new(outBuf));
        let mut player = KeyboardPlayer::new(fakeIn, fakeOut);
        let decision = player.makeDecision(&game);
    }
}
