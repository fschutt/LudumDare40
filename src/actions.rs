//! UI actions, invoked from the input / UI module

use glium::glutin::MouseCursor;

use ui::Ui;
use game::GameState;
use player_state::PlayerState;
use input::WindowState;

/// Returns if the UI is handling this event
pub fn start_game(window_state: &mut WindowState, ui: &Ui, game_state: &mut GameState) -> bool {

    match *game_state {
        GameState::StartMenu => {
            *game_state = GameState::Game(Box::new(PlayerState::default()));
            window_state.mouse_state.mouse_cursor_type = MouseCursor::Default;
        },
        _ => { }
    }

    true
}

/// Called when the mouse enters the "start game" button
pub fn start_game_enter(window_state: &mut WindowState, ui: &Ui, game_state: &mut GameState) -> bool {
    window_state.mouse_state.mouse_cursor_type = MouseCursor::Hand;
    true
}

/// Called when the mouse leaves the "start game" button
pub fn start_game_leave(window_state: &mut WindowState, ui: &Ui, game_state: &mut GameState) -> bool {
    window_state.mouse_state.mouse_cursor_type = MouseCursor::Default;

    true
}
