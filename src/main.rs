#![macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;
extern crate clap;
extern crate rand;

mod map;
mod hex;
mod game;
mod camera;
mod resource;
mod state;
mod vertex;
mod picking;
mod identifier;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;

fn main() -> Result<(), Box<dyn Error>> {
    clap::App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let mut game = game::Game::new()?;
    game.start();
    Ok(())
}
