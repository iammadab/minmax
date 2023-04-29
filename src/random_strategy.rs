use crate::{TicTacToe, TicTacToeAction};
use rand::seq::SliceRandom;

pub(crate) fn random_strategy(game: &TicTacToe) -> TicTacToeAction {
    let mut actions = game.get_actions();
    actions.shuffle(&mut rand::thread_rng());
    actions[0]
}
