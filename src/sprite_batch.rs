use sfml::graphics::{VertexArray, Vertex, PrimitiveType, RenderTarget, RenderStates, Texture};
use sfml::system::{Vector2f, SfBox};

#[derive(Copy, Clone, Default)]
pub struct TextureRegion {
    pub x : i32,
    pub y : i32,
    pub w : u32,
    pub h : u32,
}

impl TextureRegion{
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self{x, y, w, h}
    }
}

pub struct SpriteBatch{
    vertex_array : VertexArray,
    texture: SfBox<Texture>,
}


impl SpriteBatch{
    pub fn new(sheet: &str) -> Self {
        let mut vertex_array = VertexArray::default();
        vertex_array.set_primitive_type(PrimitiveType::Quads);
        Self{
            vertex_array,
            texture: Texture::from_file(sheet).unwrap(),
        }
    }

    pub fn clear(&mut self) {
        self.vertex_array.clear();
    }

    pub fn add(&mut self, x: f32, y: f32, w: u32, h: u32, region: TextureRegion) {
        let pos = Vector2f::new(x, y);
        let tex_coord = Vector2f::new(region.x as f32, region.y as f32);
        self.vertex_array.append(&Vertex::with_pos_coords(pos, tex_coord));
    
        let pos = Vector2f::new(x + w as f32, y);
        let tex_coord = Vector2f::new(region.x as f32 + region.w as f32, region.y as f32);
        self.vertex_array.append(&Vertex::with_pos_coords(pos, tex_coord));

        let pos = Vector2f::new(x + w as f32, y + h as f32);
        let tex_coord = Vector2f::new(region.x as f32 + region.w as f32, region.y as f32 + region.h as f32);
        self.vertex_array.append(&Vertex::with_pos_coords(pos, tex_coord));

        let pos = Vector2f::new(x, y + h as f32);
        let tex_coord = Vector2f::new(region.x as f32, region.y as f32 + region.h as f32);
        self.vertex_array.append(&Vertex::with_pos_coords(pos, tex_coord));
    }

    pub fn display(&mut self, window: &sfml::graphics::RenderWindow) {
        let mut states = RenderStates::default();
        states.texture = Some(&self.texture);
        window.draw_vertex_array(&self.vertex_array, states);
        self.clear();
    }
}