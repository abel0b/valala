mod map;
mod hex;
mod character;
mod lobby;
mod context;

use std::error::Error;
use std::result::Result;
use std::boxed::Box;
use clap::App;
use valala_engine::prelude::{
    Engine,
    Settings,
};

use crate::{
    lobby::Lobby,
};

fn main() -> Result<(), Box<dyn Error>> {
    App::new("Valala")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    let mut engine = Engine::new(
        context::build()
    )?;
    let lobby: Lobby = Default::default();

    engine.run(Box::new(lobby));

    Ok(())
}
