// input.rs track keyobard 
use crossterm::event::{read, Event, KeyEventKind, KeyCode, poll};

pub enum Action 
{   
    Start,
	Left,
	Right,
    Stop,
    Stay,
}


pub fn read_input() -> Action
{
    match read().unwrap() 
        {
            Event::Key(event) => 
            {
            if event.kind == KeyEventKind::Press {
                
                match event.code 
                {
                    KeyCode::Backspace => 
                    {
                        Action::Stop
                    },
                    
                    KeyCode::Enter => 
                    {
                        Action::Start
                    },
                    KeyCode::Left =>
                    {
                        Action::Left                            
                    },
                    KeyCode::Right =>
                    {
                        Action::Right
                    },
                    _ => Action::Stay,
                }
            }
            else
            {
                 Action::Stay
            }
        },
        _ => Action::Stay,
    }
    
}
