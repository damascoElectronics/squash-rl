// input.rs track keyobard 
use minifb::{Window, Key};

pub enum Action 
{   
    Start,
	Left,
	Right,
    Stop,
    Stay,
}


pub fn read_input(window:&Window) -> Action
{
    if window.is_key_down(Key::Left) 
    {
        Action::Left
    } else if window.is_key_down(Key::Right) 
    {
        Action::Right
    } else if window.is_key_down(Key::Escape) 
    {
        Action::Stop
    } else 
    {
        Action::Stay
    }
    
}
