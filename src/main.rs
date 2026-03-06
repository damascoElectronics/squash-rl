/// Main entry point for the Arcade III: Squash game.
///
/// This module parses the command-line arguments to determine whether the game should
/// be played manually by a human or automatically by a Reinforcement Learning agent.
use std::env::args;
mod game;
mod agent;

/// Standard application entry point.
fn main() {
    // Parse command-line arguments into a vector of strings
    let args: Vec<String> = std::env::args().collect();
    
    // Determine the game mode based on the first argument provided (if any)
    let mode: &str = if args.len() > 1 { &args[1] } else { "error" };

    // Execute logic based on the selected game mode
    match mode {
        "manual" => 
        { 
            // Initialize game state and window state for human play
            let mut state = game::GameState::new();
            let mut window = game::render::WindowState::new();
            state.active = true; // Mark the game as actively running

            // Main game loop: continues as long as the window is open
            while window.window.is_open()
            {
                // Read keyboard input from the user and respond accordingly
                match game::input::read_input(&window.window)
                {
                    game::input::Action::Stop => 
                    {
                        // Escape pressed, break out of the loop
                        break;
                    }
                    game::input::Action::Left =>
                    {
                        // Move the racket left
                        state.move_racket(game::Action::Left);                            
                    }
                    game::input::Action::Right =>
                    {
                        // Move the racket right
                        state.move_racket(game::Action::Right);
                    }
                    _ => {}, // Ignore other actions
                }

                // Update the physics/game logic for this frame
                state.update();
                
                // Draw the updated state to the window buffer
                window.draw(&state);
                
                // End the game if the state logic determines the game is inactive (e.g., ball missed)
                if !state.active
                {
                    break;
                }
            }
        }
        "agent" => 
        {
            // Initialize states required for reinforcement learning
            // current_state: the game state after an action
            // previous_state: the game state before the last action
            let mut current_state = game::GameState::new();
            let mut previous_state = game::GameState::new();
            let mut agent = agent::Agent::new(); // Initialize the Q-learning agent
            let mut window = game::render::WindowState::new(); // Initialize the rendering window
            
            let mut reward:i32 = 0; // Reward to provide to the agent based on action
            let mut action: game::Action = game::Action::Stay; // Action the agent chooses
            let mut episode: u32 = 0; // Training episode counter

            current_state.active = true; // Start the game

            // Main agent loop: continues as long as the window is open
            while window.window.is_open()
            {
                // Check for manual interrupt input (e.g., pressing Escape)
                match game::input::read_input(&window.window)
                {
                    game::input::Action::Stop => 
                    {
                        // Escape pressed, break out of the loop early
                        break;
                    }
                    _ => {
                        // Let the agent observe the state and decide on an action
                        action = agent.decide(&current_state);
                        
                        // Execute the chosen action
                        match action
                        {
                            game::Action::Left =>
                            {
                                // Agent chose to move left
                                current_state.move_racket(game::Action::Left);                            
                            }
                            game::Action::Right =>
                            {
                                // Agent chose to move right
                                current_state.move_racket(game::Action::Right);
                            }
                            _ => {}, // Stay action does nothing
                        }
                    },
                }

                // Update the physics/game logic based on chosen action
                current_state.update(); 
                
                // Check conditions to assign reward and update learning
                if !current_state.active
                {
                    // Game over (ball was missed): reset environment for a new episode
                    current_state = game::GameState::new();
                    current_state.active = true;
                    episode += 1;
                    
                    // Decrease exploration factor to exploit more as we learn
                    agent.epsilon = (agent.epsilon * 0.995).max(0.05);
                    
                    // Assign negative reward for losing
                    reward = -1;

                }
                else if (previous_state.ball_speed.speed_y > 0) && (current_state.ball_speed.speed_y < 0)
                {
                    // Positive reward: the ball bounced off the bottom racket correctly 
                    // (Y speed changed from positive/downwards to negative/upwards)
                    reward = 1;
                }
                else 
                {
                    // Neutral reward for regular intermediate moves to prevent overfitting
                    reward = 0;
                }
                
                // Allow the agent to learn from the transition: (State, Action, Reward, State')
                agent.learn(&previous_state, &action, reward as f32, &current_state);
                
                // Store current state as previous state for the next step
                previous_state = current_state.clone();
                
                // Provide visual output and render game every 100 episodes
                if episode % 100 == 0 {
                    // Clear terminal output screen
                    print!("\x1B[2J\x1B[1;1H");
                    // Print current progress
                    println!("Episode: {}, Epsilon: {:.3}, Score: {}", episode, agent.epsilon, current_state.score);
                    // Draw updated frame state
                    window.draw(&current_state);
                }               
            } 
            // Save the learned Q-table once finished
            agent.save(); 
        }
        _ => 
        { 
            // Inform user if an invalid or missing argument is provided
            println!("no command");
        }
    }

}