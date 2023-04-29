use crate::Player::{O, X};

trait Action {}

trait Game<A: Action> {
    /// Returns true if current game state is a terminal state
    fn is_terminal(&self) -> bool;
    /// Returns all possible actions from the current game state
    fn get_actions(&self) -> Vec<A>;
    /// Applies an action to the current state, returns a new state
    fn apply_action(&self, action: A) -> Result<Self, String>
    where
        Self: Sized;
    /// Get the value of the current state, must be terminal
    fn value(&self) -> i8;
    /// Returns true if current player is a min player
    /// Returns false if current player is a max player
    /// min players try to minimize the value output
    /// max players try to maximze the value output
    fn is_min_player(&self) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn invert(&self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
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

impl Game<TicTacToeAction> for TicTacToe {
    fn is_terminal(&self) -> bool {
        self.get_winner().is_ok()
    }

    fn get_actions(&self) -> Vec<TicTacToeAction> {
        if self.is_terminal() {
            // cannot perform further actions once we have reached the terminal state
            return Vec::new();
        }

        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, player_move)| match player_move {
                None => Some(i as u8),
                _ => None,
            })
            .collect()
    }

    // TODO: make more efficient
    //  there is no need to create a new struct for every state transition
    //  should refactor so we only duplicate data and struct just contains
    //  data transformation functions
    fn apply_action(&self, action: TicTacToeAction) -> Result<Self, String> {
        if self.is_terminal() {
            return Err(String::from("game already reached terminal state"));
        }

        if action < 0 || action > 8 {
            return Err(String::from(
                "action can only be a number between 0 and 8 inclusive",
            ));
        }

        if self.board[action as usize] != None {
            return Err(String::from("move already made at given position"));
        }

        let mut new_board_state = self.board;
        new_board_state[action as usize] = Some(self.player);

        Ok(TicTacToe::new_from_state(
            new_board_state,
            self.player.invert(),
        ))
    }

    fn value(&self) -> i8 {
        // assumes that the current state is terminal
        let winner = self.get_winner().unwrap();
        match winner {
            Some(Player::X) => 1,
            Some(Player::O) => -1,
            None => 0,
        }
    }

    fn is_min_player(&self) -> bool {
        match self.player {
            Player::X => false,
            Player::O => true
        }
    }
}

// TODO: implement to u8
type TicTacToeAction = u8;
impl Action for TicTacToeAction {}

#[cfg(test)]
mod tests {
    use crate::Player::{O, X};
    use crate::{Game, Player, TicTacToe};

    #[test]
    fn test_get_winner() {
        let game = TicTacToe::new();
        assert_eq!(game.is_terminal(), false);
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
        assert_eq!(game.is_terminal(), true);
        assert_eq!(game.value(), 1);
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
        assert_eq!(game.is_terminal(), true);
        assert_eq!(game.value(), -1);
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
        assert_eq!(game.is_terminal(), true);
        assert_eq!(game.value(), 0);
        let result = game.get_winner().expect("should be terminal");
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn test_get_actions() {
        // from empty board, all actions should be possible
        let game = TicTacToe::new();
        let actions = game.get_actions();
        assert_eq!(actions.len(), 9);
        assert_eq!(actions, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // non terminal and non draw
        let game = TicTacToe::new_from_state(
            [
                Some(X),
                Some(X),
                None,
                Some(O),
                Some(O),
                None,
                None,
                None,
                None,
            ],
            Player::X,
        );
        let actions = game.get_actions();
        assert_eq!(actions.len(), 5);
        assert_eq!(actions, [2, 5, 6, 7, 8]);

        // action set should be empty if terminal state
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
        let actions = game.get_actions();
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_apply_action() {
        let game = TicTacToe::new();
        let game = game.apply_action(0).unwrap();
        assert_eq!(
            game.board,
            [Some(X), None, None, None, None, None, None, None, None]
        );
        let game = game.apply_action(3).unwrap();
        assert_eq!(
            game.board,
            [Some(X), None, None, Some(O), None, None, None, None, None]
        );
        let game = game.apply_action(1).unwrap();
        assert_eq!(
            game.board,
            [
                Some(X),
                Some(X),
                None,
                Some(O),
                None,
                None,
                None,
                None,
                None
            ]
        );
        let game = game.apply_action(4).unwrap();
        assert_eq!(
            game.board,
            [
                Some(X),
                Some(X),
                None,
                Some(O),
                Some(O),
                None,
                None,
                None,
                None
            ]
        );

        // try to play a repeated move
        assert_eq!(game.apply_action(4).is_err(), true);

        // finish the game
        let game = game.apply_action(2).unwrap();
        assert_eq!(
            game.board,
            [
                Some(X),
                Some(X),
                Some(X),
                Some(O),
                Some(O),
                None,
                None,
                None,
                None
            ]
        );
        assert_eq!(game.is_terminal(), true);

        // try to play after terminal
        assert_eq!(game.apply_action(8).is_err(), true);
    }
}

fn minmax<A: Action>(game: Box<dyn Game<A>>) -> i8 {
    if game.is_terminal() {
        return game.value();
    }

    let actions = game.get_actions();
    let values = actions.iter().map(|action| {
        minmax(game.apply_action(action).unwrap())
    }).collect::<Vec<i8>>();

    return if game.is_min_player() {
        values.iter().min().unwrap().clone()
    } else {
        values.iter().max().unwrap().clone()
    }
}
