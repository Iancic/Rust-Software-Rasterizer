use glam::{UVec3, Vec2, Vec3, Vec4, Mat4};
use std::ops::{Add, AddAssign, MulAssign, Sub, Mul};
use crate::texture::*;
use crate::utilities::*;

#[derive(Debug, Copy, Clone)]
pub struct Vertex
{
    pub position: Vec4,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(position: Vec4, normal: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            color,
            uv,
        }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let normal = self.normal + rhs.normal;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let normal = self.normal - rhs.normal;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, normal, color, uv)
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
pub struct MeshRenderer {
    triangles: Vec<UVec3>,
    vertices: Vec<Vertex>,
}

impl MeshRenderer {
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
        let mut mesh = MeshRenderer::new();
        mesh.add_section_from_vertices(triangles, vertices);
        mesh
    }

    // TODO: as Luca said try to exercise doing it with slices
    pub fn add_section_from_vertices(&mut self, triangles: &[UVec3], vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let triangles: Vec<UVec3> = triangles.iter().map(|tri| *tri + offset).collect();
        self.triangles.extend_from_slice(&triangles);
        self.vertices.extend_from_slice(vertices);
    }

    pub fn add_section_from_buffers(
    &mut self,
    triangles: &[UVec3],
    positions: &[Vec3],
    normals: &[Vec3],
    colors: &[Vec3],
    uvs: &[Vec2],
) {
    // Calculate offset before adding new vertices
    let offset = self.vertices.len() as u32;
    
    // Offset triangle indices to account for existing vertices
    let triangles: Vec<UVec3> = triangles.iter()
        .map(|tri| *tri + offset)
        .collect();
    
    self.triangles.extend_from_slice(&triangles);

    let has_uvs = !uvs.is_empty();
    let has_colors = !colors.is_empty();

    for i in 0..positions.len() {
        let vertex = Vertex::new(
            positions[i].extend(1.0),
            normals[i],
            if has_colors { colors[i] } else { Vec3::ONE },
            if has_uvs { uvs[i] } else { Vec2::ZERO },
        );
        self.vertices.push(vertex)
    }
}

    pub fn load_from_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> MeshRenderer {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut indices = vec![];
        // TODO: handle errors
        let mut result = MeshRenderer::new();
        for primitive in mesh.primitives() {
            positions.clear();
            tex_coords.clear();
            normals.clear();
            indices.clear();

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| indices.push(i));
            }
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
            }

            let colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());

            let triangles: Vec<UVec3> = indices
                .chunks_exact(3)
                .map(|tri| UVec3::new(tri[0], tri[1], tri[2]))
                .collect();
            result.add_section_from_buffers(&triangles, &positions, &normals, &colors, &tex_coords)
        }
        result
    }

}

// for more on struct initialization check Default trait
impl Default for MeshRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for MeshRenderer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Self::from_vertices(self.triangles(), self.vertices());
        result.add_section_from_vertices(rhs.triangles(), rhs.vertices());
        result
    }
}

impl AddAssign for MeshRenderer {
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
    let clip0 = *mvp * vertices[0].position;
    let clip1 = *mvp * vertices[1].position;
    let clip2 = *mvp * vertices[2].position;

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

    let area = edge_function(sc0, sc1, sc2);

    if area <= 0.0 {
        return;
    }

    // AABB to avoid iterating through the whole buffer
    let min_x = sc0.x.min(sc1.x).min(sc2.x).floor() as i32;
    let max_x = sc0.x.max(sc1.x).max(sc2.x).ceil() as i32;
    let min_y = sc0.y.min(sc1.y).min(sc2.y).floor() as i32;
    let max_y = sc0.y.max(sc1.y).max(sc2.y).ceil() as i32;

    // Clamp to screen bounds
    let min_x = min_x.max(0) as usize;
    let max_x = max_x.min(viewport_size.x as i32) as usize;
    let min_y = min_y.max(0) as usize;
    let max_y = max_y.min(viewport_size.y as i32) as usize;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let coords = glam::vec2(x as f32, y as f32) + 0.5;
            let i = coords_to_index(x, y, viewport_size.x as usize);

            if let Some(bary) = barycentric_coordinates(coords, sc0, sc1, sc2, area) {
                let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                let depth = correction;
                let correction = 1.0 / correction;

                if depth < z_buffer[i] {
                    z_buffer[i] = depth;
                    let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                    let color = color * correction;
                    let mut color = to_argb(
                        255,
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    );
                    
                    if let Some(tex) = texture {
                        let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                        let tex_coords = tex_coords * correction;
                        color = tex.argb_at_uv(tex_coords.x, tex_coords.y);
                    }
                    
                    buffer[i] = color;
                }
            }
        }
    }

}

pub fn raster_mesh(
    mesh: &MeshRenderer,
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