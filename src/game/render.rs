use minifb::{Window, WindowOptions};
const FPS: usize = 60;	

/// Manages the game window and pixel buffer for rendering.
pub struct WindowState
{
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    pub window: Window,
}

impl WindowState
{
    /// Initializes a new `WindowState`.
    ///
    /// Creates a window with the dimensions specified by the core game field
    /// constants and allocates a display buffer for rendering.
    ///
    /// Returns:
    ///     A new `WindowState` instance.
    pub fn new() -> Self
    {
        // 1. Determine the window pixel width and height from the constants
        let width:usize = super::FIELD_WIDTH as usize;
        let height:usize = super::FIELD_HEIGHT as usize;
        
        // 2. Allocate a 1D vector to hold pixels (flattened 2D grid). Initialize all pixels to 0x0 (black)
        let buffer: Vec<u32> = vec![0;  width * height]; 
        
        // 3. Attempt to create the physical OS window using minifb
        let mut window = match Window::new
        (
            "Game", 
            width, 
            height, 
            WindowOptions
            {
                resize: false, // Prevent user from dynamically resizing window and breaking coordinates
                ..WindowOptions::default()
            }
        ) 
        {
            Ok(win) => win, // Successfully acquired window 
            Err(err) => 
            {
                // Failed to create window, panic with the reason
                println!("Unable to create window {}", err);
                panic!();
            }
        };
        
        // 4. Set the framerate target to stabilize the game pace
        window.set_target_fps(FPS);
        
        // 5. Construct and return the finalized WindowState struct
        WindowState{
            buffer: buffer,
            width: width,
            height: height,
            window: window,
        }
    }

    /// Draws the current game state to the window buffer and displays it.
    ///
    /// Renders the ball and the racket in their current positions, updates
    /// the window title with the current score, and flushes the buffer to the window.
    ///
    /// Args:
    ///     `state`: A reference to the current `GameState` containing game logic data.
    pub fn draw(&mut self, state: &super::GameState)
    {
        // 1. Extract physical integer coordinates from the state representation
        let ball_pos_x:usize = state.ball_pos.x as usize;
        let ball_pos_y:usize = state.ball_pos.y as usize;
        let racket_position_x:usize = state.racket.racket_position.x as usize;
        let racket_position_y:usize = state.racket.racket_position.y as usize;
        
        // 2. Calculate the "radius" or half-width of the racket
        let racket_length:usize = (state.racket.length/2) as usize;

        // 3. Clear the frame buffer by painting all pixels black (0)
        self.buffer.fill(0);
        
        // 4. Update the OS window's top banner to display the dynamic current score
        self.window.set_title(&format!("Squash RL - Score: {}", state.score));
        
        // 5. Paint the single pixel corresponding to the ball's location to pure white (0x00FFFFFF)
        self.buffer[ball_pos_y * self.width + ball_pos_x] = 0x00FFFFFF;

        // 6. Paint a continuous line of pixels for the racket using a loop to form the rectangle
        for i in  (racket_position_x - racket_length)..(racket_position_x + racket_length) {
            self.buffer[racket_position_y * self.width + i] = 0x00FFFFFF;
        }

        // 7. Push the drawn pixel buffer to the OS screen to make it visually update
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

}
