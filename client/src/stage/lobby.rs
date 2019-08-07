use crate::store::{Action, State};

use valala_engine::{
    prelude::{Scene, Stage, Transition},
    store::Store,
};

pub struct Lobby;

impl Stage<State, Action> for Lobby {
    fn enter(&mut self, store: &mut Store<State, Action>, scene: &mut Scene<State, Action>) {
        store.dispatch(scene, Action::EnterLobby);
        store.dispatch(scene, Action::LoadRandomMap);
    }

    fn frame(
        &mut self,
        store: &mut Store<State, Action>,
        scene: &mut Scene<State, Action>,
    ) -> Transition<State, Action> {
        store.dispatch(scene, Action::Nop);
        Transition::Continue
    }
}
