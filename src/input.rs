//! Input handling module, handles incoming events for the window

use glium;
use glium::glutin::MouseCursor;

use std::time::{Duration, Instant};
use ui::{UiRect, UiRendererData};

// Shortcuts, hard-coded
pub const SHORTCUT_MOVE_LEFT: KbShortcut = KbShortcut { modifier: None, key: 'd' };
pub const SHORTCUT_MOVE_RIGHT: KbShortcut = KbShortcut { modifier: None, key: 'a' };

// TODO: events (for the UI)
pub const MOUSE_MOVE_EVENT: &str = "mousemove";
pub const MOUSE_DOWN_EVENT: &str = "mousedown";
pub const MOUSE_UP_EVENT: &str = "mouseup";

#[derive(Debug, Copy, Clone)]
pub struct KbShortcut
{
    modifier: Option<ReducedKbModifier>,
    key: char,
}

/// Determines which keys are pressed currently (modifiers, etc.)
#[derive(Debug, Clone)]
pub struct KeyboardState
{
    /// Modifier keys that are currently actively pressed during this cycle
    pub modifiers: Vec<glium::glutin::VirtualKeyCode>,
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

/// Keyboard modifier key, reduced set suited for desktop UIs.
/// Handles things such as `AltGr` -> split into "Alt" and "Shift"
/// `RShift` and `LShift` are generalized to "Shift", same as Ctrl
/// Fn keys have a number attached to them
/// Other keys are ignored and forgotten
/// There may be problems if both Alt keys are pressed and then released
/// Therefore, keys that have a "right" and a "left" method have a number
/// attached to them, how many keys are currently pressed. Currently, this is
/// not in effect (too much work).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReducedKbModifier
{
    Fn(u8),   // Fn1, Fn2, etc.
    Function, // Function key modifier
    Alt,
    Shift,
    AltGr, // has to be seperate, same function as alt + shift
    Super, // "Super" or Windows key
    Ctrl,
    RightClickMenu,
    Tab,
    Esc,
    Del,    // "Entf" key
    Return, // Control character because of shift + return options
    Backspace,
    PgUp,
    PgDown,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    MicMute,
    TpVantange,
    Home,
    Pause,
    End,
    Roll,
    Insert,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
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
    //// Mulitplier for the scroll speed in x direction
    pub scroll_speed_x: f32,
    //// Mulitplier for the scroll speed in y direction
    pub scroll_speed_y: f32,
}

impl MouseState
{
    /// Creates a new mouse state
    /// Input: How fast the scroll (mouse) should be converted into pixels
    /// Usually around 10.0 (10 pixels per mouse wheel line)
    pub fn new(
        scroll_speed_x: f32,
        scroll_speed_y: f32,
    ) -> Self
    {
        MouseState {
            mouse_cursor_type: MouseCursor::Default,
            mouse_cursor: Some((0, 0)),
            left_down: false,
            right_down: false,
            middle_down: false,
            mouse_scroll_x: 0.0,
            mouse_scroll_y: 0.0,
            scroll_speed_x: scroll_speed_x,
            scroll_speed_y: scroll_speed_y,
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
    min_frame_time: Duration,
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
            mouse_state: MouseState::new(100.0, 100.0),
            width,
            height,
            time_of_last_update: Instant::now(),
            min_frame_time: Duration::from_millis(16),
        }
    }


    /// Handles the event, updates the UI, then returns if the window was not
    /// closed (false on closed)
    /// The second bool determines if the window should redraw itself
    #[inline]
    pub(crate) fn handle_event(
        &mut self,
        event: &glium::glutin::Event,
    ) -> (bool, bool)
    {
        // update the state of the input information
        use glium::glutin::Event::*;

        let mut should_redraw = true;

        // this will be true if the UI (drawn on top of the map) has already handled
        // the incoming event for example, we don't want a click on the UI to be
        // received as a click on the map
        let (ui_handles_event, update_renderer) = self.ui_handle_event(event);

        if !ui_handles_event {
            match *event {
                Closed => {
                    return (false, false);
                },
                MouseMoved(x, y) => {
                    should_redraw = self.handle_mouse_move(x, y);
                },
                MouseWheel(delta, phase) => {
                    should_redraw = self.handle_mouse_scroll(delta, phase);
                },
                KeyboardInput(state, code, vk_code) => {
                    self.handle_keyboard_input(state, code, vk_code);
                },
                ReceivedCharacter(c) => {
                    self.handle_received_character(c);
                },
                MouseInput(state, button) => {
                    self.handle_mouse_click(state, button);
                },
                Resized(width, height) => {
                    self.handle_resize(width, height);
                },
                Focused(b) => {
                    self.handle_focus(b);
                },
                _ => {},
            }
        }
        else {
            should_redraw = update_renderer;
        }

        // now that the state is updated, we have enough information to re-layout the
        // frame
        (true, should_redraw)
    }

    /// Handle the focus of a window
    #[inline]
    pub fn handle_focus(
        &mut self,
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
        width: u32,
        height: u32,
    )
    {
        self.width = width;
        self.height = height;
    }

    /// Updates mouse movement, returns if the screen needs to be redrawn
    #[inline]
    fn handle_mouse_move(
        &mut self,
        x: i32,
        y: i32,
    ) -> bool
    {
        if Instant::now().duration_since(self.time_of_last_update) < self.min_frame_time {
            return false;
        }

        let is_left_mouse_down = self.mouse_state.left_down;

        // no redraw needed if the mouse did not drag the map (todo: UI handling?)
        if !is_left_mouse_down {
            self.mouse_state.mouse_cursor = Some((x, y));
            return false;
        }

        // dragging action
        let cur_mouse_cursor = self.mouse_state.mouse_cursor;

        if cur_mouse_cursor.is_none() {
            self.mouse_state.mouse_cursor = Some((x, y));
            return true;
        }

        self.mouse_state.mouse_cursor = Some((x, y));
        true
    }


    /// Updates the mouse state on a click
    #[inline]
    fn handle_mouse_click(
        &mut self,
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

    /// Updates mouse scroll
    #[inline]
    fn handle_mouse_scroll(
        &mut self,
        delta: glium::glutin::MouseScrollDelta,
        _phase: glium::glutin::TouchPhase,
    ) -> bool
    {
        if Instant::now().duration_since(self.time_of_last_update) < self.min_frame_time
        {
            return false;
        }

        if let glium::glutin::MouseScrollDelta::LineDelta(_, y) = delta {
            /* handle scrolling*/
        }

        true
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
    fn handle_keyboard_input(
        &mut self,
        state: glium::glutin::ElementState,
        _code: u8,
        vk_code: Option<glium::glutin::VirtualKeyCode>,
    )
    {
        use glium::glutin::ElementState;

        if let Some(vk_code) = vk_code {
            if state == ElementState::Pressed {
                    self.keyboard_state.modifiers.push(vk_code);
            } else {
                let indices_found = self.keyboard_state.modifiers.iter().position(|e| *e == vk_code);
                if let Some(index) = indices_found {
                    self.keyboard_state.modifiers.remove(index);
                }
            }
        }
    }

    /// Handles character input (via string). Some characters are wrongly
    /// recognized as characters
    /// when in reality, they are control characters.
    #[inline]
    fn handle_received_character(
        &mut self,
        key: char,
    )
    {
        let keyboard = &mut self.keyboard_state;
        keyboard.hidden_keys.clear();
        keyboard.keys.clear();

        if keyboard.modifiers.is_empty() {
            // The key that is associated with the modifier key, so basically the "n" in
            // Ctrl + n
            let modifier_char_extra = key_to_character(key as u32);
            if let Some(hidden_key) = modifier_char_extra {
                // key is actually a ctrl + (cchar) key
                if !keyboard.hidden_keys.iter().any(|elem| *elem == hidden_key) {
                    keyboard.hidden_keys.push(hidden_key);
                }
            }
        }
        else {
            keyboard.keys.push(key);
        }
    }

    /// Parent function that handles any incoming UI event and delegates it to the
    /// rest of the UI
    /// handling functions.
    /// Returns (a, b) where
    /// - a: this function already handles the mouse event
    /// - b: the map needs to be redrawn
    #[inline]
    pub fn ui_handle_event(
        &mut self,
        event: &glium::glutin::Event,
    ) -> (bool, bool)
    {
        use glium::glutin::Event::*;

        let mut ui_is_already_handling_event = false;
        let mut ui_should_redraw = false;

        // for now the UI does not handle scrolling
        // or global hotkeys such as CTRL + S
        let (old_x, old_y) = self.mouse_state.mouse_cursor.unwrap_or((-1, -1));

        // for now the UI does not handle scrolling
        // or global hotkeys such as CTRL + S
        match *event {
            // MouseMoved is for hover events and focusin / focusout
            MouseMoved(x, y) => {
                // TODO: update x, y of self?
                ui_is_already_handling_event = self.ui_handle_mouse_move(x, y, old_x, old_y);
            },
            // MouseMoved is for onclick events, etc.
            MouseInput(state, button) => {
                ui_should_redraw = self.ui_handle_mouse_click(state, button);
            },
            _ => {},
        }

        (ui_is_already_handling_event, ui_should_redraw)
    }

    /// Checks if the mouse has moved over a UI element, calls the respective
    /// functions,
    /// returns false if the event should propagate to underlying elements.
    pub fn ui_handle_mouse_move(
        &mut self,
        x: i32,
        y: i32,
        old_x: i32,
        old_y: i32,
    ) -> bool
    {/*
        let user_interfaces = renderer.user_interfaces.borrow();
        let ui_id = if let Some(ui_id) = renderer.current_ui {
            ui_id
        }
        else {
            return false;
        };
        let ui = if let Some(ui) = user_interfaces.get(ui_id) {
            ui
        }
        else {
            return false;
        };

        for child in ui.root.children() {
            check_available_actions(&child, renderer, x, y, old_x, old_y, MOUSE_MOVE_EVENT);
        }
    */
        true
    }

    /// Checks if any element from the currently active UI was clicked, calls the
    /// respective onmousedown and onmouseup functions.
    /// Returns if the screen should be updated.
    pub fn ui_handle_mouse_click(
        &mut self,
        state: glium::glutin::ElementState,
        _button: glium::glutin::MouseButton,
    ) -> bool
    {
        /*

        use glium::glutin::ElementState::*;

        let user_interfaces = renderer.user_interfaces.borrow();
        let ui_id = if let Some(ui_id) = renderer.current_ui {
            ui_id
        }
        else {
            return false;
        };
        let ui = if let Some(ui) = user_interfaces.get(ui_id) {
            ui
        }
        else {
            return false;
        };

        let window_state = renderer.window_state.borrow();

        let mut should_update_screen = Vec::<bool>::new();

        if let Some(cursor) = window_state.mouse_state.mouse_cursor {
            match state {
                Pressed => for child in ui.root.children() {
                    should_update_screen.push(check_available_actions(
                        &child,
                        renderer,
                        cursor.0,
                        cursor.1,
                        cursor.0,
                        cursor.1,
                        MOUSE_DOWN_EVENT
                    ));
                },
                Released => for child in ui.root.children() {
                    should_update_screen.push(check_available_actions(
                        &child,
                        renderer,
                        cursor.0,
                        cursor.1,
                        cursor.0,
                        cursor.1,
                        MOUSE_UP_EVENT
                    ));
                },
            }
        }

        should_update_screen.iter().any(|e| *e)
        */
        true
    }
}

/// This function only returns valid results if there is a control character
/// pressed at the
/// same time. `glutin` will return characters such as `\u{32}` which are not
/// helpful at all.
///
#[inline]
fn key_to_character(key: u32) -> Option<char>
{
    match key {
        25 => Some('y'),
        24 => Some('x'),
        22 => Some('v'),
        3 => Some('c'),
        13 => Some('m'),
        2 => Some('b'),
        14 => Some('n'),
        1 => Some('a'),
        19 => Some('s'),
        4 => Some('d'),
        6 => Some('f'),
        7 => Some('g'),
        8 => Some('h'),
        10 => Some('j'),
        11 => Some('k'),
        12 => Some('l'),
        39 |
        45 |
        59 => Some('Ã'),
        /* 27  => { Some('Ã')}, // same as Esc?? */
        16 => Some('p'),
        15 => Some('o'),
        9 => Some('i'),
        21 => Some('u'),
        26 => Some('z'),
        20 => Some('t'),
        18 => Some('r'),
        5 => Some('e'),
        23 => Some('w'),
        17 => Some('q'),
        0 => Some('2'),
        27 => Some('3'),
        28 => Some('4'),
        29 => Some('5'),
        30 => Some('6'),
        31 => Some('7'),
        32 => Some('8'),
        33 => Some('9'),
        _ => None,
    }
}


/*
/// Returns if the screen should be updated
///
/// "event" can currently be: m
fn check_available_actions(current: &NodeRef<Rect<UiRendererData>>,
                           renderer: &Renderer,
                           x: i32,
                           y: i32,
                           old_x: i32,
                           old_y: i32,
                           event: &'static str)
                           -> bool
{
    let mut should_update_screen = Vec::<bool>::new();

    for child in current.children() {
        let previous_point_in_rect = check_point_in_rect(old_x as f32, old_y as f32, &*child.borrow());
        let current_point_in_rect = check_point_in_rect(x as f32, y as f32, &*child.borrow());

        match event {
            MOUSE_UP_EVENT   => {
                if !current_point_in_rect { continue; }
                if let Some(fptr) = child.borrow().data.data.actions.onmouseup {
                    should_update_screen.push((fptr)(renderer));
                }
            },
            MOUSE_DOWN_EVENT => {
                if !current_point_in_rect { continue; }
                if let Some(fptr) = child.borrow().data.data.actions.onmousedown {
                    should_update_screen.push((fptr)(renderer));
                }
            },
            MOUSE_MOVE_EVENT => {
                // choose between mouseenter and mouseleave
                if current_point_in_rect != previous_point_in_rect {
                    if current_point_in_rect {
                        // call mouseenter event
                        if let Some(fptr) = child.borrow().data.data.actions.onmouseenter {
                            should_update_screen.push((fptr)(renderer));
                        }
                    } else {
                        // call mouseleave event
                        if let Some(fptr) = child.borrow().data.data.actions.onmouseleave {
                            should_update_screen.push((fptr)(renderer));
                        }
                    }
                }
            },
            _ => { },
        }

        if child.borrow().data.data.actions.propagate_underlying {
            check_available_actions(&child, renderer, x, y, old_x, old_y, event);
        }
    }

    should_update_screen.iter().any(|e| *e)
}
*/

#[inline]
pub fn check_point_in_rect(
    x: f32,
    y: f32,
    rect: &UiRect<UiRendererData>,
) -> bool
{
    let min = (rect.x[0], rect.y[0]);
    let max = (rect.x[3], rect.y[3]);
    (x > min.0) && (x < max.0) && (y > min.1) && (y < max.1)
}
