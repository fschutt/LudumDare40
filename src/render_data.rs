//! Render data for one frame. This contains everything that should be rendered

use font::{FontInstanceId, Text};
use texture::TextureInstanceId;

/// All data that is needed to calculate one frame.
/// The texture system is based upon texture IDS
/// This struct is recreated on every frame
pub struct FrameRenderData<'a> {
    pub fonts: Vec<(FontInstanceId, Text<'a>)>,
    pub textures: Vec<TextureInstanceId>,
    /// Which screen to draw to (currently unused, for compositing screen effects)
    pub target_screen_texture: &'static str,
}
