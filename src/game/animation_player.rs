use std::time::Duration;
use cgmath::Vector3;
use crate::renderer::instance::Instance;
use crate::renderer::texture_manager::Atlas;
use std::collections::HashMap;
use crate::game::entity::Entity;

pub struct AnimationPlayer {
    pub playing: bool,
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
}

pub struct AnimationSheet {
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_order: Vec<usize>,
    pub frame_duration: Duration,
    pub atlas: Atlas,
    pub looped: bool,
}

pub struct Animation {
    pub frames: Vec<Frame>,
    pub frame_duration: Duration,
    pub current_frame: usize,
    pub time_accumulator: Duration,
    pub looped: bool,
    pub frame_order: Vec<usize>,
    pub instances: Vec<Instance>,
    pub frame_width: u32,
    pub position: Vector3<f32>,
}

impl Entity for Animation {
    fn position(&self) -> Vector3<f32> {
        self.position
    }

    fn instances(&self) -> &[Instance] {
        &self.instances
    }
}

impl Animation {
    pub fn new(
        position: Vector3<f32>,
        animation_sheet: &AnimationSheet,
        selection_x: u32,
        selection_y: u32,
        selection_w: u32,
        selection_h: u32,
    ) -> Self {
        let mut frames = Vec::new();

        let AnimationSheet {
            frame_width,
            frame_height,
            frame_order,
            frame_duration,
            atlas,
            looped,
        } = animation_sheet;

        // Divide the atlas selection into frames
        for i in 0..selection_h / frame_height {
            for j in 0..selection_w / frame_width {
                let mut instance_indices = Vec::new();
                for h in 0..*frame_height {
                    for w in 0..*frame_width {
                        let x = selection_x + j * frame_width + w;
                        let y = selection_y + i * frame_height + h;

                        let index = x + y * atlas.cols;
                        instance_indices.push(index);
                    }
                }
                frames.push(Frame {
                    instance_indices,
                });
            }
        }

        let mut instances = Vec::new();

        let len = frame_width * frame_height - 1;
        for h in 0..*frame_height {
            for w in 0..*frame_width {
                let xo = selection_x + w;
                let yo = selection_y + h;

                let x = position.x;
                let y = position.y - h as f32 + len as f32;

                let index = xo + yo * atlas.cols + frame_order[0] as u32;

                instances.push(Instance {
                    model: cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, y, 0.0)).into(),
                    tex_index: index,
                    atlas_index: atlas.index,
                });
            }
        }

        Self {
            frames,
            frame_duration: *frame_duration,
            current_frame: 0,
            time_accumulator: Duration::new(0, 0),
            looped: *looped,
            frame_order: frame_order.to_vec(),
            instances,
            frame_width: *frame_width,
            position,
        }
    }

    pub fn update(&mut self, position: Vector3<f32>, dt: Duration) -> bool {

        if !self.looped && self.current_frame == self.frame_order.len() - 1 {
            return true;
        }

        self.time_accumulator += dt;
        if self.time_accumulator >= self.frame_duration {
            self.time_accumulator -= self.frame_duration; // Subtract to prevent frame skip
            self.current_frame += 1;
            if self.current_frame >= self.frame_order.len() { // Changed to >= to avoid out-of-bounds
                self.current_frame = 0;
            }

            let new_frame = &self.frames[self.frame_order[self.current_frame]];

            for (i, instance) in self.instances.iter_mut().enumerate() {
                if i < new_frame.instance_indices.len() {
                    instance.tex_index = new_frame.instance_indices[i];
                }
            }
        }

        // Loop over frames with index
        let len = self.instances.len() - 1;
        for (i, instance) in self.instances.iter_mut().enumerate() {
            let xo = i % self.frame_width as usize;
            let yo = i / self.frame_width as usize;

            let x = position.x + xo as f32;
            let y = position.y - yo as f32 + len as f32;

            instance.model = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, y, 0.0)).into();
        }

        self.position = position;

        return false;
    }

    pub fn go_to_first_frame(&mut self) {
        self.current_frame = 0;
        self.time_accumulator = Duration::new(0, 0);

        let frame = &self.frames[self.frame_order[self.current_frame]];
        for (i, instance) in self.instances.iter_mut().enumerate() {
            if i < frame.instance_indices.len() {
                instance.tex_index = frame.instance_indices[i];
            }
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub instance_indices: Vec<u32>,
}

impl AnimationPlayer {
    pub fn new(animations: HashMap<String, Animation>, current_animation: String, playing: bool) -> Self {
        Self {
            animations,
            playing,
            current_animation,
        }
    }

    pub fn update(&mut self, position: Vector3<f32>, dt: Duration) {
        if self.playing {
            self.animations
                .get_mut(&self.current_animation)
                .unwrap()
                .update(position, dt);
        }
    }

    pub fn get_instances(&self) -> &Vec<Instance> {
        self.animations
            .get(&self.current_animation)
            .unwrap()
            .instances
            .as_ref()
    }

    pub fn get_current_frame(&self) -> &Vec<Instance> {
        self.animations
            .get(&self.current_animation)
            .unwrap()
            .instances
            .as_ref()
    }

    pub fn set_duration(&mut self, speed: Duration) {
        self.animations
            .get_mut(&self.current_animation)
            .unwrap()
            .frame_duration = speed;
    }

    pub fn start(&mut self) {
        if !self.playing {
            self.playing = true;
            self.animations
                .get_mut(&self.current_animation)
                .unwrap()
                .go_to_first_frame();
        }
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.animations
            .get_mut(&self.current_animation)
            .unwrap()
            .go_to_first_frame();
    }
}
