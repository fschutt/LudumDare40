//! The AudioContext is a channeled receiver than runs on a different thread.
//!
//! This makes the music independent from the game loop

use std::thread;
use std::sync::mpsc;
use std::io::BufReader;
use rodio::{Source, Decoder, Endpoint};

pub const TITLE_SCREEN_SONG_DATA: &[u8] = include_bytes!("../assets/sounds/music/title_screen.ogg");

pub const AUDIO_MSG_PLAY_TITLE_SCREEN_SONG: &'static str = "play_level_1";

pub struct AudioContext {
    sender: mpsc::Sender<&'static str>,
    thread_handle: thread::JoinHandle<()>,
    currently_loaded_song: Option<&'static str>,
}

impl AudioContext {

    /// Starts a thread, returns the context
    pub fn new() -> Self {
        // Create a simple streaming channel
        #[allow(deprecated)]
        let endpoint = ::rodio::get_default_endpoint().unwrap();

        let (tx, rx) = mpsc::channel();

        let thread_handle = thread::spawn(move || Self::async_music_loop(rx, endpoint));

        Self {
            sender: tx,
            thread_handle: thread_handle,
            currently_loaded_song: None,
        }
    }

    pub fn send_msg(&self, msg: &'static str) -> Result<(), mpsc::SendError<&'static str>> {
        self.sender.send(msg)
    }

    /// Constantly
    fn async_music_loop(rx: mpsc::Receiver<&'static str>, endpoint: Endpoint) {
        use ::std::io::Cursor;
        use ::std::io::BufReader;
        use ::std::time::Duration;

        loop {
            if let Ok(signal) = rx.try_recv() {
                match signal {
                    AUDIO_MSG_PLAY_TITLE_SCREEN_SONG => {
                        let title_screen_song = Decoder::new(BufReader::new(Cursor::new(TITLE_SCREEN_SONG_DATA))).unwrap()
                                                .repeat_infinite()
                                                .fade_in(Duration::from_secs(2));
                        ::rodio::play_raw(&endpoint, title_screen_song.convert_samples());
                    },
                    _ => { println!("error sound message: {:?}", signal);}
                }
            } else {
                break;
            }

            ::std::thread::sleep(::std::time::Duration::from_millis(30));
        }
    }

}
