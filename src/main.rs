use std::path::Path;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use glam::Vec3 as GVec3;
use std::sync::atomic::AtomicU32;

struct SceneObject {
    mesh: MeshRenderer,
    texture: Texture,
    translation: GVec3,
    rotation_deg: GVec3,
    scale: GVec3,
}

impl SceneObject {
    fn load(mesh_path: &Path, texture_path: &Path) -> Self {
        Self {
            mesh: load_gltf(mesh_path),
            texture: Texture::load(texture_path),
            translation: GVec3::ZERO,
            rotation_deg: GVec3::ZERO,
            scale: GVec3::ONE,
        }
    }

    fn mvp(&self, view: &glam::Mat4, proj: &glam::Mat4) -> glam::Mat4 {
        let rotation = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            self.rotation_deg.x.to_radians(),
            self.rotation_deg.y.to_radians(),
            self.rotation_deg.z.to_radians(),
        );
        let model = RasterTransform::new(self.translation, rotation, self.scale).local();
        proj * view * model
    }
}

#[derive(Resource)]
struct RasterizerState {
    framebuffer: Framebuffer,
    z_buffer: Vec<AtomicU32>,
    objects: Vec<SceneObject>,
    camera: RendererCamera,
    wireframe: bool,
}

#[derive(Resource)]
struct FramebufferImageHandle(Handle<Image>);

mod camera;
mod framebuffer;
mod geometry;
mod texture;
mod transform;
mod utilities;
mod window;

use crate::camera::*;
use crate::framebuffer::*;
use crate::geometry::*;
use crate::texture::*;
use crate::transform::Transform as RasterTransform;
use crate::utilities::*;
use crate::window::*;

fn startup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let z_buffer: Vec<AtomicU32> = (0..SCREEN_WIDTH * SCREEN_HEIGHT)
        .map(|_| AtomicU32::new(f32::INFINITY.to_bits()))
        .collect();

    let framebuffer = Framebuffer {
        buffer: (0..SCREEN_WIDTH * SCREEN_HEIGHT)
            .map(|_| AtomicU32::new(0))
            .collect(),
    };

    let image = Image::new_fill(
        Extent3d {
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );

    let image_handle = images.add(image);

    commands.spawn((
        Sprite {
            image: image_handle.clone(),
            custom_size: Some(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn(Camera2d);

    let initial_object = SceneObject::load(
        Path::new("assets/DamagedHelmet.gltf"),
        Path::new("assets/DamagedHelmet_albedo.jpg"),
    );

    commands.insert_resource(RasterizerState {
        framebuffer,
        z_buffer,
        objects: vec![initial_object],
        camera: RendererCamera::default(),
        wireframe: false,
    });
    commands.insert_resource(FramebufferImageHandle(image_handle));
}

fn render(
    mut images: ResMut<Assets<Image>>,
    image_handle: Res<FramebufferImageHandle>,
    mut state: ResMut<RasterizerState>,
) {
    let RasterizerState {
        framebuffer,
        z_buffer,
        objects,
        camera,
        wireframe,
    } = &mut *state;

    let view = camera.view();
    let proj = camera.projection();

    for pixel in framebuffer.buffer.iter() {
        pixel.store(0, std::sync::atomic::Ordering::Relaxed);
    }
    for z in z_buffer.iter() {
        z.store(f32::INFINITY.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }

    for obj in objects.iter() {
        let mvp = obj.mvp(&view, &proj);
        render_scene(
            &obj.mesh,
            &mvp,
            Some(&obj.texture),
            &framebuffer.buffer,
            z_buffer,
            glam::vec2(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
            *wireframe,
        );
    }

    if let Some(image) = images.get_mut(&image_handle.0) {
        if let Some(data) = image.data.as_mut() {
            convert_framebuffer_to_image(&framebuffer.buffer, data);
        }
    }
}

fn render_egui(mut contexts: EguiContexts, mut state: ResMut<RasterizerState>) {
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Scene").show(ctx, |ui| {
            ui.toggle_value(&mut state.wireframe, "Wireframe");
            ui.separator();

            let mut to_remove: Option<usize> = None;

            for (i, obj) in state.objects.iter_mut().enumerate() {
                ui.collapsing(format!("Object {}", i + 1), |ui| {
                    ui.label("Translation");
                    ui.add(egui::Slider::new(&mut obj.translation.x, -10.0..=10.0).text("x"));
                    ui.add(egui::Slider::new(&mut obj.translation.y, -10.0..=10.0).text("y"));
                    ui.add(egui::Slider::new(&mut obj.translation.z, -10.0..=10.0).text("z"));

                    ui.separator();
                    ui.label("Rotation (deg)");
                    ui.add(egui::Slider::new(&mut obj.rotation_deg.x, -180.0..=180.0).text("x"));
                    ui.add(egui::Slider::new(&mut obj.rotation_deg.y, -180.0..=180.0).text("y"));
                    ui.add(egui::Slider::new(&mut obj.rotation_deg.z, -180.0..=180.0).text("z"));

                    ui.separator();
                    ui.label("Scale");
                    ui.add(egui::Slider::new(&mut obj.scale.x, 0.1..=5.0).text("x"));
                    ui.add(egui::Slider::new(&mut obj.scale.y, 0.1..=5.0).text("y"));
                    ui.add(egui::Slider::new(&mut obj.scale.z, 0.1..=5.0).text("z"));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Reset").clicked() {
                            obj.translation = GVec3::ZERO;
                            obj.rotation_deg = GVec3::ZERO;
                            obj.scale = GVec3::ONE;
                        }
                        if ui.button("Remove").clicked() {
                            to_remove = Some(i);
                        }
                    });
                });
            }

            if let Some(i) = to_remove {
                state.objects.remove(i);
            }

            ui.separator();
            if ui.button("+ Add Helmet").clicked() {
                state.objects.push(SceneObject::load(
                    Path::new("assets/DamagedHelmet.gltf"),
                    Path::new("assets/DamagedHelmet_albedo.jpg"),
                ));
            }
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
                title: "Rasterizer".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, render)
        .add_systems(EguiPrimaryContextPass, render_egui)
        .run();
}
