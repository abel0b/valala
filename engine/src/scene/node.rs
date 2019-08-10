#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Uid(pub u32);

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum NodeIndex {
    Root,
    Camera(Uid),
    Entity(Uid),
    Group(Uid),
}

#[allow(dead_code)]
pub struct Node {
    pub dirty: bool,
    pub visible: bool,
    pub index: NodeIndex,
    pub parent: Option<NodeIndex>,
    pub children: Vec<NodeIndex>,
}

impl Node {
    pub fn with_index_and_parent(index: NodeIndex, parent: Option<NodeIndex>) -> Node {
        Node {
            parent,
            dirty: true,
            visible: true,
            index,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node_id: NodeIndex) {
        self.children.push(node_id)
    }
}
