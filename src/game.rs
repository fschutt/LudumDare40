//! Main game state

use renderer::Renderer;
use camera::Camera;
use audio::AudioContext;
use color::Color;
use physics::{PhysicsWorld, PhysicsFinalizedData};
use {FontInstanceIdMap, TextureInstanceIdMap, FastHashMap};
use font::FontInstanceId;
use texture::TextureInstanceId;
use frame::GameFrame;
use font::Text;

pub const FONT_BIG_ID: &str = "font_fredoka_big";
pub const FONT_SMALL_ID: &str = "font_fredoka_small";

pub struct Game {
    pub renderer: Renderer,
    pub audio_context: AudioContext,
    pub available_font_ids: FontInstanceIdMap,
    pub available_texture_ids: TextureInstanceIdMap,
    pub game_state: GameState,
}

/// what the state of the game currently is (what should be drawn on the screen)
pub enum GameState {
    /// The start menu should be rendered
    StartMenu,
    /// The in-game state for the player. Contains all the items for the world, etc.
    Game(Box<PlayerState>),
}

///
pub struct PlayerState {
    pub camera: Camera,
    pub physics_world: PhysicsWorld,
    pub highscore: u32,
}

impl Game {

    /// Initializes the audio and renderer, adds all the fonts, and textures
    pub fn new(width: u32, height: u32) -> Self {

        use std::io::Cursor;

        // initialize audio first so that the player knows the game doesn't hang
        let audio_context = AudioContext::new();
        audio_context.send_msg(::audio::AUDIO_MSG_PLAY_TITLE_SCREEN_SONG).unwrap();

        let mut renderer = Renderer::new(width, height).unwrap();

        // load fonts and texture needed for the game
        let font_instance_big_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_BIG_SIZE, Cursor::new(::assets::FONT));
        let font_instance_small_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_SMALL_SIZE, Cursor::new(::assets::FONT));

        let mut available_font_ids = FastHashMap::<&'static str, FontInstanceId>::default();
        available_font_ids.insert(FONT_BIG_ID, font_instance_big_id);
        available_font_ids.insert(FONT_SMALL_ID, font_instance_small_id);

        let available_texture_ids = FastHashMap::<&'static str, TextureInstanceId>::default();
        // insert textures here

        Self {
            renderer: renderer,
            audio_context: audio_context,
            available_font_ids: available_font_ids,
            available_texture_ids: available_texture_ids,
            game_state: GameState::StartMenu,
        }
    }

    /// Main game loop
    pub fn run_main_loop(&mut self) {

        use glium::Surface;

        'outer: loop {
            let begin_time = ::std::time::Instant::now();

            for ev in self.renderer.context.display.poll_events() {
                let (is_window_open, _should_redraw) = self.renderer.window_state.handle_event(&ev);
                if !is_window_open {
                    break 'outer;
                }
            }


            let mut game_frame = GameFrame {
                frame: self.renderer.context.display.draw(),
                context: &self.renderer.context,
                audio_context: &self.audio_context,
                font_ids: &self.available_font_ids,
                texture_ids: &self.available_texture_ids
            };

            match self.game_state {
                GameState::StartMenu => {
                    show_start_menu(&mut game_frame);
                },
                GameState::Game(ref player_state) => {
                    draw_game(&mut game_frame, player_state.physics_world.finalize(), &player_state.camera);
                }
            }

            game_frame.drop();

            let time_now = ::std::time::Instant::now();
            let time_diff = time_now - begin_time;
            let frame_time = ::std::time::Duration::from_millis(16);
            if time_diff < frame_time {
                ::std::thread::sleep(frame_time - time_diff);
            }
        }
    }
}


/// Draw the start menu
fn show_start_menu(frame: &mut GameFrame) {
    use glium::Surface;

    frame.clear_screen(Color::light_blue());

    // calculate centered position of the text
    let (w, h) = frame.frame.get_dimensions();

    let text = Text {
        font: frame.get_font(FONT_BIG_ID),
        text: "Hello",
        screen_x: 20,
        screen_y: 0
    };

    // draw main menu text
    frame.draw_font(&text, Color::white());
}

// Draw the actual game
fn draw_game(frame: &mut GameFrame, data: PhysicsFinalizedData, camera: &Camera) {

}
