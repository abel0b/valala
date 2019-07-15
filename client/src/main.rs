#![cfg_attr(feature = "strict", deny(warnings))]

mod data;
mod hex;
mod lobby;
mod view;

use clap::App;
use std::boxed::Box;
use std::error::Error;
use std::result::Result;
use valala_engine::prelude::{Context, Engine, ResourcePack, Settings};

use crate::lobby::Lobby;

fn main() -> Result<(), Box<dyn Error>> {
    App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let mut engine = Engine::new({
        let settings = Settings::from_file("settings.ron");
        let resource_pack = ResourcePack::default();
        let mut context = Context::new(settings, resource_pack);

        context.load_texture("stone", "stone.png");
        context.load_texture("grass", "grass.png");
        context.load_texture("dirt", "dirt.png");
        context.load_texture("water", "water.png");

        context.load_shader("map", "map");

        context
    })?;

    engine.run(Box::new(Lobby));

    Ok(())
}
