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

    pub fn run_main_loop(&mut self) {
        'outer: loop {
            let begin_time = ::std::time::Instant::now();
            for ev in self.context.display.poll_events() {
                let (is_window_open, _should_redraw) = self.window_state.handle_event(&ev);
                if !is_window_open {
                    break 'outer;
                }
            }

            // update the world if there is any

            // do the drawing

            // update the audio

            let time_now = ::std::time::Instant::now();
            let time_diff = time_now - begin_time;
            let frame_time = ::std::time::Duration::from_millis(16);
            if time_diff < frame_time {
                println!("waiting: {:?}", frame_time);
                ::std::thread::sleep(frame_time - time_diff);
            }
        }
    }
}
