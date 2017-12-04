use glium::{Program, Frame};
use audio::AudioContext;
use {TextureInstanceIdMap, FontInstanceIdMap};
use color::Color;
use font::FontInstanceId;
use texture::{TextureId, TextureInstanceId, TextureDrawOptions};
use context::OpenGlContext;
use font::Text;
use std::rc::Rc;
use glium::backend::Context;
use ShaderHashMap;

/// This does NOT represent just the screen, it
/// represents everything that can be done in 1/60th of a sencond.
/// All rendering functions are defined here.
pub struct GameFrame<'a> {
    pub frame: Frame,
    pub context: &'a OpenGlContext,
    pub font_ids: &'a FontInstanceIdMap,
    pub texture_ids: &'a TextureInstanceIdMap,
}

impl<'a> GameFrame<'a> {

    // clear the screen
    pub fn clear_screen(&mut self, color: Color) {
        use glium::Surface;
        self.frame.clear_color_and_depth(
            (color.r as f32 / 255.0,
             color.g as f32 / 255.0,
             color.b as f32 / 255.0,
             color.a as f32 / 255.0),
             1.0);
    }

    // notice: panics if the font isn't valid!!!
    pub fn get_font(&self, id: &'static str) -> FontInstanceId {
        let font_id = self.font_ids.get(id);
        *font_id.unwrap()
    }

    pub fn get_texture(&self, id: &'static str) -> TextureId {
        let texture_id = self.texture_ids.get(id);
        *texture_id.unwrap()
    }

    pub fn calculate_font_width(&self, id: &FontInstanceId, text: &str) -> f32 {
        self.context.font_system.calculate_font_width(id, text)
    }

    pub fn draw_font(&mut self, text: &Text, color: Color) {
        self.context.font_system.draw_font(&mut self.frame, text, color);
    }

    pub fn draw_texture(&mut self, display: &Rc<Context>, texture_id: &TextureInstanceId,
                        transparency: f32, shaders: &ShaderHashMap, options: TextureDrawOptions)
    {
        self.context.texture_system.draw_texture(&mut self.frame, display, texture_id, transparency, shaders, options);
    }

    pub fn drop(self) {
        self.frame.finish().unwrap();
    }
}
