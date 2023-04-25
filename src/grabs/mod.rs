use smithay::utils::{Logical, Rectangle};

use self::resize_grab::ResizeState;

pub mod move_grab;
pub mod resize_grab;

#[derive(Default)]
pub struct SurfaceData {
    pub geometry: Option<Rectangle<i32, Logical>>,
    pub resize_state: ResizeState,
}
