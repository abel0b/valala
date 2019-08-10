use crate::scene::Uid;
use crate::store::{Store, World};
use crate::view::View;

pub struct Entity<W: World> {
    pub render: Option<fn(store: &Store<W>, id: Uid) -> View>,
    pub on_mouse_up: Option<fn(id: Uid) -> W::Action>,
    pub on_mouse_down: Option<fn(id: Uid) -> W::Action>,
    pub on_hover_enter: Option<fn(id: Uid) -> W::Action>,
    pub on_hover_leave: Option<fn(id: Uid) -> W::Action>,
}
