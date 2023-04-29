mod tictactoe;

// fn minmax<A: Action>(game: Box<dyn Game<A>>) -> i8 {
//     if game.is_terminal() {
//         return game.value();
//     }
//
//     let actions = game.get_actions();
//     let values = actions.iter().map(|action| {
//         minmax(game.apply_action(action).unwrap())
//     }).collect::<Vec<i8>>();
//
//     return if game.is_min_player() {
//         values.iter().min().unwrap().clone()
//     } else {
//         values.iter().max().unwrap().clone()
//     }
// }
