
use std::io::{stdout, Write};
use std::time::Duration;
mod game;
 
fn main() {
 
    let mut state = game::GameState::new();
    let mut window = game::render::WindowState::new();
    state.active = true;
    
    while window.window.is_open()
    {
        match game::input::read_input(&window.window)
        {
            game::input::Action::Stop => 
            {
                break;
            }
            game::input::Action::Left =>
            {
                state.move_racket(game::Action::Left);                            
            }
            game::input::Action::Right =>
            {
                state.move_racket(game::Action::Right);
            }
            _ => {},
        }
    
        state.update();
        window.draw(&state);
        if !state.active
        {
            break;
        }
    }

}


