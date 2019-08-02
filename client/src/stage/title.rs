use valala_engine::{
    prelude::{Transition, Context, Stage, Scene},
    store::Store,
};
use crate::{
    stage::Lobby,
    view::{
        Character,
        Tile,
    },
    store::{State, Action},
};

pub struct Title;

impl Stage<State, Action> for Title {
    fn enter(&mut self, ctx: &Context, scene: &mut Scene) {

    }

    fn frame(&mut self, ctx: &Context, _scene: &mut Scene, store: &mut Store<State,Action>) -> Transition<State,Action> {
        if (ctx.clock.last_instant.duration_since(ctx.clock.initial_instant).as_secs() > 5) {
            Transition::Push(Box::new(Lobby))
        }
        else {
            Transition::Continue
        }
    }
}
