// input.rs track keyobard 
use minifb::{Window, Key};

/***** Structures and enums area *****/
// posible options can be performed
pub enum Action 
{   
	Left,
	Right,
    Stop,
    Stay,
}

/* methods and implementations */
// adquition of the input by the minifb
pub fn read_input(window:&Window) -> Action
{ 
    // Note is_key_down retunrs a bool, match cannot be used
    // chequing if left has been pressed.
    if window.is_key_down(Key::Left) 
    {
        Action::Left
    // chequing if right has been pressed.
    } else if window.is_key_down(Key::Right) 
    {
        Action::Right
    // chequing if left has been pressed to left the game.
    } else if window.is_key_down(Key::Escape) 
    {
        Action::Stop
    } else 
    // case of no action detected.
    {
        Action::Stay
    }
    
}
