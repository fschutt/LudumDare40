//! Main game state

use player_state::PlayerState;
use camera::Camera;
use renderer::Renderer;
use audio::AudioContext;
use color::Color;
use physics::{PhysicsWorld, PhysicsFinalizedData};
use {ShaderHashMap, FontInstanceIdMap, TextureInstanceIdMap, FastHashMap};
use font::FontInstanceId;
use texture::TextureInstanceId;
use frame::GameFrame;
use font::Text;
use std::rc::Rc;
use glium::backend::Context;

pub const FONT_BIG_ID: &str = "font_fredoka_big";
pub const FONT_SMALL_ID: &str = "font_fredoka_small";

pub const TEXTURE_START_GAME_ID: &str = "texture_start_game";

pub struct Game {
    pub renderer: Renderer,
    pub audio_context: AudioContext,
    pub available_font_ids: FontInstanceIdMap,
    pub available_texture_ids: TextureInstanceIdMap,
    pub game_state: GameState,
}

/// what the state of the game currently is (what should be drawn on the screen)
#[derive(Clone)]
pub enum GameState {
    /// The start menu should be rendered
    StartMenu,
    /// The in-game state for the player. Contains all the items for the world, etc.
    Game(Box<PlayerState>),
}

impl Game {

    /// Initializes the audio and renderer, adds all the fonts, and textures and shaders
    pub fn new(width: u32, height: u32) -> Self {

        use std::io::Cursor;

        // -- initialize audio
        let audio_context = AudioContext::new();
        audio_context.send_msg(::audio::AUDIO_MSG_PLAY_TITLE_SCREEN_SONG).unwrap();

        // -- initialize shaders
        let mut renderer = Renderer::new(width, height).unwrap();

        // -- initialize fonts
        let mut available_font_ids = FontInstanceIdMap::default();

        let font_instance_big_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_BIG_SIZE, Cursor::new(::assets::FONT));
        available_font_ids.insert(FONT_BIG_ID, font_instance_big_id);

        let font_instance_small_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_SMALL_SIZE, Cursor::new(::assets::FONT));
        available_font_ids.insert(FONT_SMALL_ID, font_instance_small_id);

        // -- initialize textures
        let mut available_texture_ids = TextureInstanceIdMap::default();

        let font_instance_big_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_BIG_SIZE, Cursor::new(::assets::FONT));
        let texture_start_game = renderer.context.add_texture_png(::assets::START_SCREEN_BUTTON_00_ID, Cursor::new(::assets::START_SCREEN_BUTTON_00));
        available_texture_ids.insert(TEXTURE_START_GAME_ID, texture_start_game);

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

            use glium::backend::Facade;

            match self.game_state {
                GameState::StartMenu => {
                    show_start_menu(&mut game_frame, self.renderer.context.display.get_context(),
                                    &self.renderer.context.shader_programs);
                },
                GameState::Game(ref player_state) => {
                    draw_game(&mut game_frame, self.renderer.context.display.get_context(),
                              &self.renderer.context.shader_programs,
                              player_state.physics_world.finalize(), &player_state.camera);
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
fn show_start_menu(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap) {
    use glium::Surface;
    use texture::TargetPixelRegion;

    frame.clear_screen(Color::light_blue());

    let main_font = frame.get_font(FONT_BIG_ID);
    let small_font = frame.get_font(FONT_SMALL_ID);

    draw_centered_text_with_shadow(frame, ::assets::GAME_TITLE, &main_font, 0.3, 2);
    draw_centered_text_with_shadow(frame, "(C) Felix Schütt", &small_font, 0.8, 1);
    draw_centered_text_with_shadow(frame, "Ludum Dare 40", &small_font, 0.85, 1);

    let start_game_button_width = 190.0; // px
    let start_game_button_height = 49.0; // px
    let half_start_game_button_width  = start_game_button_width  / 2.0;
    let half_start_game_button_height = start_game_button_height / 2.0;

    let (w, h) = frame.frame.get_dimensions();
    let center_w = w as f32 / 2.0;
    let center_h = h as f32 / 2.0;

    let left  = center_w - half_start_game_button_width;
    let right = center_w + half_start_game_button_width;
    let bottom = center_h - half_start_game_button_height;
    let top = center_h + half_start_game_button_height;


    use ui::{Ui, UiRect, UiRendererData, UiActions};

    let start_button_target_pixel_region = TargetPixelRegion {
        screen_bottom_x: left as u32,
        screen_bottom_y: bottom as u32,
        screen_width: start_game_button_width as u32,
        screen_height: start_game_button_height as u32,
    };

    // draw "start game button"
    // todo: seperate this out so the input module has access to it
    let start_game_ui = Ui {
        rectangles: vec![UiRect {
            // tl, tr, bl, br
            x: [left, right, left, right],
            y: [top, top, bottom, bottom],
            data: Box::new(UiRendererData {
                color: None,
                image: Some(TextureInstanceId {
                    source_texture_region: ::assets::START_SCREEN_BUTTON_00_TX_STR,
                    target_texture_region: start_button_target_pixel_region,
                }),
                text: None,
                actions: UiActions::empty(), // TODO: add button callback!
            })
        }],
    };

    for rect in start_game_ui.rectangles.into_iter() {
        if let Some(texture_instance_id) = rect.data.image {
            frame.draw_texture(display, &texture_instance_id, 0.7, shaders);
        }
    }

    draw_centered_text_with_shadow(frame, "Start Game", &small_font, 0.5, 0);
}

fn draw_centered_text_with_shadow(frame: &mut GameFrame, text: &str, font: &FontInstanceId, offset_y: f32, shadow_offset: u32) {

    use glium::Surface;

    // calculate centered position of the text and draw text
    let (w, h) = frame.frame.get_dimensions();
    let font_width = frame.calculate_font_width(&font, text);
    let screen_x = (w / 2) - (font_width / 2.0) as u32;
    let screen_y = h - (offset_y * h as f32) as u32;

    let text = Text { font: &font, text: text, screen_x: screen_x, screen_y: screen_y };

    let mut shadow_text = text.clone();
    shadow_text.screen_x += shadow_offset;
    shadow_text.screen_y -= shadow_offset * 2;

    frame.draw_font(&shadow_text, Color::black());
    frame.draw_font(&text, Color::white());
}

// Draw the actual game
fn draw_game(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap, data: PhysicsFinalizedData, camera: &Camera) {
    // draw highscore
    let score = "40";
    let small_font = frame.get_font(FONT_SMALL_ID);

    draw_centered_text_with_shadow(frame, score, &small_font, 0.3, 1);
}
