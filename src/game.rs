//! Main game state

use renderer::Renderer;
use audio::AudioContext;

pub struct Game {
    renderer: Renderer,
    audio_context: AudioContext,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
            let renderer = Renderer::new(width, height).unwrap();
            let audio_context = AudioContext::new();
            audio_context.send_msg(::audio::AUDIO_MSG_PLAY_TITLE_SCREEN_SONG).unwrap();

        Self {
            renderer: renderer,
            audio_context: audio_context,
        }
    }

    pub fn load_map<R>(&mut self, map: R)
    where R: ::std::io::Read
    {
        // todo: load map
    }

    pub fn show_start_menu(&mut self) {

    }

    pub fn update_score(&mut self) {

    }

    pub fn run_main_loop(&mut self) {
        self.renderer.run_main_loop();
    }
}
