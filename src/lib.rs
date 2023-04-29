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