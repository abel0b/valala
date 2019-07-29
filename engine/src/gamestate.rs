use crate::{context::Context, scene::Scene};

pub enum Action {
    Push(Box<dyn GameState>),
    Switch(Box<dyn GameState>),
    Pop,
    Continue,
    Quit,
}

pub trait GameState {
    fn enter(&mut self, _ctx: &Context, _scene: &mut Scene) {}
    fn frame(&mut self, ctx: &Context, scene: &mut Scene) -> Action;
    fn pause(&mut self, _ctx: &Context) {}
    fn resume(&mut self, _ctx: &Context) {}
    fn leave(&mut self, _ctx: &Context) {}
}

pub struct GameStateMachine {
    states: Vec<Box<dyn GameState>>,
    scenes: Vec<Scene>,
}

impl Default for GameStateMachine {
    fn default() -> GameStateMachine {
        GameStateMachine {
            states: Vec::new(),
            scenes: Vec::new(),
        }
    }
}

impl GameStateMachine {
    pub fn push(&mut self, ctx: &Context, mut gamestate: Box<dyn GameState>) {
        if let Some(prevstate) = self.states.last_mut() {
            prevstate.pause(ctx);
        }

        let mut scene = Scene::default();
        gamestate.enter(ctx, &mut scene);
        self.scenes.push(scene);
        self.states.push(gamestate);
    }

    pub fn pop(&mut self, ctx: &Context) {
        match self.states.pop() {
            Some(mut gamestate) => gamestate.leave(ctx),
            None => panic!("Empty state machine"),
        }
        self.scenes.pop().unwrap();
        if let Some(gamestate) = self.states.last_mut() {
            gamestate.resume(ctx);
        }
    }

    pub fn current(&mut self) -> Option<&mut dyn GameState> {
        match self.states.last_mut() {
            Some(state) => Some(state.as_mut()),
            None => None,
        }
    }

    pub fn scene(&mut self) -> &mut Scene {
        self.scenes.last_mut().unwrap()
    }

    pub fn update(&mut self, context: &Context) -> Action {
        match self.states.last_mut() {
            Some(gamestate) => gamestate.frame(context, self.scenes.last_mut().unwrap()),
            None => Action::Quit,
        }
    }

    pub fn render(&mut self, context: &mut Context) {
        self.scenes.last_mut().unwrap().render(context);
    }
}
