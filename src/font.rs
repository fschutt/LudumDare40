//! Font data that needs to be rendered

use glium_text::{self, TextSystem, FontTexture, TextDisplay};
use glium::backend::Facade;
use glium::Frame;

use std::io::Read;

use color::Color;
use FastHashMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FontInstanceId {
    pub font_name: &'static str,
    pub font_size: u32,
}

/// Contains the fonts, rendered at startup time
pub struct FontSystem {
    pub text_system: TextSystem,
    // font instance, hashed by source (which font it is) + size (how big the font was rendered)
    pub fonts: FastHashMap<FontInstanceId, FontTexture>,
}

impl FontSystem {
    pub fn new<F>(display: &F) -> Self where F: Facade  {
        Self {
            // The `TextSystem` contains the shaders and elements used for text display
            text_system: TextSystem::new(display),
            fonts: FastHashMap::<FontInstanceId, FontTexture>::default(),
        }
    }

    pub fn add_font<R, F>(&mut self, id: &'static str, size: u32, source: R, display: &F)
        -> FontInstanceId where R: Read, F: Facade
    {
        let id = FontInstanceId {
            font_name: id,
            font_size: size,
        };

        let actual_font = FontTexture::new(display, source, size).unwrap(); // todo: remove unwrap
        self.fonts.insert(id.clone(), actual_font);
        id
    }

    pub fn calculate_font_width(&self, id: &FontInstanceId, text: &str) -> f32 {
        let font_height = id.font_size;
        let glium_font = self.fonts.get(id).unwrap();
        let glium_text = glium_text::TextDisplay::new(&self.text_system, glium_font, text);
        glium_text.get_width() * (font_height as f32)
    }

    // draw the text to the screen
    pub fn draw_font(&self, frame: &mut Frame, text: &Text, color: Color) {
        use glium::Surface;

        let glium_font = self.fonts.get(&text.font).unwrap();
        let glium_text = glium_text::TextDisplay::new(&self.text_system, glium_font, text.text);

        let (w, h) = frame.get_dimensions();
        let font_size = text.font.font_size;
        let scale_factor = font_size as f32 / w as f32 * 2.0;
        let pos_x = (text.screen_x as f32 / w as f32 * 2.0) - 1.0;
        let pos_y = (text.screen_y as f32 / h as f32 * 2.0) - 1.0;

        let matrix: [[f32; 4]; 4] =
        [
            [scale_factor, 0.0, 0.0, 0.0],
            [0.0, scale_factor * (w as f32) / (h as f32), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [pos_x, pos_y, 0.0, 1.0f32],
        ];

        let r = color.r as f32 / 255.0;
        let g = color.g as f32 / 255.0;
        let b = color.b as f32 / 255.0;
        let a = color.a as f32 / 255.0;

        glium_text::draw(&glium_text, &self.text_system, frame, matrix, (r, g, b, a));
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Text<'a> {
    pub font: &'a FontInstanceId,
    /// The text that should be displayed
    pub text: &'a str,
    /// X and Y position of the text
    pub screen_x: u32,
    pub screen_y: u32,
}

impl<'a> Text<'a> {
    pub fn new(font: &'a FontInstanceId, text: &'a str, x: u32, y: u32) -> Self {
        Self {
            font: font,
            text: text,
            screen_x: x,
            screen_y: y,
        }
    }
}
