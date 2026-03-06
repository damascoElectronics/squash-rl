use std::env::args;
mod game;
mod agent;

fn main() {
 

    let args: Vec<String> = std::env::args().collect();
    let mode: &str = if args.len() > 1 { &args[1] } else { "error" };


    match mode {
        "manual" => 
        { 
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
        "agent" => 
        {
            let mut current_state = game::GameState::new();
            let mut previous_state = game::GameState::new();
            let mut agent = agent::Agent::new();
            let mut window = game::render::WindowState::new();
            let mut reward:i32 = 0; 
            let mut action: game::Action = game::Action::Stay;
            let mut episode: u32 = 0;

            current_state.active = true;

            while window.window.is_open()
            {
                match game::input::read_input(&window.window)
                {
                    game::input::Action::Stop => 
                    {
                        break;
                    }
                    _ => {
                        action = agent.decide(&current_state);
                        match action
                        {
                            game::Action::Left =>
                            {
                                current_state.move_racket(game::Action::Left);                            
                            }
                            game::Action::Right =>
                            {
                                current_state.move_racket(game::Action::Right);
                            }
                            _ => {},
                        }
                    },
                }

                current_state.update(); 
                if !current_state.active
                {
                    current_state = game::GameState::new();
                    current_state.active = true;
                    episode += 1;
                    agent.epsilon = (agent.epsilon * 0.995).max(0.05);
                    reward = -1;

                }
                else if (previous_state.ball_speed.speed_y > 0) && (current_state.ball_speed.speed_y < 0)
                {
                    reward = 1;
                }
                else 
                {
                    reward = 0;
                }
                agent.learn(&previous_state, &action, reward as f32, &current_state);
                previous_state = current_state.clone();
                if episode % 100 == 0 {
                    // limpiar pantalla
                    print!("\x1B[2J\x1B[1;1H");
                    println!("Episode: {}, Epsilon: {:.3}, Score: {}", episode, agent.epsilon, current_state.score);
                    window.draw(&current_state);
                }               
            } 
            agent.save(); 
        }
        _ => 
        { 
            println!("no command");
        }
    }

}