use valala_engine::{
    prelude::{Transition, Context, Stage, Scene},
    store::Store,
    color::Color,
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
        scene.set_clear_color(Color::from_rgb(86, 64, 47));
    }

    fn frame(&mut self, ctx: &Context, _scene: &mut Scene, store: &mut Store<State,Action>) -> Transition<State,Action> {
        if (ctx.clock.last_instant.duration_since(ctx.clock.initial_instant).as_secs() > 3) {
            Transition::Push(Box::new(Lobby))
        }
        else {
            Transition::Continue
        }
    }
}
