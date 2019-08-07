use crate::{context::Context, scene::Scene, store::Store};

pub enum Transition<S, A> {
    Push(Box<dyn Stage<S, A>>),
    Switch(Box<dyn Stage<S, A>>),
    Pop,
    Continue,
    Quit,
}

pub trait Stage<S, A> {
    fn enter(&mut self, _store: &mut Store<S, A>, _scene: &mut Scene<S, A>) {}
    fn frame(&mut self, store: &mut Store<S, A>, scene: &mut Scene<S, A>) -> Transition<S, A>;
    fn pause(&mut self, _store: &mut Store<S, A>) {}
    fn resume(&mut self, _store: &mut Store<S, A>) {}
    fn leave(&mut self, _store: &mut Store<S, A>) {}
}

pub struct StageMachine<S, A> {
    states: Vec<Box<dyn Stage<S, A>>>,
    scenes: Vec<Scene<S, A>>,
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
    pub fn push(&mut self, store: &mut Store<S, A>, mut stage: Box<dyn Stage<S, A>>) {
        if let Some(prevstate) = self.states.last_mut() {
            prevstate.pause(store);
        }

        let mut scene = Scene::default();
        stage.enter(store, &mut scene);
        self.scenes.push(scene);
        self.states.push(stage);
    }

    pub fn pop(&mut self, store: &mut Store<S, A>) {
        match self.states.pop() {
            Some(mut stage) => stage.leave(store),
            None => panic!("Empty state machine"),
        }
        self.scenes.pop().unwrap();
        if let Some(stage) = self.states.last_mut() {
            stage.resume(store);
        }
    }

    pub fn current(&mut self) -> Option<&mut dyn Stage<S, A>> {
        match self.states.last_mut() {
            Some(state) => Some(state.as_mut()),
            None => None,
        }
    }

    pub fn scene(&mut self) -> &mut Scene<S, A> {
        self.scenes.last_mut().unwrap()
    }

    pub fn update(&mut self, store: &mut Store<S, A>) -> Transition<S, A> {
        match self.states.last_mut() {
            Some(stage) => stage.frame(store, self.scenes.last_mut().unwrap()),
            None => Transition::Quit,
        }
    }

    pub fn render(&mut self, store: &mut Store<S, A>) {
        self.scenes.last_mut().unwrap().render(store);
    }
}
