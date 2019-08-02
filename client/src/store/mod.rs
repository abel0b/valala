mod map;

pub use map::Map;
use valala_engine::store::Store;

pub struct State {

}

impl Default for State {
    fn default() -> State {
        State {

        }
    }
}

impl State {
    pub fn new() -> State {
        Default::default()
    }
}

pub enum Action {
    Oof,
    CharacterEntered(u32),
}

fn reducer(state: &mut State, action: Action) {
    match action {
        Oof => {
            println!("oof");
        },
        _ => {}
    }
}

pub fn create() -> Store<State, Action> {
    let state = State::new();
    Store::new(state, reducer)
}
