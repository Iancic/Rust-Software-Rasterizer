use glam::Vec2;
use minifb::Window;

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 800;

pub struct WindowInstance
{
    pub window: Window
}

impl WindowInstance
{
    pub fn set_fps(&mut self, fps: usize)
    {
        self.window.set_target_fps(fps);
    }

    pub fn window_size(&mut self) -> Vec2
    {
        Vec2 { x: SCREEN_WIDTH as f32, y: SCREEN_HEIGHT as f32 }
    }
}
