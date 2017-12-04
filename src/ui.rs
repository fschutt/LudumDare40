//! UI rendering with rectangles - either has a solid color or an image attached
use color::Color;
use game::GameState;
use glium::texture::CompressedSrgbTexture2d;
use font::FontInstanceId;
use input::WindowState;
use std::rc::Rc;
use std::cell::RefCell;
use texture::TextureInstanceId;

#[derive(Clone)]
pub struct UiRect<T: Clone> {
    pub x: [f32;4],
    pub y: [f32;4],
    pub data: Box<T>,
}

/// The actual user interface is just a bunch of rectangles
/// The renctangles get mapped into screen space later on
#[derive(Default, Clone)]
pub struct Ui {
    pub rectangles: Vec<UiRect<UiRendererData>>,
}

impl Ui {
    // Returns a mutable renference if the tag (the ID)
    pub fn get_mut_rect_by_tag<'a>(&'a mut self, tag: &'static str) -> &'a mut UiRect<UiRendererData> {
        self.rectangles.iter_mut().filter(|rect| rect.data.tag == Some(tag)).next().unwrap()
    }
}

#[derive(Default, Clone)]
pub struct UiRendererData
{
    /// "Tag", like an HTML ID. Used to identify the Rectangle
    pub tag: Option<&'static str>,
    /// If the rectangle should have a background color
    pub color: Option<Color>,
    /// Image type (png, tiff, raw)
    pub image: Option<TextureInstanceId>,
    /// Optional text, font-face, size + position
    pub text: Option<(String, FontInstanceId)>,
    /// Actions associated with this rectangle
    pub actions: UiActions,
}

/// A list of function pointers that get called by the `ui_handle_event()`
/// function
/// Each function should return if the renderer needs to be updated / redrawn
#[derive(Default, Clone)]
pub struct UiActions
{
    /// What to do on a mouseup event
    pub onmouseup: Option<fn(&mut WindowState, &Ui, &mut GameState) -> bool>,
    /// onmouseenter event: Mouse has entered the current rectangle
    pub onmouseenter: Option<fn(&mut WindowState, &Ui, &mut GameState) -> bool>,
    /// onmouseleave event: Mouse has left the current rectangle
    pub onmouseleave: Option<fn(&mut WindowState, &Ui, &mut GameState) -> bool>,
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
