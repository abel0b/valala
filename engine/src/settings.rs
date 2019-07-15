use ron;
use serde::Deserialize;
use std::fs::File;

// TODO: implement Default trait

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub graphics: Graphics,
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Graphics {
    pub window_width: u32,
    pub window_height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: std::string::String,
    pub port: u16,
}

impl Settings {
    pub fn from_file(filename: &str) -> Settings {
        let f = File::open(filename).expect("Failed opening settings file");
        match ron::de::from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load settings: {}", e);
                std::process::exit(1);
            }
        }
    }
}
