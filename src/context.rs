use glium::{self, Display, Frame, IndexBuffer, PolygonMode, Program, VertexBuffer};
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::CompressedSrgbTexture2d;
use glium_text::{TextSystem, FontTexture};

use std::io::Read;

use errors::Error as AppError;
use font::FontSystem;
use FastHashMap;
use font::FontInstanceId;
use texture::TextureSystem;

/// No indices
const NO_INDICES_BUFFER: NoIndices = NoIndices(PrimitiveType::TrianglesList);

pub struct OpenGlContext
{
    /// The display of the renderer, currently OpenGL-based
    pub display: glium::backend::glutin_backend::GlutinFacade,
    // Shaders (tied to the current display)
    pub shader_programs: FastHashMap<&'static str, Program>,
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
            .with_title(format!("StackBoxes version {}", env!("CARGO_PKG_VERSION")))
            .build_glium()
            .map_err(|_e| AppError { })?;

        let shader_programs = FastHashMap::<&'static str, Program>::default();
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
}
