use crate::context::Context;
use crate::scene::Scene;

pub struct Store<'a, S, A> {
    pub context: Context<'a>,
    pub state: S,
    reducer: fn(&mut Store<'a, S, A>, &mut Scene<S, A>, A),
}

impl<'a, S, A> Store<'a, S, A> {
    pub fn new(
        context: Context<'a>,
        state: S,
        reducer: fn(&mut Store<'a, S, A>, &mut Scene<S, A>, A),
    ) -> Store<'a, S, A> {
        Store {
            context,
            state,
            reducer,
        }
    }

    pub fn dispatch(&mut self, scene: &mut Scene<S, A>, action: A) {
        (self.reducer)(self, scene, action);
    }
}
