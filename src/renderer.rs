use input::WindowState;
use errors::Error as AppError;
use context::OpenGlContext;

pub struct Renderer
{
    pub context: OpenGlContext,
    pub window_state: WindowState,
}

impl Renderer {

    /// Creates a new renderer. Does not add any fonts or textures
    pub fn new(width: u32, height: u32) -> Result<Self, AppError> {
        Ok(Self {
            context: OpenGlContext::new(width, height)?,
            window_state: WindowState::new(width, height),
        })
    }
}
