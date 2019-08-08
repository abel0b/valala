use crate::store::{Action, State};

use valala_engine::{
    prelude::{Scene, Stage, Transition},
    store::Store,
};

pub struct Lobby;

impl Stage<State> for Lobby {
    fn enter(&mut self, store: &mut Store<State>, scene: &mut Scene<State>) {
        store.dispatch(scene, Action::EnterLobby);
        store.dispatch(scene, Action::LoadRandomMap);
    }

    fn frame(&mut self, store: &mut Store<State>, scene: &mut Scene<State>) -> Transition<State> {
        store.dispatch(scene, Action::Nop);
        Transition::Continue
    }
}
