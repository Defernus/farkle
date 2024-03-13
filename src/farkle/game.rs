use crate::*;

pub struct FarkleGame {
    players: Vec<Player>,
    turn: Turn,
}

impl FarkleGame {
    pub fn new(players: Vec<Player>) -> Result<Self, ZonkCreationError> {
        if players.len() < 2 {
            return Err(ZonkCreationError::NotEnoughPlayers(players.len()));
        }

        Ok(Self {
            turn: Turn::new(players[0].dice(), 0),
            players,
        })
    }

    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.turn.player_index()]
    }

    fn get_next_player_index(&self) -> usize {
        (self.turn.player_index() + 1) % self.players.len()
    }

    pub fn get_next_player(&self) -> &Player {
        &self.players[self.get_next_player_index()]
    }

    pub fn get_last_roll_result(&self) -> Option<Vec<RollResult>> {
        self.turn.get_last_roll_result()
    }

    pub fn roll(&mut self) -> Result<Vec<RollResult>, RollError> {
        self.turn.roll()
    }

    pub fn next_turn(&mut self) {
        if self.turn.is_waiting_for_roll() {
            self.players[self.turn.player_index()].score += self.turn.total_score();
        }

        let next_player_index = self.get_next_player_index();
        self.turn = Turn::new(self.players[next_player_index].dice(), next_player_index);
    }

    pub fn is_waiting_for_roll(&self) -> bool {
        self.turn.is_waiting_for_roll()
    }

    pub fn use_dice(&mut self, dice_indexes: Vec<usize>) -> Result<u32, UseDiceError> {
        let score = self.turn.use_dice(dice_indexes)?;

        if self.turn.is_finished() {
            self.players[self.turn.player_index()].score += self.turn.total_score();

            let next_player_index = self.get_next_player_index();
            self.turn = Turn::new(self.players[next_player_index].dice(), next_player_index);
        }

        Ok(score)
    }

    pub fn try_use_dice(&self, dice_indexes: Vec<usize>) -> Result<u32, UseDiceError> {
        self.turn.clone().use_dice(dice_indexes)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ZonkCreationError {
    #[error("There must be at least 2 players, but only {0} were provided.")]
    NotEnoughPlayers(usize),
}
