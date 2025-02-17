use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;
use crate::renderer::instance::Instance;

pub struct Font {
    pub sprites: Vec<Sprite>,
}

impl Font {

    pub fn new(
        x: f32,
        y: f32,
        text: &str,
        left: bool,
        font_type: &str,
        renderer: &Renderer,
    ) -> Self {

        let mut sprites = Vec::new();

        for (i, c) in text.char_indices() {

            let c = match left {
                true => c,
                false => text.chars().rev().nth(i).unwrap(),
            };

            let tex_index = match c {
                'A' => 0,  'a' => 28, '0' => 56,
                'B' => 1,  'b' => 29, '1' => 57,
                'C' => 2,  'c' => 30, '2' => 58,
                'D' => 3,  'd' => 31, '3' => 59,
                'E' => 4,  'e' => 32, '4' => 60,
                'F' => 5,  'f' => 33, '5' => 61,
                'G' => 6,  'g' => 34, '6' => 62,
                'H' => 7,  'h' => 35, '7' => 63,
                'I' => 8,  'i' => 36, '8' => 64,
                'J' => 9,  'j' => 37, '9' => 65,
                'K' => 10, 'k' => 38, ' ' => 66,
                'L' => 11, 'l' => 39, '?' => 67,
                'M' => 12, 'm' => 40, '/' => 88,
                'N' => 13, 'n' => 41, '-' => 89,
                'O' => 14, 'o' => 42,
                'P' => 15, 'p' => 43,
                'Q' => 16, 'q' => 44,
                'R' => 17, 'r' => 45,
                'S' => 18, 's' => 46,
                'T' => 19, 't' => 47,
                'U' => 20, 'u' => 48,
                'V' => 21, 'v' => 49,
                'W' => 22, 'w' => 50,
                'X' => 23, 'x' => 51,
                'Y' => 24, 'y' => 52,
                'Z' => 25, 'z' => 53,
                '.' => 26,
                ',' => 27,

                _ => 0,
            };

            let pos_x = if left {
                x + i as f32 * 5.0
            } else {
                x - i as f32 * 5.0
            };

            let pos_y = y;
            let tex_x = tex_index % 28;
            let tex_y = tex_index / 28;
            let tex_w = 1;
            let tex_h = 1;

            let sprite = renderer.create_sprite(pos_x, pos_y, tex_x, tex_y, tex_w, tex_h, font_type, 1.0, 1.0).expect("");

            sprites.push(sprite);
        }

        Self {
            sprites,
        }
    }

    pub fn instanced(&self) -> Vec<Instance> {
        let mut instances = Vec::new();
        for letter in &self.sprites {
            instances.extend_from_slice(&letter.texture);
        }
        return instances;
    }
}
