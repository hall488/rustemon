pub fn orthographic_projection(width: f32, height: f32) -> [[f32; 4]; 4] {
    [
        [2.0 / width, 0.0, 0.0, 0.0],
        [0.0, 2.0 / height, 0.0, 0.0],
        [0.0, 0.0, -1.0, 0.0],
        [-1.0, 1.0, 0.0, 1.0], // Move the projection back along the z-axis
    ]
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WindowMapUniform {
    ortho_proj: [[f32; 4]; 4],
}

impl WindowMapUniform {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            ortho_proj: orthographic_projection(width, height),
        }
    }

    pub fn update_ortho_proj(&mut self, width: f32, height: f32) {
        self.ortho_proj = orthographic_projection(width, height);
    }
}
