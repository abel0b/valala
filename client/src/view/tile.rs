use crate::store::{Action, State, Tile};
use valala_engine::{
    resource::ShaderId,
    scene::NodeId,
    store::Store,
    view::{Hoverable, Renderable, View, ViewBuilder},
};

pub struct TileEntity;

impl Hoverable<State> for TileEntity {
    fn hover_enter(&self, node: NodeId) -> Action {
        Action::HoverEnterTile(node)
    }
    fn hover_leave(&self, node: NodeId) -> Action {
        Action::HoverLeaveTile(node)
    }
}

impl Renderable<State> for TileEntity {
    fn render(&self, store: &Store<State>, node: NodeId) -> View {
        let mut view = ViewBuilder::from_node(node);
        let state = store
            .world
            .tiles
            .values()
            .find(|t| t.entity == node)
            .unwrap();
        let tile = view.geometry();
        let color = if state.hovered {
            (0.1, 0.8, 0.1, 1.0)
        } else if state.y == 0 {
            let a = (((state.q - state.r) % 3 + 3) % 3) as f32;
            (
                0.85 + a * 0.1,
                0.85 + a * 0.1,
                0.85 + a * 0.1,
                0.85 + a * 0.1,
            )
        } else {
            (0.5, 0.5, 0.5, 1.0)
        };
        tile.shader(ShaderId("color"));
        tile.vertex(
            (
                state.center.0,
                (state.y as f32) * Tile::HEIGHT,
                state.center.1,
            ),
            color,
            (0.5, 0.5),
        )
        .vertex(state.corners_up[0], color, (0.0, 0.5))
        .vertex(state.corners_up[1], color, (0.333_333, 0.0))
        .vertex(state.corners_up[2], color, (0.666_666, 0.0))
        .vertex(state.corners_up[3], color, (1.0, 0.5))
        .vertex(state.corners_up[4], color, (0.666_666, 1.0))
        .vertex(state.corners_up[5], color, (0.333_333, 1.0))
        .vertex(state.corners_down[0], color, (0.0, 0.5))
        .vertex(state.corners_down[1], color, (0.333_333, 0.0))
        .vertex(state.corners_down[2], color, (0.666_666, 0.0))
        .vertex(state.corners_down[3], color, (1.0, 0.5))
        .vertex(state.corners_down[4], color, (0.666_666, 1.0))
        .vertex(state.corners_down[5], color, (0.333_333, 1.0))
        .triangle(0, 1, 2)
        .triangle(0, 2, 3)
        .triangle(0, 3, 4)
        .triangle(0, 4, 5)
        .triangle(0, 5, 6)
        .triangle(0, 6, 1)
        .triangle(1, 7, 8)
        .triangle(1, 2, 8)
        .triangle(2, 8, 9)
        .triangle(2, 3, 9)
        .triangle(3, 9, 10)
        .triangle(3, 4, 10)
        .triangle(4, 10, 11)
        .triangle(4, 5, 11)
        .triangle(5, 11, 12)
        .triangle(5, 6, 12)
        .triangle(6, 12, 7)
        .triangle(6, 1, 7);

        let color = (0.2, 0.2, 0.2, 1.0);

        let border = view.geometry();
        border.shader(ShaderId("color"));
        border
            .vertex(
                (
                    state.center.0,
                    (state.y as f32) * Tile::HEIGHT,
                    state.center.1,
                ),
                color,
                (0.5, 0.5),
            )
            .vertex(state.corners_up[0], color, (0.0, 0.5))
            .vertex(state.corners_up[1], color, (0.333_333, 0.0))
            .vertex(state.corners_up[2], color, (0.666_666, 0.0))
            .vertex(state.corners_up[3], color, (1.0, 0.5))
            .vertex(state.corners_up[4], color, (0.666_666, 1.0))
            .vertex(state.corners_up[5], color, (0.333_333, 1.0))
            .vertex(state.corners_down[0], color, (0.0, 0.5))
            .vertex(state.corners_down[1], color, (0.333_333, 0.0))
            .vertex(state.corners_down[2], color, (0.666_666, 0.0))
            .vertex(state.corners_down[3], color, (1.0, 0.5))
            .vertex(state.corners_down[4], color, (0.666_666, 1.0))
            .vertex(state.corners_down[5], color, (0.333_333, 1.0))
            .line(1, 2)
            .line(2, 3)
            .line(3, 4)
            .line(4, 5)
            .line(5, 6)
            .line(6, 1)
            .line(7, 8)
            .line(8, 9)
            .line(10, 11)
            .line(11, 12)
            .line(12, 7)
            .line(1, 7)
            .line(2, 8)
            .line(3, 9)
            .line(4, 10)
            .line(5, 11)
            .line(6, 12);

        view.build()
    }
}
