#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub fn get_quad() -> (Vec<Vertex>, Vec<u16>) {

    let top_left = Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.001, 0.001],
    };

    let bottom_left = Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.001, 0.999],
    };

    let bottom_right = Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [0.999, 0.999],
    };

    let top_right = Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [0.999, 0.001],
    };

    let vertices = vec![top_left, bottom_left, bottom_right, top_right];

    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

    (vertices, indices)
}
