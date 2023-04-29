mod minmax_strategy;
mod random_strategy;
mod tictactoe;

use crate::tictactoe::{draw_board, Player, TicTacToe, TicTacToeAction};
use minmax_strategy::minmax_strategy;
use random_strategy::random_strategy;

struct PlayTicTacToe<T, U>
where
    T: Fn(&TicTacToe) -> TicTacToeAction,
    U: Fn(&TicTacToe) -> TicTacToeAction,
{
    instance: TicTacToe,
    x_strategy: T,
    o_strategy: U,
    current_player: Player,
}

impl<T, U> PlayTicTacToe<T, U>
where
    T: Fn(&TicTacToe) -> TicTacToeAction,
    U: Fn(&TicTacToe) -> TicTacToeAction,
{
    fn init(x_strategy: T, o_strategy: U) -> Self {
        let game = TicTacToe::new();
        let current_player = game.player;

        Self {
            instance: game,
            current_player,
            x_strategy,
            o_strategy,
        }
    }

    fn play(x_strategy: T, o_strategy: U) {
        let mut game = Self::init(x_strategy, o_strategy);

        while game.instance.is_terminal() == false {
            let action = match game.current_player {
                Player::X => (game.x_strategy)(&game.instance),
                Player::O => (game.o_strategy)(&game.instance),
            };

            let new_board_state = game.instance.apply_action(action).unwrap();
            println!("{}", draw_board(&new_board_state));
            game.instance = new_board_state;
            game.current_player = game.instance.player;
        }

        println!("{}", game.instance.value());
    }
}

fn main() {
    PlayTicTacToe::play(minmax_strategy, random_strategy);
}
