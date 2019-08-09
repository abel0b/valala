use crate::{stage::Lobby, store::State};
use valala_engine::{
    color::Color,
    prelude::{Scene, Stage, Transition},
    store::Store,
};

pub struct Title;

impl Stage<State> for Title {
    fn enter(&mut self, _store: &mut Store<State>, scene: &mut Scene<State>) {
        scene.set_clear_color(Color::from_rgb(86, 64, 47));
    }

    fn frame(&mut self, _store: &mut Store<State>, _scene: &mut Scene<State>) -> Transition<State> {
        Transition::Push(Box::new(Lobby))
        // if store
        //     .context
        //     .clock
        //     .last_instant
        //     .duration_since(store.context.clock.initial_instant)
        //     .as_secs()
        //     > 3
        // {
        //     Transition::Push(Box::new(Lobby))
        // } else {
        //     Transition::Continue
        // }
    }
}
