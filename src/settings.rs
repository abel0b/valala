use serde::Deserialize;

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
