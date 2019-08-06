use crate::{
    stage::Lobby,
    store::{Action, State},
};
use valala_engine::{
    color::Color,
    prelude::{Context, Scene, Stage, Transition},
    store::Store,
};

pub struct Title;

impl Stage<State, Action> for Title {
    fn enter(
        &mut self,
        _ctx: &Context,
        scene: &mut Scene<Action>,
        _store: &mut Store<State, Action>,
    ) {
        scene.set_clear_color(Color::from_rgb(86, 64, 47));
    }

    fn frame(
        &mut self,
        ctx: &Context,
        _scene: &mut Scene<Action>,
        _store: &mut Store<State, Action>,
    ) -> Transition<State, Action> {
        if ctx
            .clock
            .last_instant
            .duration_since(ctx.clock.initial_instant)
            .as_secs()
            > 3
        {
            Transition::Push(Box::new(Lobby))
        } else {
            Transition::Continue
        }
    }
}
