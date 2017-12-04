//! The AudioContext is a channeled receiver than runs on a different thread.
//!
//! This makes the music independent from the game loop

use std::thread;
use std::sync::mpsc;
use cpal;
use lewton::inside_ogg::OggStreamReader;

pub struct AudioContext {
    sender: mpsc::Sender<&'static str>,
    thread_handle: thread::JoinHandle<()>,
}

impl AudioContext {

    /// Starts a thread, returns the context
    pub fn new() -> Self {

        let (tx, rx) = mpsc::channel();

        let thread_handle = thread::spawn(move || Self::async_music_loop(rx));

        Self {
            sender: tx,
            thread_handle: thread_handle,
        }
    }

    pub fn send_msg(&self, msg: &'static str) -> Result<(), mpsc::SendError<&'static str>> {
        self.sender.send(msg)
    }

    fn async_music_loop(rx: mpsc::Receiver<&'static str>) {

        use cpal::{UnknownTypeBuffer, Sample};
        use std::thread;
        use std::sync::Arc;

        // decode all the songs
        let title_screen_song = Arc::new(Song::decode_from_bytes(::assets::TITLE_SCREEN_SONG_DATA));
        let game_song_1 = Arc::new(Song::decode_from_bytes(::assets::GAME_SONG_1_DATA));
        let ending_screen_song = Arc::new(Song::decode_from_bytes(::assets::ENDING_SONG_1_DATA));

        println!("decoding done!");

        let mut last_song_tx: Option<mpsc::Sender<()>> = None;
        let mut last_song_id = "";

        while let Ok(event) = rx.recv() {

            if event == last_song_id { continue; }
            let mut current_song = title_screen_song.clone();

            match event {
                ::assets::AUDIO_MSG_PLAY_TITLE_SCREEN_SONG => { current_song = title_screen_song.clone(); },
                ::assets::AUDIO_MSG_PLAY_GAME_SONG => { current_song = game_song_1.clone(); },
                ::assets::AUDIO_MSG_PLAY_ENDING_SONG => { current_song = ending_screen_song.clone(); },
                _ => { println!("received garbage on audio thread: {:?}", event); }
            }

            let csong = current_song.clone();
            let (new_tx, new_rx) = mpsc::channel();

            thread::spawn(move || {

                let endpoint = cpal::default_endpoint().unwrap();
                let event_loop = cpal::EventLoop::new();

                let mut iter = (&*csong).decoded.iter().cycle();

                let supported_formats_range = endpoint.supported_formats().unwrap().next().unwrap();
                let mut format = supported_formats_range.with_max_samples_rate();
                format.samples_rate = cpal::SamplesRate(current_song.sample_rate);
                let voice_id = event_loop.build_voice(&endpoint, &format).unwrap();
                event_loop.play(voice_id);

                // event loop blocks
                event_loop.run(move |_voice_id, mut buffer| {

                    // can only be i16
                    if let UnknownTypeBuffer::I16(ref mut buffer) = buffer {
                        'outer: for (idx, d) in buffer.iter_mut().enumerate() {
                            *d = iter.next().map(|s| s.to_i16()).unwrap_or(0_i16);
                        }
                    }

                    if let Err(mpsc::TryRecvError::Disconnected) = new_rx.try_recv() {
                        return None;
                    }

                    Some(())
                });

            });

            // save the transmitting end in order to kill the song when we start a new song
            last_song_id = event;
            last_song_tx = Some(new_tx);
        }
    }
}

pub struct Song {
    pub decoded: Vec<i16>,
    pub sample_rate: u32,
}

impl Song {

    /// DO NOT TOUCH THIS DECODING FUNCTION IT IS MAGIC
    pub fn decode_from_bytes(data: &[u8]) -> Self {
        let mut srr = OggStreamReader::new(::std::io::Cursor::new(data)).unwrap();
        let sample_rate = srr.ident_hdr.audio_channels as f32 * srr.ident_hdr.audio_sample_rate as f32;

        let mut decoded = Vec::<i16>::new();
        while let Ok(Some(mut pck_samples)) = srr.read_dec_packet_itl() {
            decoded.append(&mut pck_samples);
        }

        Self {
            decoded: decoded,
            sample_rate: sample_rate as u32,
        }
    }
}
