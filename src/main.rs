 // Include from other libraries
use minifb::{Window, WindowOptions, Key};

const WIDTH: usize = 800;
const HEIGHT: usize = 450;

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
        for i in 0..WIDTH * HEIGHT
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
    let mut color: u32,

    color
}

fn main() 
{
    let mut framebuffer = Framebuffer{
        buffer: vec![0; WIDTH * HEIGHT],
        width: WIDTH,
        height: HEIGHT
    };

    let mut current_window = WindowInstance{
        window: Window::new("Rasterizer", framebuffer.width, framebuffer.height , WindowOptions::default(),)
                            .unwrap_or_else(|e| {panic!("{}", e);}),
    };

    current_window.set_fps(60);

    while current_window.window.is_open() && !current_window.window.is_key_down(Key::Escape)
    {
        framebuffer.clear_screen(5);

        framebuffer.plot_pixel(200, 200, 255);

        current_window.window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }

}