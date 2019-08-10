use valala_engine::scene::Uid;

pub enum Action {
    EnterLobby,
    LoadRandomMap,
    MouseDownTile(Uid),
    HoverEnterTile(Uid),
    HoverLeaveTile(Uid),
    Nop,
}
