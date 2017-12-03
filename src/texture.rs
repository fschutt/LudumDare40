use FastHashMap;
use std::io::{BufRead, Seek};
use glium::texture::Texture2d;
use glium::backend::Facade;
use glium::texture::RawImage2d;

#[derive(Default)]
pub struct TextureSystem {
    // Images used by the renderer
    pub textures: FastHashMap<&'static str, Texture2d>,
}

/// Width, height and offsets into the texture
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourcePixelRegion {
    pub top_x: u32,
    pub top_y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextureRegion {
    /// Texture ID for looking it up in the TextureSystem at runtime
    pub texture_id: &'static str,
    /// Region of the texture that should be drawn (i.e.)
    pub region: SourcePixelRegion,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextureInstanceId {
    pub source_texture_region: TextureRegion,
    /// X and Y where the TextureRegion should be drawn on the screen
    pub screen_x: u32,
    pub screen_y: u32,
}

impl TextureSystem {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_png_texture<R, F>(&mut self, id: &'static str, source: R, display: &F)
        where R: BufRead + Seek, F: Facade
    {
        use image;
        let image = image::load(source, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = Texture2d::new(display, image).unwrap();

        self.textures.insert(id, opengl_texture);
    }
}
