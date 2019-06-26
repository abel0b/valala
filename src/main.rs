#![macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;

mod map;
mod hex;
mod game;
mod camera;
mod resource;
mod state;
mod vertex;
mod picking;
mod identifier;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Valala v{}", env!("CARGO_PKG_VERSION"));
    let mut game = game::Game::new()?;
    game.start();
    Ok(())
}
