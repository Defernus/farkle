use crate::*;

pub struct Player {
    id: String,
    pub(super) score: u32,
    dice: Vec<Dice>,
}

impl Player {
    pub fn new(id: &str, dice: Vec<Dice>) -> Result<Self, PlayerCreationError> {
        if dice.len() != 6 {
            return Err(PlayerCreationError::InvalidDiceCount(dice.len()));
        }

        Ok(Self {
            id: id.to_string(),
            score: 0,
            dice,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn dice(&self) -> Vec<Dice> {
        self.dice.clone()
    }

    pub fn roll(&self) -> Vec<RollResult> {
        self.dice.iter().map(|d| d.roll()).collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlayerCreationError {
    #[error("Expected 6 dice, but got {0}.")]
    InvalidDiceCount(usize),
}
