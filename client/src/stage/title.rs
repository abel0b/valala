use crate::{
    stage::Lobby,
    store::{Action, State},
};
use valala_engine::{
    color::Color,
    prelude::{Scene, Stage, Transition},
    store::Store,
};

pub struct Title;

impl Stage<State, Action> for Title {
    fn enter(&mut self, _store: &mut Store<State, Action>, scene: &mut Scene<State, Action>) {
        scene.set_clear_color(Color::from_rgb(86, 64, 47));
    }

    fn frame(
        &mut self,
        store: &mut Store<State, Action>,
        _scene: &mut Scene<State, Action>,
    ) -> Transition<State, Action> {
        if store
            .context
            .clock
            .last_instant
            .duration_since(store.context.clock.initial_instant)
            .as_secs()
            > 3
        {
            Transition::Push(Box::new(Lobby))
        } else {
            Transition::Continue
        }
    }
}
