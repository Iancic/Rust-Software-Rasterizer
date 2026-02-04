use minifb::{Key, Window, WindowOptions};
use std::path::Path;

mod utilities;
mod texture;
mod geometry;
mod window;
mod framebuffer;
mod camera;
mod transform;

use crate::geometry::*;
use crate::transform::*;
use crate::utilities::*;
use crate::camera::*;
use crate::texture::*;
use crate::framebuffer::*;
use crate::window::*;

fn main()
{
    // Resources (object, texture and z buffer)
    let mut z_buffer = vec![f32::INFINITY; SCREEN_HEIGHT * SCREEN_WIDTH];

    let texture = Texture::load(Path::new("assets/dirt.jpg"));

    let camera = Camera::default();

    // Face made with 2 triangles indexed
    // Face x6 -> Cube
    let v0 = Vertex {
        position: glam::vec3(-1.0, -1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v1 = Vertex {
        position: glam::vec3(-1.0, 1.0, 1.0),
        color: glam::vec3(1.0, 0.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v2 = Vertex {
        position: glam::vec3(1.0, 1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 0.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        position: glam::vec3(1.0, -1.0, 1.0),
        color: glam::vec3(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let mut rot = std::f32::consts::FRAC_PI_4;

    //+z
    let transform0 = Transform::IDENTITY;
    //-z
    let transform1 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::PI,
        0.0,
        0.0,
    ));
    //+y
    let transform2 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //-y
    let transform3 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        -std::f32::consts::FRAC_PI_2,
        0.0,
        0.0,
    ));
    //+x
    let transform4 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        -std::f32::consts::FRAC_PI_2,
        0.0,
    ));
    //-x
    let transform5 = Transform::from_rotation(glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        0.0,
        std::f32::consts::FRAC_PI_2,
        0.0,
    ));

    let triangles = vec![glam::uvec3(2, 1, 0), glam::uvec3(3, 2, 0)];
    let vertices = vec![v0, v1, v2, v3];

    let mesh = Mesh::from_vertices(&triangles, &vertices);

    // Framebuffer to rasterize into
    let mut framebuffer = Framebuffer{
        buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT
    };

    // Window to display framebuffer
    let mut current_window = WindowInstance{
        window: Window::new("Rasterizer", framebuffer.width, framebuffer.height , WindowOptions::default(),)
                            .unwrap_or_else(|e| {panic!("{}", e);}),
    };

    current_window.set_fps(60);

    while current_window.window.is_open() && !current_window.window.is_key_down(Key::Escape)
    {
        // Rendering starts here:

        // Matrixes for model, view and projection space for rasterization.
        let parent_local = Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, 0.0, 0.0)).local();
        let view = camera.view();
        let proj = camera.projection();

        // Clear color and depth
        clear_buffer(&mut framebuffer.buffer, 0);
        clear_buffer(&mut z_buffer, f32::INFINITY);
        
        //
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform0.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform1.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform2.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform3.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform4.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );
        raster_mesh(
            &mesh,
            &(proj * view * parent_local * transform5.local()),
            Some(&texture),
            &mut framebuffer.buffer,
            &mut z_buffer,
            current_window.window_size(),
        );

        // Rendering ends here:
        current_window.window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

}