use core::f32::consts::PI;
use crate::hex::HexTile;
use crate::mesh;
use crate::world;

pub enum Action {
    Push(Box<dyn GameState>),
    Pop,
    Continue,
    Quit,
}

pub trait GameState {
    fn enter(&self) {}
    fn update(&self, world: &mut world::World) -> Action;
    fn pause(&self) {}
    fn resume(&self) {}
    fn leave(&self) {}
}
