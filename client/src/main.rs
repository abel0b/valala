mod hex;
mod data;
mod view;
mod lobby;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use clap::App;
use valala_engine::prelude::{
    Engine,
    Settings,
    ResourcePack,
    Context,
};

use crate::{
    lobby::Lobby,
};

fn main() -> Result<(), Box<dyn Error>> {
    App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let mut engine = Engine::new({
        let settings = Settings::from_file("settings.ron");
        let resource_pack = ResourcePack::new();

        Context::new(settings, resource_pack)
    })?;

    engine.run(Box::new(Lobby));

    Ok(())
}
