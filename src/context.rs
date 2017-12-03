use glium::{self, Display, Frame, IndexBuffer, PolygonMode, Program, VertexBuffer};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::CompressedSrgbTexture2d;
use glium_text::{TextSystem, FontTexture};

use std::io::{BufRead, Seek, Read};

use errors::Error as AppError;
use font::FontSystem;
use ShaderHashMap;
use font::FontInstanceId;
use texture::{TextureId, TextureSystem};

/// No indices
pub const NO_INDICES_BUFFER: NoIndices = NoIndices(PrimitiveType::TrianglesList);
pub const PIXEL_TO_SCREEN_SHADER_ID: &str = "pixel_to_screen_shader";
pub const PIXEL_TO_SCREEN_VERT_SHADER_SOURCE: &str = include_str!("../shaders/pixel_to_screen_space.vert.glsl");
pub const PIXEL_TO_SCREEN_FRAG_SHADER_SOURCE: &str = include_str!("../shaders/pixel_to_screen_space.frag.glsl");

pub struct OpenGlContext
{
    /// The display of the renderer, currently OpenGL-based
    pub display: glium::backend::glutin_backend::GlutinFacade,
    // Shaders (tied to the current display)
    pub shader_programs: ShaderHashMap,
    // The text cache + text shaders (from glium_text)
    pub font_system: FontSystem,
    /// Textures
    pub texture_system: TextureSystem,
}

impl OpenGlContext {

    /// Creates a new window, compiles the shaders
    pub fn new(
        width: u32,
        height: u32,
    ) -> Result<Self, AppError>
    {
        use glium::DisplayBuild;
        use glium::glutin::{WindowBuilder, GlRequest};
        use glium::debug::DebugCallbackBehavior;
        use glium::Surface;
        use glium::glutin;

        let display = glutin::WindowBuilder::new()
            .with_gl(GlRequest::Latest)
            .with_vsync()
            .with_dimensions(width, height)
            .with_min_dimensions(600, 400)
            .with_srgb(None)
            .with_multisampling(4)
            .with_title(format!("{} version {}", ::assets::GAME_TITLE, env!("CARGO_PKG_VERSION")))
            .build_glium()
            .map_err(|_e| AppError { })?;

        let mut shader_programs = ShaderHashMap::default();
        shader_programs.insert(PIXEL_TO_SCREEN_SHADER_ID, Program::from_source(
            &display, PIXEL_TO_SCREEN_VERT_SHADER_SOURCE, PIXEL_TO_SCREEN_FRAG_SHADER_SOURCE, None
        ).unwrap());

        let texture_system = TextureSystem::new();
        let font_system = FontSystem::new(&display);

        Ok(Self {
            display: display,
            shader_programs: shader_programs,
            font_system: font_system,
            texture_system: texture_system,
        })
    }

    // load a font
    pub fn add_font<R>(&mut self, id: &'static str, size: u32, source: R)
        -> FontInstanceId where R: Read
    {
        self.font_system.add_font(id, size, source, &self.display)
    }

    pub fn add_texture_png<R>(&mut self, id: &'static str, source: R)
        -> TextureId
        where R: BufRead + Seek
    {
        self.texture_system.add_png_texture(id, source, &self.display)
    }
}
