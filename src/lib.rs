trait Action {}

trait Game<A: Action> {
    /// Returns true if current game state is a terminal state
    fn is_terminal(&self) -> bool;
    /// Returns all possible actions from the current game state
    fn get_actions(&self) -> Vec<A>;
    /// Applies an action to the current state, returns a new state
    fn apply_action(&self, action: A) -> Self;
    /// Get the value of the current state, must be terminal
    fn value(&self) -> u8;
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
}

struct TicTacToe {
    board: [Option<Player>; 9],
    player: Player,
}

impl TicTacToe {
    /// Initializes new tic tac toe state
    fn new() -> Self {
        Self {
            board: [None; 9],
            player: Player::X,
        }
    }

    fn new_from_state(board: [Option<Player>; 9], player: Player) -> Self {
        Self { board, player }
    }

    /// Returns the winner of the current state
    /// Gives an error if the state is non-terminal
    /// None if it's a draw
    /// and the player variant if there is a winner
    fn get_winner(&self) -> Result<Option<Player>, String> {
        // Check horizontal lines
        for i in (0..9).step_by(3) {
            if let Some(player) = self.board[i] {
                if self.board[i + 1] == Some(player) && self.board[i + 2] == Some(player) {
                    return Ok(Some(player));
                }
            }
        }

        // Check vertical lines
        for i in 0..3 {
            if let Some(player) = self.board[i] {
                if self.board[i + 3] == Some(player) && self.board[i + 6] == Some(player) {
                    return Ok(Some(player));
                }
            }
        }

        // Check diagonal lines
        if let Some(player) = self.board[0] {
            if self.board[4] == Some(player) && self.board[8] == Some(player) {
                return Ok(Some(player));
            }
        }

        if let Some(player) = self.board[2] {
            if self.board[4] == Some(player) && self.board[6] == Some(player) {
                return Ok(Some(player));
            }
        }

        // no winner, we need to check if any move can still be made
        let not_draw = self.board.iter().any(|val| val == &None);

        match not_draw {
            true => Err(String::from("not a terminal state")),
            false => Ok(None),
        }
    }
}

type TicTacToeAction = u8;
impl Action for TicTacToeAction {}

#[cfg(test)]
mod tests {
    use crate::Player::{O, X};
    use crate::{Player, TicTacToe};

    #[test]
    fn test_get_winner() {
        let game = TicTacToe::new();
        assert_eq!(game.get_winner().is_err(), true);

        // Terminal state with x winning
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
        let winner = game
            .get_winner()
            .expect("should be terminal")
            .expect("not draw");
        assert_eq!(winner, X);

        // Terminal state with o winning
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
        let winner = game
            .get_winner()
            .expect("should be terminal")
            .expect("not draw");
        assert_eq!(winner, O);

        // Terminal state with a draw
        let game = TicTacToe::new_from_state(
            [
                Some(X),
                Some(O),
                Some(X),
                Some(X),
                Some(O),
                Some(O),
                Some(O),
                Some(X),
                Some(X),
            ],
            Player::X,
        );
        let result = game.get_winner().expect("should be terminal");
        assert_eq!(result.is_none(), true);
    }
}
