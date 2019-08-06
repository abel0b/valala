mod character;
mod map;
mod state;
mod tile;

pub use character::Character;
pub use state::State;
pub use tile::Tile;
use valala_engine::store::Store;

pub enum Action {
    Nop,
    HoverEnterTile(i32, i32, i32),
}

fn reducer(_state: &mut State, action: Action) {
    match action {
        Action::HoverEnterTile(q, r, y) => {
            println!("tile {:?}", (q, r, y));
        }
        Action::Nop => {}
    }
}

pub fn create() -> Store<State, Action> {
    let state = State::new();
    Store::new(state, reducer)
}
