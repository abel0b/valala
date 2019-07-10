use crate::{
    scene::Scene,
    context::Context,
};

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
}

impl GameStateMachine {
    pub fn new() -> GameStateMachine {
        GameStateMachine {
            states: Vec::new(),
        }
    }

    pub fn push(&mut self, ctx: &Context, scene: &mut Scene, mut gamestate: Box<dyn GameState>) {
        if let Some(prevstate) = self.states.last_mut() {
            prevstate.pause(ctx);
        }

        gamestate.enter(ctx, scene);
        self.states.push(gamestate);
    }

    pub fn pop(&mut self, ctx: &Context) {
        match self.states.pop() {
            Some(mut gamestate) => gamestate.leave(ctx),
            None => panic!("Empty state machine"),
        }
        if let Some(gamestate) = self.states.last_mut() {
            gamestate.resume(ctx);
        }
    }

    pub fn current(&mut self) -> Option<&mut Box<dyn GameState>> {
        self.states.last_mut()
    }
}
