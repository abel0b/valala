use valala_engine::prelude::{ResourcePack, Context, Settings};

pub fn build() -> Context {
    let settings = Settings::from_file("settings.ron");
    let resource_pack = ResourcePack::new();

    // resource_pack.load_texture(display, TextureId::Terrain, "terrain.png");
    //
    // resource_pack.load_shader(display, ShaderId::Line, "line");
    // resource_pack.load_shader(display, ShaderId::Path, "path");
    // resource_pack.load_shader(display, ShaderId::Picking, "picking");
    // resource_pack.load_shader(display, ShaderId::Character, "character");
    // resource_pack.load_shader(display, ShaderId::Terrain, "terrain");
    //
    // resource_pack.load_mesh(display, MeshId::Character, "character");

    Context::new(settings, resource_pack)
}
