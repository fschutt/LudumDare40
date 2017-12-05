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
pub const FONT_MEDIUM_ID: &str = "font_fredoka_medium";
pub const FONT_SMALL_ID: &str = "font_fredoka_small";

pub const TEXTURE_START_GAME_ID: &str = "texture_start_game";
pub const TEXTURE_HERO_CHARACTER_ID: &str = "texture_hero_character";
pub const TEXTURE_CRATE_ID: &str = "texture_crate";
pub const TEXTURE_BACKGROUND_ID: &str = "texture_background";

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

    pub fn get_song(&self) -> &'static str {
        match *self {
            GameState::StartMenu => {
                ::assets::AUDIO_MSG_PLAY_TITLE_SCREEN_SONG
            },
            GameState::Game(_) => {
                ::assets::AUDIO_MSG_PLAY_GAME_SONG
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

        // -- initialize shaders
        let mut renderer = Renderer::new(width, height).unwrap();

        // -- initialize fonts
        let mut available_font_ids = FontInstanceIdMap::default();

        let font_instance_big_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_BIG_SIZE, Cursor::new(::assets::FONT));
        available_font_ids.insert(FONT_BIG_ID, font_instance_big_id);

        let font_instance_medium_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_MEDIUM_SIZE, Cursor::new(::assets::FONT));
        available_font_ids.insert(FONT_MEDIUM_ID, font_instance_medium_id);

        let font_instance_small_id = renderer.context.add_font(::assets::FONT_ID, ::assets::FONT_SMALL_SIZE, Cursor::new(::assets::FONT));
        available_font_ids.insert(FONT_SMALL_ID, font_instance_small_id);

        // -- initialize textures
        let mut available_texture_ids = TextureInstanceIdMap::default();

        let texture_start_game = renderer.context.add_texture_png(::assets::START_SCREEN_BUTTON_00_ID, Cursor::new(::assets::START_SCREEN_BUTTON_00));
        available_texture_ids.insert(TEXTURE_START_GAME_ID, texture_start_game);

        let texture_hero_char = renderer.context.add_texture_png(::assets::HERO_TEXTURE_ID, Cursor::new(::assets::HERO_TEXTURE));
        available_texture_ids.insert(TEXTURE_HERO_CHARACTER_ID, texture_hero_char);

        let texture_crate = renderer.context.add_texture_png(::assets::CRATE_TEXTURE_ID, Cursor::new(::assets::CRATE_TEXTURE_DATA));
        available_texture_ids.insert(TEXTURE_CRATE_ID, texture_crate);

        let texture_background = renderer.context.add_texture_png(::assets::BACKGROUND_3_TEXTURE_ID, Cursor::new(::assets::BACKGROUND_3_TEXTURE_DATA));
        available_texture_ids.insert(TEXTURE_BACKGROUND_ID, texture_background);

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
        use input::GameInputEvent;

        let mut previous_frame_ui = Ui::default();
        let mut previous_mouse_cursor_type = MouseCursor::Default;

        'outer: loop {

            let begin_time = ::std::time::Instant::now();
            let mut input_events = Vec::<GameInputEvent>::new();

            // updates the game state, by using the UI of the previous frame as a reference
            for ev in self.renderer.context.display.poll_events() {
                let (is_window_open, mut events) = self.renderer.window_state.handle_event(&ev, &previous_frame_ui, &mut self.game_state);
                if !is_window_open { break 'outer; }
                input_events.append(&mut events);
            }

            // update the mouse cursor
            let current_mouse_cursor_type = self.renderer.window_state.mouse_state.mouse_cursor_type;
            if current_mouse_cursor_type != previous_mouse_cursor_type {
                self.renderer.context.display.get_window().unwrap().set_cursor(current_mouse_cursor_type);
                previous_mouse_cursor_type = current_mouse_cursor_type;
            }

            // update the audio, change song if needed. the audio module only changes the song if it has changed
            self.audio_context.send_msg(self.game_state.get_song())
            .map_err(|e| { println!("could not send new song: {:}", e); })
            .unwrap_or(());

            // the GameState generates the UI
            let mut current_frame_ui = self.game_state.get_ui();

            let mut game_frame = GameFrame {
                frame: self.renderer.context.display.draw(),
                context: &self.renderer.context,
                font_ids: &self.available_font_ids,
                texture_ids: &self.available_texture_ids
            };

            match self.game_state {
                GameState::StartMenu => {
                    show_start_menu(&mut game_frame, self.renderer.context.display.get_context(),
                                    &self.renderer.context.shader_programs, &mut current_frame_ui);
                },
                GameState::Game(ref mut player_state) => {
                    player_state.camera.screen_width = self.renderer.window_state.width as f32;
                    player_state.camera.screen_height = self.renderer.window_state.height as f32;

                    let world_finalized = player_state.finalize(input_events);
                    show_game(&mut game_frame, self.renderer.context.display.get_context(),
                              &self.renderer.context.shader_programs,
                              &world_finalized, &player_state.camera);
                }
            }

            game_frame.drop();
            self.renderer.context.texture_system.highest_texture.set(0.99);
            previous_frame_ui = current_frame_ui;
            ::std::thread::sleep(::std::time::Duration::from_millis(16));
        }
    }
}

/// Draw the start menu
fn show_start_menu(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap, ui: &mut Ui)
{
    use glium::Surface;
    use texture::TargetPixelRegion;
    use ui::{Ui, UiRect, UiRendererData, UiActions};
    use texture::TextureDrawOptions;

    frame.clear_screen(Color::light_blue());

    let big_font = frame.get_font(FONT_BIG_ID);
    let medium_font = frame.get_font(FONT_MEDIUM_ID);
    let small_font = frame.get_font(FONT_SMALL_ID);

    let (w, h) = frame.frame.get_dimensions();
    let center_w = w as f32 / 2.0;
    let center_h = h as f32 / 2.0;

    draw_text_with_shadow(frame, ::assets::GAME_TITLE, &big_font, 0.3, center_w, 2);
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
            frame.draw_texture(display, &texture_instance_id, 1.0, shaders, TextureDrawOptions::default());
        }
    }

    let start_game_text = "Start Game";
    let font_width = frame.calculate_font_width(&medium_font, start_game_text);
    let text = Text {
        font: &medium_font,
        text: start_game_text,
        screen_x: (center_w as u32) - ((font_width / 2.0) as u32),
        screen_y: (center_h as u32) - (medium_font.font_size / 2)
    };
    frame.draw_font(&text, Color::black());
}

fn draw_text_with_shadow(frame: &mut GameFrame, text: &str, font: &FontInstanceId,
                                  offset_y: f32, offset_x: f32, shadow_offset: u32)
{
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
             game_finalized_data: &PhysicsFinalizedData, camera: &Camera)
{
    use glium::Surface;

    frame.clear_screen(Color::light_blue());
    draw_background(frame, display, shaders, game_finalized_data);
    draw_highscore(frame, display, shaders, game_finalized_data);
    draw_crates(frame, display, shaders, game_finalized_data);
    draw_character(frame, display, shaders, game_finalized_data);

}

fn draw_background(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap,
                   game_finalized_data: &PhysicsFinalizedData)
{
    use texture::{TargetPixelRegion, TextureDrawOptions};
    use glium::Surface;

    let (w, h) = frame.frame.get_dimensions();

    let max = w.max(h);
    let min = w.min(h);
    let aspect_ratio = w as f32 / h as f32;

    let background_sprite_region = TargetPixelRegion {
        screen_bottom_x: 0,
        screen_bottom_y: 0,
        screen_width: w,
        screen_height: h,
    };

    let texture_instance_id = TextureInstanceId {
        // TODO: set character state (side, flying, etc.) here
        source_texture_region: ::assets::BACKGROUND_3_TEXTURE_TX_STR,
        target_texture_region: background_sprite_region,
    };

    frame.draw_texture(display, &texture_instance_id, 1.0, shaders, TextureDrawOptions::PixelPerfect);
}

fn draw_crates(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap,
               game_finalized_data: &PhysicsFinalizedData)
{
    use texture::{TargetPixelRegion, TextureDrawOptions};

    for crate_box in &game_finalized_data.crates {

        let crate_sprite_region = TargetPixelRegion {
            screen_bottom_x: crate_box.x as u32,
            screen_bottom_y: crate_box.y as u32,
            screen_width: crate_box.width as u32,
            screen_height: crate_box.height as u32,
        };

        let texture_instance_id = TextureInstanceId {
            // TODO: set character state (side, flying, etc.) here
            source_texture_region: ::assets::CRATE_TEXTURE_TX_STR,
            target_texture_region: crate_sprite_region,
        };

        frame.draw_texture(display, &texture_instance_id, 1.0, shaders, TextureDrawOptions::PixelPerfect);
    }
}

fn draw_character(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap,
                  game_finalized_data: &PhysicsFinalizedData)
{
    use texture::{TargetPixelRegion, TextureDrawOptions};

    let player_sprite_region = TargetPixelRegion {
        screen_bottom_x: game_finalized_data.player_position.x as u32,
        screen_bottom_y: game_finalized_data.player_position.y as u32,
        screen_width: game_finalized_data.player_position.width as u32,
        screen_height: game_finalized_data.player_position.height as u32,
    };

    let texture_instance_id = TextureInstanceId {
        // TODO: set character state (side, flying, etc.) here
        source_texture_region: ::assets::HERO_TX_NORMAL_STR,
        target_texture_region: player_sprite_region,
    };

    frame.draw_texture(display, &texture_instance_id, 1.0, shaders, TextureDrawOptions::PixelPerfect);
}

fn draw_highscore(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap,
                  game_finalized_data: &PhysicsFinalizedData)
{
    let score = format!("{:.2}", game_finalized_data.highscore);
    let initial_floor_height = 25.0;
    let big_font = frame.get_font(FONT_BIG_ID);

    let height_in_screen_pixels = ((game_finalized_data.highscore) + initial_floor_height) as u32;
    let font_offset = 25;

    frame.draw_font(&Text { font: &big_font, text: &score, screen_x: 25 + 2, screen_y: height_in_screen_pixels  + font_offset - 4 }, Color::black());
    frame.draw_font(&Text { font: &big_font, text: &score, screen_x: 25, screen_y: height_in_screen_pixels + font_offset }, Color::white());

    draw_highscore_line(frame, display, height_in_screen_pixels, shaders);
}

fn draw_ground(frame: &mut GameFrame, display: &Rc<Context>, shaders: &ShaderHashMap)
{

}

fn draw_highscore_line(frame: &mut GameFrame, display: &Rc<Context>, line_height: u32, shaders: &ShaderHashMap)
{
    use glium::Surface;
    use glium::DrawParameters;
    use texture::PixelScreenVert;
    use glium::VertexBuffer;

    // 100 pixel = 10 points in height of the highscore line
    // line is drawn using GL_LINES
    let (w, h) = frame.frame.get_dimensions();

    let mut verts_a = Vec::with_capacity(20);
    let mut verts_b = Vec::with_capacity(20);

    let mut x_val = 10;
    while x_val < w {
        verts_a.push(PixelScreenVert {
            position:   [(x_val + 2) as f32,
                         line_height as f32,
                         0.2],
            tex_coords: [0.0, 0.0]
        });
        verts_b.push(PixelScreenVert {
            position:   [x_val as f32,
                         (line_height + 2) as f32,
                         0.2],
            tex_coords: [0.0, 0.0]
        });
        x_val += 40;

        verts_a.push(PixelScreenVert {
            position:   [(x_val + 2) as f32,
                         line_height as f32,
                         0.2],
            tex_coords: [0.0, 0.0]
        });
        verts_b.push(PixelScreenVert {
            position:   [x_val as f32,
                         (line_height + 2) as f32,
                         0.2],
            tex_coords: [0.0, 0.0]
        });
        x_val += 20;
    }

    let vbuf_a = VertexBuffer::new(display, &verts_a).unwrap();
    let vbuf_b = VertexBuffer::new(display, &verts_b).unwrap();

    let draw_parameters = DrawParameters {
        line_width: Some(9.0),
        .. Default::default()
    };

    let uniforms = uniform!{
        window_width: w as f32,
        window_height: h as f32,
        in_color: [0.0_f32, 0.0, 0.0, 1.0],
    };

    let program = shaders.get(::context::PIXEL_TO_SCREEN_SHADER_LINE_ONLY_ID).unwrap();
    frame.frame.draw(&vbuf_a, ::context::NO_INDICES_BUFFER_LINE, program, &uniforms, &draw_parameters).unwrap();

    let uniforms = uniform!{
        window_width: w as f32,
        window_height: h as f32,
        in_color: [0.9_f32, 0.9, 0.9, 1.0],
    };

    frame.frame.draw(&vbuf_b, ::context::NO_INDICES_BUFFER_LINE, program, &uniforms, &draw_parameters).unwrap();
}
