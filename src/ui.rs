//! UI rendering with rectangles - either has a solid color or an image attached
use color::Color;
use game::GameState;
use glium::Texture2d;
use font::FontInstanceId;
use std::rc::Rc;
use std::cell::RefCell;
use texture::TextureInstanceId;

pub struct UiRect<T> {
    pub x: [f32;4],
    pub y: [f32;4],
    pub data: Box<T>,
}

/// The actual user interface is just a bunch of rectangles
/// The renctangles get mapped into screen space later on
pub struct Ui {
    pub rectangles: Vec<UiRect<UiRendererData>>,
}


#[derive(Clone)]
pub struct UiRendererData
{
    /// If the rectangle should have a background color
    pub color: Option<Color>,
    /// Image type (png, tiff, raw)
    pub image: Option<TextureInstanceId>,
    /// Optional text, font-face, size + position
    pub text: Option<(String, FontInstanceId)>,
    /// Actions associated with this rectangle
    pub actions: UiActions,
}

impl UiRendererData
{
    /// Creates an invisible rectangle
    pub fn empty() -> Self
    {
        Self {
            color: None,
            image: None,
            text: None,
            actions: UiActions::empty(),
        }
    }
}

/// A list of function pointers that get called by the `ui_handle_event()`
/// function
/// Each function should return if the renderer needs to be updated / redrawn
#[derive(Clone)]
pub struct UiActions
{
    /// What to do on a mouseup event
    pub onmouseup: Option<fn(&mut GameState) -> bool>,
    /// onmouseenter event: Mouse has entered the current rectangle
    pub onmouseenter: Option<fn(&mut GameState) -> bool>,
    /// onmouseleave event: Mouse has left the current rectangle
    pub onmouseleave: Option<fn(&mut GameState) -> bool>,
}

impl UiActions
{
    pub fn empty() -> Self
    {
        Self {
            onmouseup: None,
            onmouseenter: None,
            onmouseleave: None,
        }
    }
}
