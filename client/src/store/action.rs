use valala_engine::scene::NodeId;

pub enum Action {
    EnterLobby,
    LoadRandomMap,
    MouseDownTile(NodeId),
    HoverEnterTile(NodeId),
    HoverLeaveTile(NodeId),
    Nop,
}
