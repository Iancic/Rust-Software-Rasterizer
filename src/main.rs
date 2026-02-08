use std::path::Path;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use glam::Vec3 as GVec3;
// Store 
#[derive(Resource)]
struct RasterizerState {
    framebuffer: Framebuffer,
    z_buffer: Vec<f32>,
    mesh: MeshRenderer,
    texture: Texture,
    camera: RendererCamera,
}

// This is attached to an entity so I can acces the buffer anytime.
#[derive(Resource)]
struct FramebufferImageHandle(Handle<Image>);

#[derive(Resource)]
struct ModelTransform {
    translation: GVec3,
    rotation_deg: GVec3,
    scale: GVec3,
}

mod utilities;
mod texture;
mod geometry;
mod window;
mod framebuffer;
mod camera;
mod transform;

use crate::geometry::*;
use crate::transform::Transform as RasterTransform;
use crate::utilities::*;
use crate::camera::*;
use crate::texture::*;
use crate::framebuffer::*;
use crate::window::*;


fn startup(mut commands: Commands,
    mut images: ResMut<Assets<Image>>,){
    // Resources (object, texture and z buffer)
    let z_buffer = vec![f32::INFINITY; SCREEN_HEIGHT * SCREEN_WIDTH];

    let camera = RendererCamera::default();

    let texture = Texture::load(Path::new("assets/DamagedHelmet_albedo.jpg"));
    let mesh = load_gltf(Path::new("assets/DamagedHelmet.gltf"));

    // Framebuffer to rasterize into
    let framebuffer = Framebuffer{
        buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT]
    };

    // Create a Bevy image (GPU texture) from your framebuffer
    // This is where format conversion happens
    let image = Image::new_fill(
        Extent3d {
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255], // Initial black color
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );

    // Add the image to Bevy's asset system and get a handle
    let image_handle = images.add(image);

    commands.spawn((
        Sprite {
            image: image_handle.clone(),
            custom_size: Some(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a camera so the sprite gets rendered
    commands.spawn(Camera2d);
    
    // Store the rasterizer state and image handle as resources
    commands.insert_resource(RasterizerState {
        framebuffer,
        z_buffer,
        mesh,
        texture,
        camera,
    });
    commands.insert_resource(FramebufferImageHandle(image_handle));
    commands.insert_resource(ModelTransform {
        translation: GVec3::ZERO,
        rotation_deg: GVec3::ZERO,
        scale: GVec3::ONE,
    });

}

fn update(){

}

fn render(
    mut images: ResMut<Assets<Image>>,
    image_handle: Res<FramebufferImageHandle>,
    mut state: ResMut<RasterizerState>,
    model: Res<ModelTransform>,
    ){
    
    // The custom rendering begins here

    // Matrixes for model, view and projection space for rasterization.
    let rotation = glam::Quat::from_euler(
        glam::EulerRot::XYZ,
        model.rotation_deg.x.to_radians(),
        model.rotation_deg.y.to_radians(),
        model.rotation_deg.z.to_radians(),
    );
    let parent_local = RasterTransform::new(model.translation, rotation, model.scale).local();
    let RasterizerState {
        framebuffer,
        z_buffer,
        mesh,
        texture,
        camera,
    } = &mut *state;

    let view = camera.view();
    let proj = camera.projection();

    // Clear color and depth
    clear_buffer(&mut framebuffer.buffer, 0);
    clear_buffer(z_buffer, f32::INFINITY);

    render_scene(
        &*mesh,
        &(proj * view * parent_local),
        Some(&*texture),
        &mut framebuffer.buffer,
        z_buffer,
        glam::vec2(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
    );

    // Credit: Codex 5.2 + utility to convert
    // Get the Bevy image and update its data
    if let Some(image) = images.get_mut(&image_handle.0) {
        // Convert your ARGB u32 buffer to RGBA byte buffer
        if let Some(data) = image.data.as_mut() {
            convert_framebuffer_to_image(&framebuffer.buffer, data);
        }
    }

}

fn render_egui(mut contexts: EguiContexts, mut model: ResMut<ModelTransform>) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // Credit: Codex 5.2
        egui::Window::new("Model Transform").show(ctx, |ui| {
            ui.label("Translation");
            ui.add(egui::Slider::new(&mut model.translation.x, -10.0..=10.0).text("x"));
            ui.add(egui::Slider::new(&mut model.translation.y, -10.0..=10.0).text("y"));
            ui.add(egui::Slider::new(&mut model.translation.z, -10.0..=10.0).text("z"));

            ui.separator();
            ui.label("Rotation (deg)");
            ui.add(egui::Slider::new(&mut model.rotation_deg.x, -180.0..=180.0).text("x"));
            ui.add(egui::Slider::new(&mut model.rotation_deg.y, -180.0..=180.0).text("y"));
            ui.add(egui::Slider::new(&mut model.rotation_deg.z, -180.0..=180.0).text("z"));

            ui.separator();
            ui.label("Scale");
            ui.add(egui::Slider::new(&mut model.scale.x, 0.1..=5.0).text("x"));
            ui.add(egui::Slider::new(&mut model.scale.y, 0.1..=5.0).text("y"));
            ui.add(egui::Slider::new(&mut model.scale.z, 0.1..=5.0).text("z"));

            ui.separator();
            if ui.button("Reset").clicked() {
                model.translation = GVec3::ZERO;
                model.rotation_deg = GVec3::ZERO;
                model.scale = GVec3::ONE;
            }
        });
    }
}

fn main()
{
    // Integrated with Bevy ;)
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .add_systems(Update, render)
        .add_systems(EguiPrimaryContextPass, render_egui)
        .run();
}
