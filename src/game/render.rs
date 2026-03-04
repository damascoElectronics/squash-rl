use minifb::{Window, WindowOptions};
const FPS: usize = 60;	

pub struct WindowState
{
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    window: Window,
}

impl WindowState
{
    pub fn new() -> Self
    {
        let width:usize = super::FIELD_WIDTH as usize;
        let height:usize = super::FIELD_HEIGHT as usize;
        let buffer: Vec<u32> = vec![0;  width * height]; 
        let mut window= match Window::new
        (
            "Game", 
            width, 
            height, 
            WindowOptions::default()
        ) 
        {
            Ok(win) => win,
            Err(err) => 
            {
                println!("Unable to create window {}", err);
                panic!();
            }
        };
        window.set_target_fps(FPS);
        WindowState{
            buffer: buffer,
            width: width,
            height: height,
            window: window,
        }
    }

    pub fn draw(&mut self, state: &super::GameState)
    {
        let ball_pos_x:usize = state.ball_pos.x as usize;
        let ball_pos_y:usize = state.ball_pos.y as usize;
        let racket_position_x:usize = state.racket.racket_position.x as usize;
        let racket_position_y:usize = state.racket.racket_position.y as usize;
        let racket_length:usize = (state.racket.length/2) as usize;

        self.buffer.fill(0);
        self.buffer[ball_pos_y * self.width + ball_pos_x] = 0x00FFFFFF;

         for i in  (racket_position_x - racket_length)..(racket_position_x + racket_length) {
            self.buffer[racket_position_y * self.width + i] = 0x00FFFFFF;
        }

        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

}
