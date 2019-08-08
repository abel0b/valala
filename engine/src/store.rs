use crate::context::Context;
use crate::scene::Scene;

pub trait World
where
    Self: std::marker::Sized,
{
    type Action;
    fn apply<'a>(store: &mut Store<'a, Self>, scene: &mut Scene<Self>, action: Self::Action);
}

pub struct Store<'a, W: World> {
    pub world: W,
    pub context: Context<'a>,
}

impl<'a, W> Store<'a, W>
where
    W: World,
{
    pub fn new(context: Context<'a>, world: W) -> Store<'a, W> {
        Store { context, world }
    }

    pub fn dispatch(&mut self, scene: &mut Scene<W>, action: W::Action) {
        W::apply(self, scene, action);
    }
}
