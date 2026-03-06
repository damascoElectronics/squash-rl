/// Tracks keyboard input for the game.
use minifb::{Window, Key};

/// Possible action options that can be performed by the player or agent.
pub enum Action 
{   
    /// Move the racket to the left.
	Left,
    /// Move the racket to the right.
	Right,
    /// Stop the game or exit.
    Stop,
    /// Do not move the racket.
    Stay,
}

/* methods and implementations */
/// Reads the current state of the keyboard from the provided window.
/// 
/// Translates the keyboard input into an `Action` that the game or agent can process.
/// 
/// Args:
///     `window`: A reference to the minifb `Window` object to check for key presses.
/// 
/// Returns:
///     An `Action` corresponding to the key pressed (`Left`, `Right`, `Stop`, or `Stay`).
pub fn read_input(window:&Window) -> Action
{ 
    // Check if the Left Arrow key is currently depressed
    if window.is_key_down(Key::Left) 
    {
        // Return the Left action to instruct the racket to move left
        Action::Left
    } 
    // Otherwise, check if the Right Arrow key is currently depressed
    else if window.is_key_down(Key::Right) 
    {
        // Return the Right action to instruct the racket to move right
        Action::Right
    } 
    // Otherwise, check if the Escape key is currently depressed
    else if window.is_key_down(Key::Escape) 
    {
        // Return the Stop action to gracefully close the game window
        Action::Stop
    } 
    // If none of the relevant keys are depressed
    else 
    {
        // Return the Stay action to keep the racket in its current position
        Action::Stay
    }
}
