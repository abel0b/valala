use crate::{
    scene::Scene,
    store::{Store, World},
};

pub enum Transition<W: World> {
    Push(Box<dyn Stage<W>>),
    Switch(Box<dyn Stage<W>>),
    Pop,
    Continue,
    Quit,
}

impl<W> Transition<W>
where
    W: World,
{
    pub fn is_continue(&self) -> bool {
        match self {
            Transition::Continue => true,
            _ => false,
        }
    }
}

pub trait Stage<W: World> {
    fn enter(&mut self, _store: &mut Store<W>, _scene: &mut Scene<W>) {}
    fn frame(&mut self, store: &mut Store<W>, scene: &mut Scene<W>) -> Transition<W>;
    fn pause(&mut self, _store: &mut Store<W>) {}
    fn resume(&mut self, _store: &mut Store<W>) {}
    fn leave(&mut self, _store: &mut Store<W>) {}
}

pub struct StageMachine<W: World> {
    states: Vec<Box<dyn Stage<W>>>,
    scenes: Vec<Scene<W>>,
}

impl<W> Default for StageMachine<W>
where
    W: World,
{
    fn default() -> StageMachine<W> {
        StageMachine {
            states: Vec::new(),
            scenes: Vec::new(),
        }
    }
}

impl<W> StageMachine<W>
where
    W: World,
{
    pub fn push(&mut self, store: &mut Store<W>, mut stage: Box<dyn Stage<W>>) {
        if let Some(prevstate) = self.states.last_mut() {
            prevstate.pause(store);
        }

        let mut scene = Scene::default();
        stage.enter(store, &mut scene);
        self.scenes.push(scene);
        self.states.push(stage);
    }

    pub fn pop(&mut self, store: &mut Store<W>) {
        match self.states.pop() {
            Some(mut stage) => stage.leave(store),
            None => panic!("Empty state machine"),
        }
        self.scenes.pop().unwrap();
        if let Some(stage) = self.states.last_mut() {
            stage.resume(store);
        }
    }

    pub fn current(&mut self) -> Option<&mut dyn Stage<W>> {
        match self.states.last_mut() {
            Some(state) => Some(state.as_mut()),
            None => None,
        }
    }

    pub fn scene(&mut self) -> &mut Scene<W> {
        self.scenes.last_mut().unwrap()
    }

    pub fn update(&mut self, store: &mut Store<W>) -> Transition<W> {
        match self.states.last_mut() {
            Some(stage) => stage.frame(store, self.scenes.last_mut().unwrap()),
            None => Transition::Quit,
        }
    }

    pub fn render(&mut self, store: &mut Store<W>) {
        self.scenes.last_mut().unwrap().render(store);
    }

    pub fn handle(&mut self, store: &mut Store<W>, event: &glium::glutin::Event) -> Transition<W> {
        self.scenes.last_mut().unwrap().handle(store, event)
    }
}
