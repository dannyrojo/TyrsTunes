pub struct State {
    pub selected_track: Option<usize>,
    

}

impl State {
    pub fn new() -> Self {
        State {
            selected_track: None,
        }
    }
}

pub fn initialize_state() -> (State, State) {
    let state_1 = State::new();
    let state_2 = State::new();
    (state_1, state_2)
}
