use glam::{Vec2, Vec3};
use crate::Path;
use crate::MeshRenderer;

pub fn edge_function(v0: Vec2, v1: Vec2, point: Vec2) -> f32
{
    (point.x - v0.x) * (v1.y - v0.y) - (point.y - v0.y) * (v1.x - v0.x)
}

pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let m0 = edge_function(point, v1, v2);
    let m1 = edge_function(point, v2, v0);
    let m2 = edge_function(point, v0, v1);
    // instead of 3 divisions we can do 1/area *
    let a = 1.0 / area;
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

pub fn to_argb(a: u8, r: u8, g: u8, b: u8) -> u32
{
    let mut color: u32 = a as u32;
    color = (color << 8) + r as u32;
    color = (color << 8) + g as u32;
    color = (color << 8) + b as u32;
    color
}

pub fn coords_to_index(u: usize, v: usize, width: usize) -> usize
{
    u + v * width
}

// ASK ABOUT THIS
// NDC to map to screen
pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}

// NOTE: learn more about templates
pub fn clear_buffer<T>(buffer: &mut Vec<T>, value: T)
where
    T: Copy,
{
    // will "consume" the iterator and return the n of iterations
    buffer.iter_mut().map(|x| *x = value).count();
}

pub fn load_gltf(path: &Path) -> MeshRenderer {
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() {
        for node in scene.nodes() {
            if let Some(mesh) = node.mesh() {
                return MeshRenderer::load_from_gltf(&mesh, &buffers);
            }
        }
    }

    MeshRenderer::new()
}

// Credit: Claude Sonnet 4.5
// Helper function to convert your u32 ARGB format to Bevy's RGBA8 byte format
fn convert_framebuffer_to_image(framebuffer: &[u32], image_data: &mut [u8]) {
    for (i, &pixel) in framebuffer.iter().enumerate() {
        // Your pixel is packed as: (alpha << 24) | (red << 16) | (green << 8) | blue
        // Extract each component
        let a = ((pixel >> 24) & 0xFF) as u8;
        let r = ((pixel >> 16) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel & 0xFF) as u8;
        
        // Bevy expects RGBA format: [r, g, b, a] as sequential bytes
        let byte_index = i * 4;
        image_data[byte_index] = r;
        image_data[byte_index + 1] = g;
        image_data[byte_index + 2] = b;
        image_data[byte_index + 3] = a;
    }
}