use crate::{TicTacToe, TicTacToeAction};
use rand::seq::SliceRandom;

/// Chooses a random action from the shuffled action list
pub(crate) fn random_strategy(game: &TicTacToe) -> TicTacToeAction {
    let mut actions = game.get_actions();
    actions.shuffle(&mut rand::thread_rng());
    actions[0]
}
