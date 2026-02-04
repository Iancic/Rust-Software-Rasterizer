use glam::{UVec3, Vec2, Vec3, Vec4, Mat4};
use std::ops::{Add, AddAssign, MulAssign, Sub, Mul};
use crate::texture::*;
use crate::utilities::*;

#[derive(Debug, Copy, Clone)]
pub struct Vertex
{
    pub position: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
        }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn triangles(&self) -> &Vec<UVec3> {
        &self.triangles
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn get_vertices_from_triangle(&self, triangle: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.x as usize],
            &self.vertices[triangle.y as usize],
            &self.vertices[triangle.z as usize],
        ]
    }

    pub fn from_vertices(triangles: &[UVec3], vertices: &[Vertex]) -> Self {
        let mut mesh = Mesh::new();
        mesh.add_section_from_vertices(triangles, vertices);
        mesh
    }

    // we can also do it with slices
    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let triangles: Vec<UVec3> = triangles.iter().map(|tri| *tri + offset).collect();
        self.triangles.extend_from_slice(&triangles);
        self.vertices.extend_from_slice(vertices);
    }
}

// for more on struct initialization check Default trait
impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Mesh {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self::from_vertices(self.triangles(), self.vertices());
        result.add_section_from_vertices(rhs.triangles(), rhs.vertices());
        result
    }
}

impl AddAssign for Mesh {
    fn add_assign(&mut self, rhs: Self) {
        self.add_section_from_vertices(rhs.triangles(), rhs.vertices());
    }
}


pub fn raster_triangle(
    vertices: &[&Vertex; 3],
    mvp: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    let clip0 = *mvp * Vec4::from((vertices[0].position, 1.0));
    let clip1 = *mvp * Vec4::from((vertices[1].position, 1.0));
    let clip2 = *mvp * Vec4::from((vertices[2].position, 1.0));

    let rec0 = 1.0 / clip0.w;
    let rec1 = 1.0 / clip1.w;
    let rec2 = 1.0 / clip2.w;

    // This would be the output of the vertex shader (clip space)
    // then we perform perspective division to transform in ndc
    // now x,y,z componend of ndc are between -1 and 1
    let ndc0 = clip0 * rec0;
    let ndc1 = clip1 * rec1;
    let ndc2 = clip2 * rec2;

    // perspective division on all attributes
    let v0 = *vertices[0] * rec0;
    let v1 = *vertices[1] * rec1;
    let v2 = *vertices[2] * rec2;

    // screeen coordinates remapped to window
    let sc0 = glam::vec2(
        map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
    );
    let sc1 = glam::vec2(
        map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
    );
    let sc2 = glam::vec2(
        map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(-ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
    );

    for (i, pixel) in buffer.iter_mut().enumerate() {
        let coords = index_to_coords(i, viewport_size.y as usize);
        // center of the pixel
        let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

        let area = edge_function(sc0, sc1, sc2);

        if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
            //interpolated 1/z(w)
            let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
            let depth = correction;
            // 1/(1/z) = z
            let correction = 1.0 / correction;

            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                // Color using vertex colors
                let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                let color = color * correction;
                let color = to_argb(
                    255,
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                );
                /*
                // Otherwise using texture
                if let Some(tex) = texture {
                    let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                    let tex_coords = tex_coords * correction;

                    color = tex.argb_at_uv(tex_coords.x, tex_coords.y);
                }
                color = to_argb(
                    255,
                    (depth * 255.0) as u8,
                    (depth * 255.0) as u8,
                    (depth * 255.0) as u8,
                );
                 */
                *pixel = color;
            }
        }
    }
}

pub fn raster_mesh(
    mesh: &Mesh,
    mvp: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    for triangle in mesh.triangles() {
        let vertices = mesh.get_vertices_from_triangle(*triangle);
        raster_triangle(&vertices, mvp, texture, buffer, z_buffer, viewport_size);
    }
}