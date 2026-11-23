use crate::vec2::Vec2;

/// What the user is doing with the mouse button held down.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Idle,
    /// Drawing a new shape, from the point where the drag started.
    Drawing {
        start: Vec2,
    },
    /// Moving the shape at `index`; `last` is the mouse position at the previous frame.
    Moving {
        index: usize,
        last: Vec2,
    },
    PickingHue,
    PickingSatVal,
}
