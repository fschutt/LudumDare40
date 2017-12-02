use glium::{self, Display, Frame, IndexBuffer, PolygonMode, Program, VertexBuffer};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::CompressedSrgbTexture2d;
use twox_hash::XxHash;

use std::hash::BuildHasherDefault;

use errors::Error as AppError;
use std::collections::HashMap;

/// No indices
const NO_INDICES_BUFFER: NoIndices = NoIndices(PrimitiveType::TrianglesList);

pub struct OpenGlContext
{
    /// The display of the renderer, currently OpenGL-based
    pub display: Display,
    // Shaders (tied to the current display)
    // pub shader_programs: HashMap<&'static str, Program, BuildHasherDefault<XxHash>>,
    // Images used by the renderer
    // pub images: HashMap<&'static str, CompressedSrgbTexture2d, BuildHasherDefault<XxHash>>,
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

        let display = WindowBuilder::new()
            .with_gl(GlRequest::GlThenGles {
                opengl_version: (3, 1),
                opengles_version: (3, 0),
            })
            .with_vsync()
            .with_dimensions(width, height)
            .with_min_dimensions(600, 400)
            .with_srgb(Some(true))
            .with_multisampling(4)
            .with_title(format!("StackBoxes version {}", env!("CARGO_PKG_VERSION")))
            .build_glium_debug(glium::debug::DebugCallbackBehavior::DebugMessageOnError)
            .map_err(|_e| AppError { })?;

        // let shader_programs = HashMap<String, Program, BuildHasherDefault<XxHash>>::new();
        // let images = HashMap<String, CompressedSrgbTexture2d, BuildHasherDefault<XxHash>>::new();

        Ok(Self {
            display: display,
            // shader_programs: shader_programs,
            // images: images,
        })
    }
}
