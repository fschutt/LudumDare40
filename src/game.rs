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
use ui::{Ui, UiRect, UiRendererData, UiActions};

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
#[derive(Debug, Clone)]
pub enum GameState {
    /// The start menu should be rendered
    StartMenu,
    /// The in-game state for the player. Contains all the items for the world, etc.
    Game(Box<PlayerState>),
}

impl GameState {
    /// Get the UI of the game, depending on what state the game is in.
    pub fn get_ui(&self) -> Ui {
        match *self {
            GameState::StartMenu => {
                // Start menu UI
                Ui {
                    rectangles: vec![UiRect {
                        // "Press start" button
                        x: [0.0, 0.0, 0.0, 0.0],
                        y: [0.0, 0.0, 0.0, 0.0],
                        data: Box::new(UiRendererData {
                            /* important! the start button has a tag - its ID */
                            tag: Some(::assets::START_SCREEN_BUTTON_00_ID),
                            actions: UiActions {
                                onmouseenter: Some(::actions::start_game_enter),
                                onmouseleave: Some(::actions::start_game_leave),
                                onmouseup: Some(::actions::start_game),
                                .. Default::default()
                            },
                            .. Default::default()
                        })
                    }],
                }
            },
            GameState::Game(ref player_state) => {
                Ui::default()
            }
        }
    }
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
        use glium::backend::Facade;
        use glium::glutin::MouseCursor;

        let mut previous_frame_ui = Ui::default();
        let mut previous_mouse_cursor_type = MouseCursor::Default;

        'outer: loop {

            let begin_time = ::std::time::Instant::now();

            // updates the game state, by using the UI of the previous frame as a reference
            for ev in self.renderer.context.display.poll_events() {
                let is_window_open = self.renderer.window_state.handle_event(&ev, &previous_frame_ui, &mut self.game_state);
                if !is_window_open {
                    break 'outer;
                }
            }

            // update the mouse cursor
            let current_mouse_cursor_type = self.renderer.window_state.mouse_state.mouse_cursor_type;
            if current_mouse_cursor_type != previous_mouse_cursor_type {
                self.renderer.context.display.get_window().unwrap().set_cursor(current_mouse_cursor_type);
                previous_mouse_cursor_type = current_mouse_cursor_type;
            }

            // the GameState generates the UI
            let mut current_frame_ui = self.game_state.get_ui();

            let mut game_frame = GameFrame {
                frame: self.renderer.context.display.draw(),
                context: &self.renderer.context,
                audio_context: &self.audio_context,
                font_ids: &self.available_font_ids,
                texture_ids: &self.available_texture_ids
            };

            match self.game_state {
                GameState::StartMenu => {
                    show_start_menu(&mut game_frame, self.renderer.context.display.get_context(),
                                    &self.renderer.context.shader_programs, &mut current_frame_ui);
                },
                GameState::Game(ref player_state) => {
                    show_game(&mut game_frame, self.renderer.context.display.get_context(),
                              &self.renderer.context.shader_programs,
                              player_state.physics_world.finalize(), &player_state.camera, &player_state);
                }
            }

            game_frame.drop();

            previous_frame_ui = current_frame_ui;
            ::std::thread::sleep(::std::time::Duration::from_millis(16));
        }
    }
}

/// Draw the start menu
fn show_start_menu(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap, ui: &mut Ui) {

    use glium::Surface;
    use texture::TargetPixelRegion;
    use ui::{Ui, UiRect, UiRendererData, UiActions};

    frame.clear_screen(Color::light_blue());

    let main_font = frame.get_font(FONT_BIG_ID);
    let small_font = frame.get_font(FONT_SMALL_ID);

    let (w, h) = frame.frame.get_dimensions();
    let center_w = w as f32 / 2.0;
    let center_h = h as f32 / 2.0;

    draw_text_with_shadow(frame, ::assets::GAME_TITLE, &main_font, 0.3, center_w, 2);
    draw_text_with_shadow(frame, "(C) Felix Schütt", &small_font, 0.8, center_w, 1);
    draw_text_with_shadow(frame, "Ludum Dare 40", &small_font, 0.85, center_w, 1);

    let start_game_button_width = 190.0; // px
    let start_game_button_height = 49.0; // px
    let half_start_game_button_width  = start_game_button_width  / 2.0;
    let half_start_game_button_height = start_game_button_height / 2.0;

    let left  = center_w - half_start_game_button_width;
    let right = center_w + half_start_game_button_width;
    let bottom = center_h - half_start_game_button_height;
    let top = center_h + half_start_game_button_height;

    let start_button_target_pixel_region = TargetPixelRegion {
        screen_bottom_x: left as u32,
        screen_bottom_y: bottom as u32,
        screen_width: start_game_button_width as u32,
        screen_height: start_game_button_height as u32,
    };

    let button_arr_x = [left, right, left, right];
    let button_arr_y = [top, top, bottom, bottom];

    {
        let start_btn = ui.get_mut_rect_by_tag(::assets::START_SCREEN_BUTTON_00_ID);

        start_btn.x = button_arr_x;
        start_btn.y = button_arr_y;
        start_btn.data.image = Some(TextureInstanceId {
            source_texture_region: ::assets::START_SCREEN_BUTTON_00_TX_STR,
            target_texture_region: start_button_target_pixel_region,
        });
    }

    for rect in ui.rectangles.iter_mut() {
        if let Some(ref texture_instance_id) = rect.data.image {
            frame.draw_texture(display, &texture_instance_id, 1.0, shaders);
        }
    }

    draw_text_with_shadow(frame, "Start Game", &small_font, 0.5, center_w, 0);
}

fn draw_text_with_shadow(frame: &mut GameFrame, text: &str, font: &FontInstanceId,
                                  offset_y: f32, offset_x: f32, shadow_offset: u32) {

    use glium::Surface;

    // calculate centered position of the text and draw text
    let (w, h) = frame.frame.get_dimensions();
    let font_width = frame.calculate_font_width(&font, text);
    let screen_x = (offset_x - (font_width / 2.0)) as u32;
    let screen_y = h - (offset_y * h as f32) as u32;

    let text = Text { font: &font, text: text, screen_x: screen_x, screen_y: screen_y };

    let mut shadow_text = text.clone();
    shadow_text.screen_x += shadow_offset;
    shadow_text.screen_y -= shadow_offset * 2;

    frame.draw_font(&shadow_text, Color::black());
    frame.draw_font(&text, Color::white());
}

// --- draw game

// Draw the actual game
fn show_game(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap,
             data: PhysicsFinalizedData, camera: &Camera, player_state: &PlayerState)
{
    use glium::Surface;

    frame.clear_screen(Color::light_blue());

    // draw highscore
    let score = format!("{:.2}", player_state.highscore);
    let initial_floor_height = 25.0;
    let big_font = frame.get_font(FONT_BIG_ID);

    let height_in_screen_pixels = ((player_state.highscore / 10.0) + initial_floor_height) as u32;

    frame.draw_font(&Text { font: &big_font, text: &score, screen_x: 50 - 2, screen_y: height_in_screen_pixels - 4 }, Color::black());
    frame.draw_font(&Text { font: &big_font, text: &score, screen_x: 50, screen_y: height_in_screen_pixels }, Color::white());

    draw_highscore_line_test(frame, display, height_in_screen_pixels);
}

fn draw_highscore_line_test(frame: &mut GameFrame, display: &Rc<Context>, line_height: u32)
{
    use glium::Surface;
    use glium::DrawParameters;

    // 100 pixel = 10 points in height of the highscore line
    // line is drawn using GL_LINES
    let (w, h) = frame.frame.get_dimensions();

    let mut x_val = 0;
    while x_val < w {
        x_val += 20;
        x_val += 50;
    }

    let draw_parameters = DrawParameters {
        .. Default::default()
    };
}
