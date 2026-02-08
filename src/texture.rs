use crate::utilities::*;
use stb_image;
use std::path::Path;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    to_argb(
                        255,
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
            }
        } else {
            panic!("Unsupported texture type");
        }
    }

    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        // Claude Sonnet 4.5 improved
        // glTF default sampler wrapping is REPEAT. Using rem_euclid keeps negatives sane too.
        let u = u.rem_euclid(1.0);
        let v = v.rem_euclid(1.0);

        let u = u * (self.width - 1) as f32;
        let v = v * (self.height - 1) as f32;

        let x = u as usize;
        let y = v as usize;

        let id = coords_to_index(x, y, self.width);

        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb(255, 255, 0, 255)
        }
    }
}
