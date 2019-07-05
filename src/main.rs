#![macro_use]
extern crate glium;
extern crate image;
extern crate cgmath;
extern crate clap;
extern crate rand;
extern crate tobj;
extern crate ron;
extern crate serde;

mod map;
mod hex;
mod engine;
mod camera;
mod resource;
mod gamestate;
mod mesh;
mod picking;
mod ui;
mod world;
mod entity;
mod scene;
mod character;
mod settings;
mod lobby;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;

fn main() -> Result<(), Box<dyn Error>> {
    clap::App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    engine::Engine::new()?.run();
    Ok(())
}
