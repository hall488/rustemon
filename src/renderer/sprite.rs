use super::instance::Instance;
use cgmath::{Matrix4, Vector3};

pub struct Sprite {
    pub texture: Vec<Instance>,
    pub x: f32,
    pub y: f32,
    pub tex_x: u32,
    pub tex_y: u32,
    pub tex_w: u32,
    pub tex_h: u32,
    pub atlas_index: u32,
    pub atlas_w: u32,
    pub atlas_h: u32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Sprite {

    pub fn new(
        x: f32,
        y: f32,
        tex_x: u32,
        tex_y: u32,
        tex_w: u32,
        tex_h: u32,
        atlas_index: u32,
        atlas_w: u32,
        atlas_h: u32,
        scale_x: f32,
        scale_y: f32
    ) -> Self {

        //scale is set to scale / atlas size
        let scale = cgmath::Matrix4::from_nonuniform_scale(scale_x / atlas_w as f32, scale_y / atlas_h as f32, 1.0);

        let mut texture = Vec::new();

        for tx in tex_x..tex_x + tex_w {
            for ty in tex_y..tex_y + tex_h {
                let tx_o = tx - tex_x;
                let ty_o = ty - tex_y;

                //calculates where each tile needs to be relative to the top left
                let vector = cgmath::Vector3::new(x + tx_o as f32 * scale_x / atlas_w as f32 - 1.0 + scale_x / atlas_w as f32 / 2.0, 1.0 - y - ty_o as f32 * scale_y / atlas_h as f32 - scale_y / atlas_h as f32 / 2.0, 0.0);
                let model = (cgmath::Matrix4::from_translation(vector) * scale).into();
                let tex_index = tx + ty * atlas_w;

                let instance = Instance {
                    model,
                    tex_index,
                    atlas_index,
                };

                texture.push(instance);
            }
        }

        Self {
            texture,
            x,
            y,
            tex_x,
            tex_y,
            tex_w,
            tex_h,
            atlas_index,
            atlas_w,
            atlas_h,
            scale_x,
            scale_y,
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;

        let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale_x / self.atlas_w as f32, self.scale_y / self.atlas_h as f32, 1.0);

        for (i, instance) in self.texture.iter_mut().enumerate() {
            let tx = i as u32 % self.tex_w + self.tex_x;
            let ty = i as u32 / self.tex_w + self.tex_y;

            let tx_o = tx - self.tex_x;
            let ty_o = ty - self.tex_y;

            let vector = cgmath::Vector3::new(self.x + tx_o as f32 * self.scale_x / self.atlas_w as f32 - 1.0 + self.scale_x / self.atlas_w as f32 / 2.0, 1.0 - self.y - ty_o as f32 * self.scale_y / self.atlas_h as f32 - self.scale_y / self.atlas_h as f32 / 2.0, 0.0);
            let model = (cgmath::Matrix4::from_translation(vector) * scale).into();

            instance.model = model;
        }
    }
}
