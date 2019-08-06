use crate::{context::Context, scene::Scene, store::Store};

pub enum Transition<S, A> {
    Push(Box<dyn Stage<S, A>>),
    Switch(Box<dyn Stage<S, A>>),
    Pop,
    Continue,
    Quit,
}

pub trait Stage<S, A> {
    fn enter(&mut self, _ctx: &Context, _scene: &mut Scene<A>, _store: &mut Store<S, A>) {}
    fn frame(
        &mut self,
        ctx: &Context,
        scene: &mut Scene<A>,
        store: &mut Store<S, A>,
    ) -> Transition<S, A>;
    fn pause(&mut self, _ctx: &Context) {}
    fn resume(&mut self, _ctx: &Context) {}
    fn leave(&mut self, _ctx: &Context) {}
}

pub struct StageMachine<S, A> {
    states: Vec<Box<dyn Stage<S, A>>>,
    scenes: Vec<Scene<A>>,
}

impl<S, A> Default for StageMachine<S, A> {
    fn default() -> StageMachine<S, A> {
        StageMachine {
            states: Vec::new(),
            scenes: Vec::new(),
        }
    }
}

impl<S, A> StageMachine<S, A> {
    pub fn push(
        &mut self,
        ctx: &Context,
        mut stage: Box<dyn Stage<S, A>>,
        store: &mut Store<S, A>,
    ) {
        if let Some(prevstate) = self.states.last_mut() {
            prevstate.pause(ctx);
        }

        let mut scene = Scene::default();
        stage.enter(ctx, &mut scene, store);
        self.scenes.push(scene);
        self.states.push(stage);
    }

    pub fn pop(&mut self, ctx: &Context) {
        match self.states.pop() {
            Some(mut stage) => stage.leave(ctx),
            None => panic!("Empty state machine"),
        }
        self.scenes.pop().unwrap();
        if let Some(stage) = self.states.last_mut() {
            stage.resume(ctx);
        }
    }

    pub fn current(&mut self) -> Option<&mut dyn Stage<S, A>> {
        match self.states.last_mut() {
            Some(state) => Some(state.as_mut()),
            None => None,
        }
    }

    pub fn scene(&mut self) -> &mut Scene<A> {
        self.scenes.last_mut().unwrap()
    }

    pub fn update(&mut self, context: &Context, store: &mut Store<S, A>) -> Transition<S, A> {
        match self.states.last_mut() {
            Some(stage) => stage.frame(context, self.scenes.last_mut().unwrap(), store),
            None => Transition::Quit,
        }
    }

    pub fn render(&mut self, context: &mut Context, store: &mut Store<S, A>) {
        self.scenes.last_mut().unwrap().render(context, store);
    }
}
