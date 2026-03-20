/// Tracks the overall mathematical state of the game.
///
/// This module contains the main logic for updating positions, collisions, and scores.

pub mod input;
pub mod render;

/// Game area width in pixels.
pub const FIELD_WIDTH: u32 = 800; 
/// Game area height in pixels.
pub const FIELD_HEIGHT: u32 = 600;
/// Width of the player racket.
const RACKET_LENGTH: u32 = 20;
/// Height/thickness of the player racket.
const RACKET_HEIGHT: u32 = 4;
/// Space between the racket and the bottom of the game area.
const SPACE: u32 = 5;

/***** Structures and enums area *****/

/// Stores information about object speeds in the game.
#[derive(Clone)]
pub struct Speed
{
    /// Speed along the X axis.
	pub speed_x: i32,
    /// Speed along the Y axis.
	pub speed_y: i32,
    /// Angle of movement (in radians).
	angle: f32,
}

/// Represents a 2D position in the game area.
#[derive(Clone)]
pub struct Position
{
    /// X coordinate.
	pub x:u32,
    /// Y coordinate.
	pub y:u32,
}

/// Stores the properties and state of the player racket.
#[derive(Clone)]
pub struct Racket
{
    /// Length of the racket.
	length:u32,
    /// Height of the racket.
	height:u32,
	curve:u32,
    /// Movement speed of the racket.
	racket_speed: i32,
    /// Current position of the racket.
	pub racket_position: Position,
}
/// Contains the general information and state of the game.
#[derive(Clone)]
pub struct GameState
{
    /// Current position of the ball.
	pub ball_pos:Position,
    /// Current speed and direction of the ball.
	pub ball_speed: Speed,
    /// Current state of the racket.
	pub racket: Racket,
    /// Current score of the game.
	pub score: i32, 
    /// Indicates whether the game is currently actively playing.
 	pub active: bool,
}
// posible options can be performed
/// Possible movement actions for the racket.
pub enum Action 
{
    /// Move the racket to the left.
	Left,
    /// Move the racket to the right.
	Right,
    /// Keep the racket stationary.
	Stay,
}

/* methods and implementations */
use Action::*;

impl GameState 
{	
    /// Initializes the game with the starting state data.
    ///
    /// Sets the initial positions for the ball and racket, defaults speed,
    /// and ensures the game starts in an inactive state until updated.
    ///
    /// Returns:
    ///     A new `GameState` instance.
	pub fn new() -> Self 
		{
        	GameState
			{
			// initial state of the ball's position
			ball_pos: Position {
				x: FIELD_WIDTH/2,
				y: FIELD_HEIGHT - SPACE - RACKET_HEIGHT - 1,
			},
			// initial state of the ball's speed
			ball_speed: Speed
			{
				speed_x: 5,
				speed_y: 5,
				angle: 0.785398,
			},
			// initial state of the racket 
			racket: Racket 
			{
				length: RACKET_LENGTH,
				height: RACKET_HEIGHT,
				curve: 0,
				racket_speed:5,
				racket_position: Position
				{
					x: FIELD_WIDTH/2,
					y: FIELD_HEIGHT - SPACE - RACKET_HEIGHT,
				},
			},
			score: 0, 	// score initialitation 
			active: false,	// game start in stop, to move active changes to true
		}
	}
    /// Updates the game state by one frame slice logic.
    ///
    /// Evaluates the movement of the ball, checks for collisions with walls or 
    /// the racket, updates the score, and handles end-game conditions.
	pub fn update(&mut self)
	{
		if self.active == true
		{
			// 1. Calculate the hypothetical new position of the ball based on current speed
            let new_x:i32 = self.ball_pos.x as i32 + self.ball_speed.speed_x;
            let new_y:i32 = self.ball_pos.y as i32 - self.ball_speed.speed_y;
            
			// 2. Left Wall Collision: Check if the ball hit the left boundary (x == 0)
			if new_x == 0 
			{
			    // Reverse horizontal direction and keep moving
				self.ball_speed.speed_x = -self.ball_speed.speed_x; 
			}
            
            // 3. Right Wall Collision: Check if the ball hit the right boundary (x == FIELD_WIDTH)
			if new_x == FIELD_WIDTH as i32
			{
			    // Reverse horizontal direction and keep moving
				self.ball_speed.speed_x = -self.ball_speed.speed_x; 
			}
			
			// 4. Top Wall Collision: Check if the ball hit the ceiling (y == 0)
			if new_y == 0
			{
			    // Reverse vertical direction to bounce off ceiling
				self.ball_speed.speed_y = -self.ball_speed.speed_y;
            }
            
            // 5. Racket Collision: Check if the ball coordinates overlap with the racket
            //    - Check horizontal overlap: within (racket_x - length/2) to (racket_x + length/2)
            //    - Check vertical exact match: y equals the specific height of the racket surface
			if (new_x >= (self.racket.racket_position.x as i32 - (RACKET_LENGTH as i32 /2))) && 
			   (new_x <= (self.racket.racket_position.x as i32 + (RACKET_LENGTH as i32 /2))) && 
			   (new_y == (FIELD_HEIGHT as i32 - SPACE as i32 - RACKET_HEIGHT as i32 - 1))
			{
			    // Bounce the ball upwards
				self.ball_speed.speed_y = -self.ball_speed.speed_y;
				// Increment the player's score for a successful hit
				self.score += 1;
			}
			
            // 6. Floor Collision (Game Over): Check if the ball went past the bottom threshold
			if new_y >= FIELD_HEIGHT as i32
			{
			    // Stop the game loop and immediately return
				self.active = false;
				return;
			}
			
			// 7. Apply the validated new coordinates to the ball's actual position
			self.ball_pos.x = new_x as u32;
			self.ball_pos.y = new_y as u32;
		}
	}

    /// Updates the racket's position based on a given action.
    ///
    /// Applies movement to the left or right while respecting the boundaries 
    /// of the game area.
    ///
    /// Args:
    ///     `dir`: The `Action` (Left, Right, or Stay) to apply to the racket.
	pub fn move_racket(&mut self, dir:Action)
	{
		match dir
		{
			Left => 
			{
			    // Calculate what the leftmost coordinate of the racket would be if moved left
				let new_x_left:i32 = self.racket.racket_position.x as i32 - (RACKET_LENGTH as i32 /2) - self.racket.racket_speed;
				
				// Ensure that the new left edge doesn't cross the left boundary (0)
				if new_x_left > 0 
				{
				    // Safely update the position
					self.racket.racket_position.x = self.racket.racket_position.x - self.racket.racket_speed as u32;	
				}
			},
    		Right => 
			{
			    // Calculate what the rightmost coordinate of the racket would be if moved right
				let new_x_right:i32 = self.racket.racket_position.x as i32 + (RACKET_LENGTH as i32 /2) + self.racket.racket_speed;
				
				// Ensure that the new right edge doesn't cross the right boundary (FIELD_WIDTH)
				if new_x_right < FIELD_WIDTH as i32
				{
				    // Safely update the position
					self.racket.racket_position.x = self.racket.racket_position.x + self.racket.racket_speed as u32;
				}
			},
			Stay => {} // Do nothing
		}
	}
}

