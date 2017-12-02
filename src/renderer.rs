use input::WindowState;
use errors::Error as AppError;
use context::OpenGlContext;

pub struct Renderer
{
    pub context: OpenGlContext,
    pub window_state: WindowState,
}

impl Renderer {
    /// Creates a new renderer
    pub fn new(width: u32, height: u32) -> Result<Self, AppError> {
        Ok(Self {
            context: OpenGlContext::new(width, height)?,
            window_state: WindowState::new(width, height),
        })
    }

    pub fn load_map<R>(&mut self, map: R)
    where R: ::std::io::Read
    {
        // todo: load map
    }

    pub fn show_start_menu(&mut self) {

    }

    pub fn run_main_loop(&mut self) {

    }
}
