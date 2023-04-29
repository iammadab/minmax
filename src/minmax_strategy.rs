use crate::{TicTacToe, TicTacToeAction};
use rand::seq::SliceRandom;

type Evaluation = i8;
type BestMove = u8;

pub(crate) fn minmax_strategy(game: &TicTacToe) -> TicTacToeAction {
    minmax(game).1.unwrap()
}

// TODO: clean this up
fn minmax(game: &TicTacToe) -> (Evaluation, Option<BestMove>) {
    if game.is_terminal() {
        return (game.value(), None);
    }

    let mut actions = game.get_actions();

    // shuffling action list so minmax doesn't always select the same best move
    // in cases where there are more than 1 best moves
    actions.shuffle(&mut rand::thread_rng());

    let values = actions
        .iter()
        .map(|action| minmax(&game.apply_action(action.clone()).unwrap()).0)
        .collect::<Vec<i8>>();
    let action_value = actions.iter().zip(values);

    return if game.is_min_player() {
        action_value
            .min_by_key(|(_, value)| value.clone())
            .map(|(action, value)| (value, Some(*action as u8)))
            .unwrap()
    } else {
        action_value
            .max_by_key(|(_, value)| value.clone())
            .map(|(action, value)| (value, Some(*action as u8)))
            .unwrap()
    };
}

#[cfg(test)]
mod test {
    use crate::minmax_strategy::minmax;
    use crate::tictactoe::Player;
    use crate::tictactoe::Player::{O, X};
    use crate::TicTacToe;
    use std::cmp::min;

    #[test]
    fn test_minmax_evaluation() {
        let game = TicTacToe::new();
        // evaluation for starting position should be 0
        let eval = minmax(&game).0;
        assert_eq!(eval, 0);

        // terminal state where o wins
        let game = TicTacToe::new_from_state(
            [
                Some(X),
                Some(X),
                Some(O),
                Some(X),
                Some(O),
                None,
                Some(O),
                None,
                None,
            ],
            Player::X,
        );
        let eval = minmax(&game).0;
        assert_eq!(eval, -1);

        // terminal state where x wins
        let game = TicTacToe::new_from_state(
            [
                Some(X),
                Some(X),
                Some(X),
                Some(O),
                Some(O),
                None,
                None,
                None,
                None,
            ],
            Player::O,
        );
        let eval = minmax(&game).0;
        assert_eq!(eval, 1);

        // Intermediate state but o makes a mistake
        let game = TicTacToe::new();
        // x plays at position 0 should still be a draw
        let game = game.apply_action(0).unwrap();
        assert_eq!(minmax(&game).0, 0);
        // o makes a mistake and plays at 1
        // x should have a forced win
        let game = game.apply_action(1).unwrap();
        let (eval, best_move) = minmax(&game);
        assert_eq!(eval, 1);
    }
}
