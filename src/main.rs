
use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::event::{read, Event, KeyEventKind, KeyCode, poll};
mod game;
 
fn main() {
 
    let mut state = game::GameState::new();
    let _ = crossterm::terminal::enable_raw_mode().unwrap();
    state.active = true;
    
    'a: loop 
    {            
        while poll(Duration::from_millis(0)).unwrap()
        {
            match game::input::read_input()
            {
                game::input::Action::Stop => 
                {
                    break 'a;
                }
                game::input::Action::Left =>
                {
                    state.move_racket(game::Action::Left);                            
                }
                game::input::Action::Right =>
                {
                    state.move_racket(game::Action::Right);
                }
                _ => continue,
            }
        }
        state.update();
        draw(&state);
        if !state.active
        {
            break 'a;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let _ = crossterm::terminal::disable_raw_mode();
}


fn draw(state: &game::GameState) {
    // limpiar pantalla
    print!("\x1B[2J\x1B[1;1H");
    // imprimir score
    println!("Score: {}", state.score);
    // imprimir posición de la pelota (debug)
    println!("Ball: ({}, {})", state.ball_pos.x, state.ball_pos.y);
    // imprimir posición de la raqueta
    println!("Racket: ({},{}), rang({} , {})", state.racket.racket_position.x,  state.racket.racket_position.y,  state.racket.racket_position.x - 10,  state.racket.racket_position.x + 10);
}