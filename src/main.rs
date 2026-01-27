 // Include from other libraries
use minifb::{Window, WindowOptions, Key};
use glam::{Vec2, Vec3};

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 800;

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

impl Framebuffer 
{
   fn clear_screen(&mut self, color: u32)
   {
        for i in 0..SCREEN_WIDTH * SCREEN_HEIGHT
        {
            self.buffer[i as usize] = color;
        }
   }

   fn plot_pixel(&mut self, x_coordinate: usize, y_coordinate: usize, _color: u32)
   {
        if x_coordinate < self.width.try_into().unwrap() && y_coordinate <self.height.try_into().unwrap()
        {
            self.buffer[self.width * y_coordinate + x_coordinate] = _color;
        }
   }
}

fn to_argb(a: u8, r: u8, g: u8, b: u8) -> u32
{
    let mut color: u32 = a as u32;
    color = (color << 8) + r as u32;
    color = (color << 8) + g as u32;
    color = (color << 8) + b as u32;
    color
}

struct Edge
{
    point1: Vec2,
    point2: Vec2,
}

fn edge_function(edge: &Edge, point: Vec3) -> f32
{
    (point.x - edge.point1.x) * (edge.point2.y - edge.point1.y) - (point.y - edge.point1.y) * (edge.point2.x - edge.point1.x)
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

fn main() 
{
     // 3 edges to test against
    let mut edge1 = Edge{point1: Vec2::new(0.0, 0.0), point2: Vec2::new(0.0, 0.0)};    
    edge1.point1 = Vec2::new(100.0, 200.0);
    edge1.point2 = Vec2::new(300.0, 500.0);

    let mut edge2= Edge{point1: Vec2::new(0.0, 0.0), point2: Vec2::new(0.0, 0.0)};    
    edge2.point1 = Vec2::new(300.0, 500.0);
    edge2.point2 = Vec2::new(500.0, 200.0);

    let mut edge3= Edge{point1: Vec2::new(0.0, 0.0), point2: Vec2::new(0.0, 0.0)};    
    edge3.point1 = Vec2::new(500.0, 200.0);
    edge3.point2 = Vec2::new(100.0, 200.0);

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
        //framebuffer.clear_screen(to_argb(255, 255, 255, 255));
        
        for (i, pixel) in framebuffer.buffer.iter_mut().enumerate()
        {
            let point = Vec3::new(i as f32 % SCREEN_WIDTH as f32, i as f32 / SCREEN_WIDTH as f32, 0.0);
            let bool1 = edge_function(&edge1, point);
            let bool2 = edge_function(&edge2, point);
            let bool3 = edge_function(&edge3, point);
            
            if is_inside(bool1, bool2, bool3)
            {
                *pixel = to_argb(
                            255,
                            255,
                            0,
                            0);            
            }
        }
        
        current_window.window
            .update_with_buffer(&framebuffer.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

}