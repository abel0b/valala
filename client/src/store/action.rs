use valala_engine::scene::NodeId;

pub enum Action {
    EnterLobby,
    LoadRandomMap,
    HoverEnterTile(NodeId),
    HoverLeaveTile(NodeId),
    Nop,
}
