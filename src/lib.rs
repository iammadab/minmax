mod tictactoe;

use tictactoe::TicTacToe;

fn minmax(game: &TicTacToe) -> i8 {
    if game.is_terminal() {
        return game.value();
    }

    let actions = game.get_actions();
    let values = actions
        .iter()
        .map(|action| minmax(&game.apply_action(action.clone()).unwrap()))
        .collect::<Vec<i8>>();

    return if game.is_min_player() {
        values.iter().min().unwrap().clone()
    } else {
        values.iter().max().unwrap().clone()
    };
}

#[cfg(test)]
mod test {
    use crate::tictactoe::Player;
    use crate::tictactoe::Player::{O, X};
    use crate::{minmax, TicTacToe};
    use std::cmp::min;

    #[test]
    fn test_minmax_evaluation() {
        let game = TicTacToe::new();
        // evaluation for starting position should be 0
        let eval = minmax(&game);
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
        let eval = minmax(&game);
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
        let eval = minmax(&game);
        assert_eq!(eval, 1);

        // Intermediate state but o makes a mistake
        let game = TicTacToe::new();
        // x plays at position 0 should still be a draw
        let game = game.apply_action(0).unwrap();
        assert_eq!(minmax(&game), 0);
        // o makes a mistake and plays at 1
        // x should have a forced win
        let game = game.apply_action(1).unwrap();
        assert_eq!(minmax(&game), 1);
    }
}
