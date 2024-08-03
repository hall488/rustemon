use cgmath::{ortho, Matrix4, Point3, Vector3};
use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{PhysicalKey, KeyCode};

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub znear: f32,
    pub zfar: f32,
    pub aspect: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = ortho(self.left * self.aspect, self.right * self.aspect, self.bottom, self.top, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ConfigUniform {
    pub apply_camera: u32,
    pub _padding: u32,
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_z_pressed: bool,
    is_x_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_z_pressed: false,
            is_x_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &KeyEvent) -> bool {

        let keycode = match event.physical_key {
            PhysicalKey::Code(keycode) => keycode,
            PhysicalKey::Unidentified(_) => return false,
        };

        let state = &event.state;

        let is_pressed = *state == ElementState::Pressed;
        match keycode {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            KeyCode::KeyZ => {
                self.is_z_pressed = is_pressed;
                true
            }
            KeyCode::KeyX => {
                self.is_x_pressed = is_pressed;
                true
            }
            _ => false,
        }

    }

    pub fn update_camera(&self, camera: &mut Camera, position: Vector3<f32>) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_z_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_x_pressed{
            camera.eye -= forward_norm * self.speed;
        }

        camera.target = Point3::new(position.x, position.y, camera.target.z).into();
        camera.eye = Point3::new(position.x, position.y, camera.eye.z).into();

    }


}
