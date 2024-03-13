use crate::*;

#[derive(Clone)]
pub struct Turn {
    dice: Vec<Dice>,
    player_index: usize,
    actions_score: Vec<u32>,
    turn_state: TurnState,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub(super) enum TurnState {
    #[default]
    WaitForRoll,
    RollResult(Vec<RollResult>),
}

impl Turn {
    pub fn new(player: &Player) -> Self {
        Self {
            dice: player.dice().clone(),
            player_index: 0,
            actions_score: Vec::new(),
            turn_state: TurnState::WaitForRoll,
        }
    }

    pub fn total_score(&self) -> u32 {
        self.actions_score.iter().sum()
    }

    pub fn player_index(&self) -> usize {
        self.player_index
    }

    pub fn is_waiting_for_roll(&self) -> bool {
        matches!(self.turn_state, TurnState::WaitForRoll)
    }

    pub fn roll(&mut self) -> Result<Vec<RollResult>, RollError> {
        if !self.is_waiting_for_roll() {
            return Err(RollError::InvalidState);
        }

        let roll_result = self.dice.iter().map(|d| d.roll()).collect::<Vec<_>>();

        self.turn_state = TurnState::RollResult(roll_result.clone());

        Ok(roll_result)
    }

    pub fn get_last_roll_result(&self) -> Option<Vec<RollResult>> {
        match &self.turn_state {
            TurnState::RollResult(roll_result) => Some(roll_result.clone()),
            _ => None,
        }
    }

    pub fn has_any_combination(&self) -> bool {
        let state = match &self.turn_state {
            TurnState::RollResult(roll_result) => roll_result,
            _ => return false,
        };

        match get_score(state.clone()) {
            Ok(_) => true,
            // check if we have any combination to use
            Err(rest) => rest.len() != state.len(),
        }
    }

    /// After rolling the dice, the player can choose to use some of them to score points.
    /// This method will return the score of the chosen dice.
    pub fn use_dice(&mut self, dice_indexes: Vec<usize>) -> Result<u32, UseDiceError> {
        let state = match &self.turn_state {
            TurnState::RollResult(roll_result) => roll_result,
            _ => return Err(UseDiceError::InvalidState),
        };

        let mut used_results = Vec::with_capacity(dice_indexes.len());

        for &index in &dice_indexes {
            let result = *state.get(index).ok_or(UseDiceError::WrongDiceIndexes)?;

            used_results.push(result);
        }

        let score = get_score(used_results)
            .map_err(|unused| UseDiceError::InvalidDiceCombination(unused))?;

        self.dice
            .retain_indexed(|index, _| !dice_indexes.contains(&index));

        self.turn_state = TurnState::WaitForRoll;
        self.actions_score.push(score);

        Ok(score)
    }

    pub fn is_finished(&self) -> bool {
        self.dice.is_empty()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RollError {
    #[error("The turn is in an invalid state to roll.")]
    InvalidState,
}

#[derive(thiserror::Error, Debug)]
pub enum UseDiceError {
    #[error("The turn is in an invalid state to use dice.")]
    InvalidState,
    #[error("Wrong dice indexes.")]
    WrongDiceIndexes,
    #[error("Invalid dice combination, unused dice: {0:?}.")]
    InvalidDiceCombination(Vec<RollResult>),
}
