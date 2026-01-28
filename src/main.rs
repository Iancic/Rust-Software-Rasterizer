use minifb::{Key, Window, WindowOptions};
use glam::{Vec2, Vec3, Vec4};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 800;

fn to_argb(input_color: Vec4) -> u32
{
    let mut color: u32 = input_color.x as u32;
    color = (color << 8) + input_color.y as u32;
    color = (color << 8) + input_color.z as u32;
    color = (color << 8) + input_color.w as u32;
    color
}

struct Vertex
{
    position: Vec3,
    normal: Vec3,
    color: Vec3,
    uv: Vec2,
}

struct Edge
{
    point1: Vec2,
    point2: Vec2,
}

fn edge_function(v0: &Vertex, v1: &Vertex, point: &Vec3) -> f32
{
    (point.x - v0.position.x) * (v1.position.y - v0.position.y) - (point.y - v0.position.y) * (v1.position.x - v0.position.x)
}

fn is_inside(edge1: f32, edge2: f32, edge3: f32) -> bool
{
    if edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0
    {
        return true;
    }
    else
    {
        return false;
    }
}

struct WindowInstance
{
    window: Window
}

impl WindowInstance
{
    fn set_fps(&mut self, fps: usize)
    {
        self.window.set_target_fps(fps);
    }
}

struct Framebuffer
{
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

fn main()
{
    let v0 = Vertex{
        position: Vec3{x: 200.0, y: 200.0, z: 0.0},  // top-left
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 255.0, y: 0.0, z: 0.0},
        uv: Vec2{x: 0.0, y: 0.0},
    };

    let v1 = Vertex{
        position: Vec3{x: 600.0, y: 200.0, z: 0.0},  // top-right
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 0.0, y: 255.0, z: 0.0},
        uv: Vec2{x: 1.0, y: 0.0},
    };

    let v2 = Vertex{
        position: Vec3{x: 200.0, y: 600.0, z: 0.0},  // bottom-left
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 0.0, y: 0.0, z: 255.0},
        uv: Vec2{x: 0.0, y: 1.0},
    };

    // Triangle 2 (top-right, bottom-right, bottom-left)
    let v3 = Vertex{
        position: Vec3{x: 600.0, y: 200.0, z: 0.0},  // top-right
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 0.0, y: 255.0, z: 0.0},
        uv: Vec2{x: 1.0, y: 0.0},
    };

    let v4 = Vertex{
        position: Vec3{x: 600.0, y: 600.0, z: 0.0},  // bottom-right
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 255.0, y: 255.0, z: 0.0},
        uv: Vec2{x: 1.0, y: 1.0},
    };

    let v5 = Vertex{
        position: Vec3{x: 200.0, y: 600.0, z: 0.0},  // bottom-left
        normal: Vec3{x: 0.0, y: 0.0, z: 0.0},
        color: Vec3{x: 0.0, y: 0.0, z: 255.0},
        uv: Vec2{x: 0.0, y: 1.0},
    };

    let mut framebuffer = Framebuffer{
        buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT
    };

    let mut current_window = WindowInstance{
        window: Window::new("Rasterizer", framebuffer.width, framebuffer.height , WindowOptions::default(),)
                            .unwrap_or_else(|e| {panic!("{}", e);}),
    };

    current_window.set_fps(60);

    while current_window.window.is_open() && !current_window.window.is_key_down(Key::Escape)
    {
        for (i, pixel) in framebuffer.buffer.iter_mut().enumerate()
        {
            let point = Vec3::new(i as f32 % SCREEN_WIDTH as f32, i as f32 / SCREEN_WIDTH as f32, 0.0);

            let area = edge_function(&v0, &v2, &v1.position);
            let weight_1 = edge_function(&v0, &v2, &point);
            let weight_2 = edge_function(&v2, &v1, &point);
            let weight_3 = edge_function(&v1, &v0, &point);
            let weight_4 = edge_function(&v3, &v5, &point);
            let weight_5 = edge_function(&v5, &v4, &point);
            let weight_6 = edge_function(&v4, &v3, &point);

            if is_inside(weight_1, weight_2, weight_3)
            {
                let color_vert_a = v0.color * weight_1 / area;
                let color_vert_b = v1.color * weight_2 / area;
                let color_vert_c = v2.color * weight_3 / area;

                let final_color = color_vert_a + color_vert_b + color_vert_c;

                *pixel = to_argb(Vec4::new(255f32, final_color.x, final_color.y, final_color.z));
            }
            if is_inside(weight_4, weight_5, weight_6)
            {
                let color_vert_a = v0.color * weight_1 / area;
                let color_vert_b = v1.color * weight_2 / area;
                let color_vert_c = v2.color * weight_3 / area;

                let final_color = color_vert_a + color_vert_b + color_vert_c;

                *pixel = to_argb(Vec4::new(255f32, final_color.x, final_color.y, final_color.z));
            }
        }

        current_window.window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

}