//! Input handling module, handles incoming events for the window

use glium;
use glium::glutin::{Event, MouseCursor, ElementState,VirtualKeyCode, MouseButton};

use std::time::{Duration, Instant};
use ui::{Ui, UiRect, UiRendererData};
use game::GameState;

pub enum GameInputEvent {
    PlayerJump,
    PlayerGoLeft,
    PlayerGoRight,
    PlayerTakeBox,
}

/// Determines which keys are pressed currently (modifiers, etc.)
#[derive(Debug, Clone)]
pub struct KeyboardState
{
    /// Modifier keys that are currently actively pressed during this cycle
    pub modifiers: Vec<VirtualKeyCode>,
    /// Hidden keys, such as the "n" in CTRL + n. Always lowercase
    pub hidden_keys: Vec<char>,
    /// Actual keys pressed during this cycle (i.e. regular text input)
    pub keys: Vec<char>,
}

impl KeyboardState
{
    pub fn new() -> Self
    {
        Self {
            modifiers: Vec::new(),
            hidden_keys: Vec::new(),
            keys: Vec::new(),
        }
    }
}

/// Mouse position on the screen
#[derive(Debug, Copy, Clone)]
pub struct MouseState
{
    /// Current mouse cursor type
    pub mouse_cursor_type: MouseCursor,
    //// Where the mouse cursor is. None if the window is not focused
    pub mouse_cursor: Option<(i32, i32)>,
    //// Is the left MB down?
    pub left_down: bool,
    //// Is the right MB down?
    pub right_down: bool,
    //// Is the middle MB down?
    pub middle_down: bool,
    /// How far has the mouse scrolled in x direction?
    pub mouse_scroll_x: f32,
    /// How far has the mouse scrolled in y direction?
    pub mouse_scroll_y: f32,
}

impl MouseState
{
    /// Creates a new mouse state
    /// Input: How fast the scroll (mouse) should be converted into pixels
    /// Usually around 10.0 (10 pixels per mouse wheel line)
    pub fn new() -> Self
    {
        MouseState {
            mouse_cursor_type: MouseCursor::Default,
            mouse_cursor: Some((0, 0)),
            left_down: false,
            right_down: false,
            middle_down: false,
            mouse_scroll_x: 0.0,
            mouse_scroll_y: 0.0,
        }
    }
}

/// State, size, etc of the window, for comparing to the last frame
#[derive(Debug, Clone)]
pub struct WindowState
{
    /// The state of the keyboard
    pub(crate) keyboard_state: KeyboardState,
    /// The state of the mouse
    pub(crate) mouse_state: MouseState,
    /// Width of the window
    pub width: u32,
    /// Height of the window
    pub height: u32,
    /// Time of the last rendering update, set after the `redraw()` method
    pub time_of_last_update: Instant,
    /// Minimum frame time
    pub min_frame_time: Duration,
}

impl WindowState
{
    /// Creates a new window state
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self
    {
        Self {
            keyboard_state: KeyboardState::new(),
            mouse_state: MouseState::new(),
            width,
            height,
            time_of_last_update: Instant::now(),
            min_frame_time: Duration::from_millis(16),
        }
    }


    /// Handles the event, updates the UI, then returns if the window was not
    /// closed (false on closed)
    #[inline]
    pub(crate) fn handle_event(
        &mut self,
        event: &Event,
        ui: &Ui,
        game_state: &mut GameState,
    ) -> (bool, Vec<GameInputEvent>)
    {
        // update the state of the input information
        use glium::glutin::Event::*;

        // this will be true if the UI (drawn on top of the map) has already handled
        // the incoming event for example, we don't want a click on the UI to be
        // received as a click on the map
        let _ui_handles_event = self.ui_handle_event(ui, game_state, event);

        match *event {
            Closed => return (false, Vec::new()),
            MouseMoved(x, y) => { self.handle_mouse_move(game_state, x, y); },
            KeyboardInput(state, _, vk_code) => { self.handle_vk_code(game_state, state, vk_code); },
            MouseInput(state, button) => { self.handle_mouse_click(game_state, state, button); },
            Resized(width, height) => { self.handle_resize(game_state, width, height); }
            Focused(b) => { self.handle_focus(game_state, b); },
            _ => { },
        }

        (true, self.update_game_state_from_kbinput(game_state))
    }

    /// Handle the focus of a window
    #[inline]
    pub fn handle_focus(
        &mut self,
        _game_state: &mut GameState,
        focused: bool,
    )
    {
        if !focused {
            self.mouse_state.mouse_cursor = None;
        }
    }

    /// Handle the "resize window" event
    #[inline]
    fn handle_resize(
        &mut self,
        _game_state: &mut GameState,
        width: u32,
        height: u32,
    )
    {
        self.width = width;
        self.height = height;
    }

    /// Updates mouse movement
    #[inline]
    fn handle_mouse_move(
        &mut self,
        game_state: &mut GameState,
        x: i32,
        y: i32,
    )
    {
        if Instant::now().duration_since(self.time_of_last_update) < self.min_frame_time {
            return;
        }

        let is_left_mouse_down = self.mouse_state.left_down;

        // no redraw needed if the mouse did not drag the map (todo: UI handling?)
        if !is_left_mouse_down {
            self.mouse_state.mouse_cursor = Some((x, self.height as i32 - y));
            return;
        }

        // dragging action
        let cur_mouse_cursor = self.mouse_state.mouse_cursor;

        if cur_mouse_cursor.is_none() {
            self.mouse_state.mouse_cursor = Some((x, self.height as i32 - y));
            return;
        }

        self.mouse_state.mouse_cursor = Some((x, self.height as i32 - y));
    }


    /// Updates the mouse state on a click
    #[inline]
    fn handle_mouse_click(
        &mut self,
        game_state: &mut GameState,
        state: ::glium::glutin::ElementState,
        button: ::glium::glutin::MouseButton,
    )
    {
        use glium::glutin::ElementState::*;
        use glium::glutin::MouseButton::*;

        let mouse = &mut self.mouse_state;

        match state {
            Pressed => match button {
                Left => {
                    mouse.left_down = true;
                },
                Right => {
                    mouse.right_down = true;
                },
                Middle => {
                    mouse.middle_down = true;
                },
                Other(_) => {},
            },

            Released => match button {
                Left => {
                    mouse.left_down = false;
                },
                Right => {
                    mouse.right_down = false;
                },
                Middle => {
                    mouse.middle_down = false;
                },
                Other(_) => {},
            },
        }
    }

    /// Checks if the there is a control character present. If yes, we will likely
    /// receive a
    /// "Character received" event later on. This is important because the
    /// "character received"
    /// event does not know if the character contains control characters. For
    /// example Ctrl + M and the
    /// Enter key have the same key code. Without this function it is impossible to
    /// distinguish
    /// between the two
    #[inline]
    fn handle_vk_code(
        &mut self,
        _game_state: &mut GameState,
        state: ElementState,
        vk_code: Option<VirtualKeyCode>
    ) {
        if vk_code.is_none() { return; }
        let vk_code = vk_code.unwrap();
        if state == ElementState::Pressed {
                self.keyboard_state.modifiers.push(vk_code);
        } else {
            let indices_found = self.keyboard_state.modifiers.iter().position(|e| *e == vk_code);
            if let Some(index) = indices_found {
                self.keyboard_state.modifiers.remove(index);
            }
        }
    }

    /// Parent function that handles any incoming UI event and delegates it to the
    /// rest of the UI handling functions.
    ///
    /// Returns if this function already handles the mouse event
    #[inline]
    pub fn ui_handle_event(
        &mut self,
        ui: &Ui,
        game_state: &mut GameState,
        event: &Event,
    ) -> bool
    {
        use glium::glutin::Event;

        let (old_x, old_y) = self.mouse_state.mouse_cursor.unwrap_or((-1, -1));

        // for now the UI does not handle scrolling
        // or global hotkeys such as CTRL + S
        match *event {
            // MouseMoved is for hover events and focusin / focusout
            Event::MouseMoved(x, y) => {
                self.ui_handle_mouse_move(ui, game_state, x, y, old_x, old_y)
            },
            // MouseInput is for onclick events
            Event::MouseInput(state, button) => {
                self.ui_handle_mouse_click(ui, game_state, state, button)
            },
            _ => false
        }
    }

    /// Checks if the mouse has moved over a UI element, calls the respective
    /// functions,
    ///
    /// returns false if the event should propagate to underlying elements.
    pub fn ui_handle_mouse_move(
        &mut self,
        ui: &Ui,
        game_state: &mut GameState,
        x: i32,
        y: i32,
        old_x: i32,
        old_y: i32,
    )  -> bool
    {
        for rect in ui.rectangles.iter() {
            let previous_point_in_rect = check_point_in_rect(old_x as f32, old_y as f32, &rect);
            let current_point_in_rect = check_point_in_rect(x as f32, y as f32, &rect);

            // choose between mouseenter and mouseleave
            if current_point_in_rect != previous_point_in_rect {
                if current_point_in_rect {
                    // call mouseenter event
                    if let Some(fptr) = rect.data.actions.onmouseenter {
                        return (fptr)(self, ui, game_state);
                    }
                } else {
                    // call mouseleave event
                    if let Some(fptr) = rect.data.actions.onmouseleave {
                        return (fptr)(self, ui, game_state);
                    }
                }
            }
        }

        false
    }

    /// Checks if any element from the currently active UI was clicked, calls the
    /// respective onmousedown and onmouseup functions.
    /// Returns if the screen should be updated.
    pub fn ui_handle_mouse_click(
        &mut self,
        ui: &Ui,
        game_state: &mut GameState,
        state: ElementState,
        _button: MouseButton,
    ) -> bool
    {
        let (x, y) = self.mouse_state.mouse_cursor.unwrap_or((-1, -1));

        if state == ElementState::Released {
            for rect in ui.rectangles.iter() {
                if !check_point_in_rect(x as f32, y as f32, &rect) { continue; }
                if let Some(fptr) = rect.data.actions.onmouseup {
                    return (fptr)(self, ui, game_state);
                }
            }
        }

        false
    }

    /// Update the WASD keys
    pub fn update_game_state_from_kbinput(&mut self, game_state: &mut GameState)
        -> Vec<GameInputEvent>
    {
        let mut relevant_inputs = Vec::<GameInputEvent>::new();

        if let GameState::Game(ref mut player_state) = *game_state {
            for key in &self.keyboard_state.modifiers {
                println!("key: {:?}", key);
                match *key {
                    VirtualKeyCode::W => { relevant_inputs.push(GameInputEvent::PlayerJump); },
                    VirtualKeyCode::Space => { relevant_inputs.push(GameInputEvent::PlayerJump); },
                    VirtualKeyCode::A => { relevant_inputs.push(GameInputEvent::PlayerGoLeft); },
                    VirtualKeyCode::D => { relevant_inputs.push(GameInputEvent::PlayerGoRight); },
                    VirtualKeyCode::Right => { relevant_inputs.push(GameInputEvent::PlayerGoRight); },
                    VirtualKeyCode::Left => { relevant_inputs.push(GameInputEvent::PlayerGoLeft); },
                    VirtualKeyCode::Up => { relevant_inputs.push(GameInputEvent::PlayerJump); },
                    _ => { }
                }
            }
        }

        self.keyboard_state.modifiers.clear();
        self.keyboard_state.hidden_keys.clear();

        relevant_inputs
    }
}

#[inline]
pub fn check_point_in_rect(
    x: f32,
    y: f32,
    rect: &UiRect<UiRendererData>,
) -> bool
{
    // top left, top right, bottom left, bottom right
    let (left, top) = (rect.x[0], rect.y[0]); // top left
    let (right, bottom) = (rect.x[3], rect.y[3]); // bottom right
    (x > left) && (x < right) && (y > bottom) && (y < top)
}
