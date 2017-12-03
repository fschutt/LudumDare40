use glium::Frame;
use audio::AudioContext;
use {TextureInstanceIdMap, FontInstanceIdMap};
use color::Color;
use font::FontInstanceId;
use texture::TextureInstanceId;
use context::OpenGlContext;
use font::Text;

/// This does NOT represent just the screen, it
/// represents everything that can be done in 1/60th of a sencond.
/// All rendering functions are defined here.
pub struct GameFrame<'a> {
    pub frame: Frame,
    pub context: &'a OpenGlContext,
    pub audio_context: &'a AudioContext,
    pub font_ids: &'a FontInstanceIdMap,
    pub texture_ids: &'a TextureInstanceIdMap,
}

impl<'a> GameFrame<'a> {

    // clear the screen
    pub fn clear_screen(&mut self, color: Color) {
        use glium::Surface;
        self.frame.clear_color(color.r as f32 / 255.0,
                               color.g as f32 / 255.0,
                               color.b as f32 / 255.0,
                               color.a as f32 / 255.0);
    }

    // notice: panics if the font isn't valid!!!
    pub fn get_font(&self, id: &'static str) -> FontInstanceId {
        let font_id = self.font_ids.get(id);
        *font_id.unwrap()
    }

    pub fn get_texture(&self, id: &'static str) -> TextureInstanceId {
        let texture_id = self.texture_ids.get(id);
        *texture_id.unwrap()
    }

    pub fn draw_font(&mut self, text: &Text, color: Color) {
        self.context.font_system.draw_font(&mut self.frame, text, color);
    }

    pub fn drop(self) {
        self.frame.finish().unwrap();
    }
}
