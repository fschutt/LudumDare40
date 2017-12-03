use FastHashMap;
use std::io::{BufRead, Seek};
use glium::texture::Texture2d;
use glium::backend::{Context, Facade};
use glium::texture::RawImage2d;
use glium::{DrawParameters, VertexBuffer, Program, Frame};
use ui::UiRect;
use std::rc::Rc;
use ShaderHashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TextureId {
    pub texture_id: &'static str
}

#[derive(Default)]
pub struct TextureSystem {
    // Images used by the renderer
    pub textures: FastHashMap<TextureId, Texture2d>,
}

/// Width, height and offsets into the texture
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourcePixelRegion {
    pub bottom_x: u32,
    pub bottom_y: u32,
    pub width: u32,
    pub height: u32,
}

/// Where the texture region should be draw on the screen
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TargetPixelRegion {
    pub screen_bottom_x: u32,
    pub screen_bottom_y: u32,
    pub screen_width: u32,
    pub screen_height: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SourceTextureRegion {
    /// Texture ID for looking it up in the TextureSystem at runtime
    pub texture_id: TextureId,
    /// Region of the texture that should be drawn (i.e.)
    pub region: SourcePixelRegion,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextureInstanceId {
    pub source_texture_region: SourceTextureRegion,
    pub target_texture_region: TargetPixelRegion,
}

#[derive(Copy, Clone)]
pub struct PixelScreenVert {
    pub position: [f32;3],
    pub tex_coords: [f32;2],
}

implement_vertex!(PixelScreenVert, position, tex_coords);

impl TextureSystem {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_png_texture<R, F>(&mut self, id: &'static str, source: R, display: &F)
        -> TextureId
        where R: BufRead + Seek, F: Facade
    {
        use image;
        let image = image::load(source, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let opengl_texture = Texture2d::new(display, image).unwrap();

        let id = TextureId { texture_id: id };
        self.textures.insert(id.clone(), opengl_texture);
        id
    }

    // TODO: group textures by texture_id.source_texture_region.texture_id
    pub fn draw_texture(&self, frame: &mut Frame, display: &Rc<Context>,
        texture_id: &TextureInstanceId, transparency: f32, shaders: &ShaderHashMap)
    {
        use glium::{Surface, Blend, Depth};
        use glium::draw_parameters::DepthTest;
        use glium::draw_parameters::DepthClamp;

        let shader = shaders.get(::context::PIXEL_TO_SCREEN_SHADER_ID).unwrap();
        let texture = self.textures.get(&texture_id.source_texture_region.texture_id).unwrap();
        let (t_w, t_h) = texture.dimensions();
        let source_tr = &texture_id.source_texture_region.region;
        let target_tr = &texture_id.target_texture_region;

        let z = 0.1_f32;
        let top_left = PixelScreenVert {
            position:   [ target_tr.screen_bottom_x as f32,
                         (target_tr.screen_bottom_y + target_tr.screen_height) as f32,
                         z],
            tex_coords: [
                        (( source_tr.bottom_x as f32 / t_w as f32)),
                        (((source_tr.bottom_y + source_tr.height) as f32 / t_h as f32))
                        ],
        };

        let top_right = PixelScreenVert {
            position:   [(target_tr.screen_bottom_x + target_tr.screen_width) as f32,
                         (target_tr.screen_bottom_y + target_tr.screen_height) as f32,
                         z],
            tex_coords: [
                        (((source_tr.bottom_x + source_tr.width) as f32 / t_w as f32)),
                        (((source_tr.bottom_y + source_tr.height) as f32 / t_h as f32))
                        ],
        };

        let bottom_left = PixelScreenVert {
            position:   [ target_tr.screen_bottom_x as f32,
                          target_tr.screen_bottom_y as f32,
                          z],
            tex_coords: [
                        (( source_tr.bottom_x as f32 / t_w as f32)),
                        (( source_tr.bottom_y as f32 / t_h as f32))
                        ],
        };

        let bottom_right = PixelScreenVert {
            position:   [(target_tr.screen_bottom_x + target_tr.screen_width) as f32,
                          target_tr.screen_bottom_y as f32,
                          z],
            tex_coords: [
                        (((source_tr.bottom_x + source_tr.width) as f32 / t_w as f32)),
                        (( source_tr.bottom_y as f32 / t_h as f32))
                        ],
        };

        let (w, h) = frame.get_dimensions();

        let uniforms = uniform!(
            window_width: w as f32,
            window_height: h as f32,
            transparency: transparency,
            tex: texture,
        );

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                range: (0.0, 1.0),
                clamp: DepthClamp::NoClamp,
            },
            .. Default::default()
        };

        let vertex_buf = [bottom_left, top_left, bottom_right, top_right];
        let vbuf = VertexBuffer::new(display, &vertex_buf).unwrap();
        frame.draw(&vbuf, ::context::NO_INDICES_BUFFER_TRIANGLE, shader, &uniforms, &DrawParameters::default()).unwrap();
    }
}

