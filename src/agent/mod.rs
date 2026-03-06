/// Reinforcement Learning Agent.
///
/// Implements a Q-learning agent capable of playing the game through exploration and exploitation.
use crate::game::{GameState, FIELD_HEIGHT, FIELD_WIDTH, Action};
use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

/// Number of discrete zones to divide the game area into.
const NUM_ZONES: u32 = 40;
const ZONE_RELATION_H: i32 = (FIELD_HEIGHT / NUM_ZONES) as i32 ;
const ZONE_RELATION_W: i32 = (FIELD_WIDTH / NUM_ZONES) as i32 ;

/// The Q-learning Agent.
#[derive(Serialize, Deserialize)]
pub struct Agent 
{
    /// Q-table mapping states `(zone_x_ball, zone_y_ball, dir_x, dir_y, zone_x_racket)` to Q-values for each action.
    q_table: HashMap<(i32,i32,i32,i32,i32), [f32; 3]>,  
    /// Learning rate (alpha).
    alpha: f32,                                      
    /// Discount factor (gamma).
    gamma: f32,                                     
    /// Exploration factor (epsilon). Decays over time for exploitation.
    pub epsilon: f32,                                   

}

impl Agent
{
    /// Initializes a new RL agent.
    ///
    /// Constructs the agent with default hyperparameters (alpha = 0.3, gamma = 0.99, epsilon = 0.99)
    /// and attempts to load an existing Q-table from `qtable.bin` if available.
    ///
    /// Returns:
    ///     A new initialized `Agent`.
    pub fn new() -> Self 
    {
        let mut agent = Agent {
            q_table: HashMap::new(),
            alpha: 0.3,
            gamma: 0.99,
            epsilon: 0.99,
        };
        agent.load();
        agent
    }

    /// Converts a continuous game state into a discrete state representation.
    ///
    /// Reduces the game's high-resolution coordinate system into predefined zones
    /// to make the Q-learning state space manageable.
    ///
    /// Args:
    ///     `state`: A reference to the current `GameState`.
    ///
    /// Returns:
    ///     A tuple `(zone_x_ball, zone_y_ball, dir_x, dir_y, zone_x_racket)` representing the discrete state.
    fn discretize (&self, state: &GameState) -> (i32,i32,i32,i32,i32)
    {
        // Calculate the discrete horizontal zone of the ball
        let zone_x_ball:i32 = state.ball_pos.x as i32 / ZONE_RELATION_W;
        
        // Calculate the discrete vertical zone of the ball
        let zone_y_ball:i32 = state.ball_pos.y as i32 / ZONE_RELATION_H;
        
        // Get the direction sign for Y coordinate (1, -1, or 0)
        let dir_y:i32 = state.ball_speed.speed_y.signum(); 
        
        // Get the direction sign for X coordinate (1, -1, or 0)
        let dir_x:i32 = state.ball_speed.speed_x.signum();  
        
        // Calculate the discrete horizontal zone of the racket
        let zone_x_racket:i32 = state.racket.racket_position.x as i32 / ZONE_RELATION_W;
        
        // Return the tuple representing the full discrete state
        (zone_x_ball, zone_y_ball, dir_x, dir_y, zone_x_racket)
    }

    /// Chooses the next action for the agent to take.
    ///
    /// Uses an epsilon-greedy strategy. With probability `epsilon`, it chooses a random action (exploration).
    /// Otherwise, it selects the action with the highest Q-value for the current state (exploitation).
    ///
    /// Args:
    ///     `state`: A reference to the current `GameState`.
    ///
    /// Returns:
    ///     An `Action` (Left, Right, or Stay) to be performed by the agent.
    pub fn decide(&self, state: &GameState) -> Action
    {
        let mut rng = rand::rng();
        
        // Epsilon-greedy exploration: With probability `epsilon`, take a random action
        if (rng.random::<f32>()) < self.epsilon
        {
            random_action()
        }
        else
        {
            // Exploitation: Find the best known action from the Q-table
            let key = self.discretize(state);
            let q_values = self.q_table.get(&key);

            match q_values
            {
                // If the state hasn't been visited before, default to random action
                None => random_action(),
                Some(values) =>
                {
                    // Find the index of the action with the highest Q-value
                    let max_index = values
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    
                    // Map the numerical index back to the Action enum
                    match max_index 
                    {
                        0 => Action::Left,
                        1 => Action::Right,
                        _ => Action::Stay,
                    }

                },
            }

        }
    }
    

    /// Updates the Q-table based on the agent's experience.
    ///
    /// Applies the Q-learning formula (Bellman equation) considering the state transition
    /// and the received reward.
    ///
    /// Args:
    ///     `state_before`: The game state before taking the action.
    ///     `action`: The action taken.
    ///     `reward`: The numeric reward received after the action.
    ///     `state_after`: The game state resulting from the action.
    pub fn learn(&mut self, state_before: &GameState, action: &Action, reward: f32, state_after: &GameState)
    {
        // 1. Get the discrete key for the state before the action
        let key_before = self.discretize(&state_before);
        
        // 2. Get the discrete key for the state after the action
        let key_after = self.discretize(&state_after);
        
        // 3. Retrieve Q-values for the "after" state, or default to zeros if unvisited
        let q_values_after = self.q_table.get(&key_after).copied().unwrap_or([0.0, 0.0, 0.0]);
        
        // 4. Retrieve mutable Q-values for the "before" state, creating the entry if it's new
        let q_values_before = self.q_table.entry(key_before).or_insert([0.0, 0.0, 0.0]);

        // Map the action performed to its numerical index
        let action_index = match action {
            Action::Left => 0,
            Action::Right => 1,
            Action::Stay => 2,
        };

        // 5. Calculate Bellman equation updates
        let current_q = q_values_before[action_index];
        // Find the maximum Q-value achievable in the next state
        let max_future_q = q_values_after.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        // Q(s, a) = Q(s, a) + alpha * [Reward + gamma * max(Q(s', a')) - Q(s, a)]
        let new_q = current_q + self.alpha * (reward + self.gamma * max_future_q - current_q);
        
        // 6. Update the table with the new Q-value
        q_values_before[action_index] = new_q;
        

    }

    /// Saves the learned Q-table to a binary file.
    ///
    /// Serializes the `q_table` using `bincode` and writes it to `qtable.bin`.
    pub fn save(&self) 
    {
        let bytes = bincode::serialize(&self.q_table).unwrap();
        fs::write("qtable.bin", bytes).unwrap();
    }

    /// Loads the Q-table from a binary file if it exists.
    ///
    /// Reads `qtable.bin` and deserializes it into `self.q_table`. Replaces the current
    /// Q-table entirely if successful.
    pub fn load(&mut self) 
    {
        if let Ok(bytes) = fs::read("qtable.bin") 
        {
            self.q_table = bincode::deserialize(&bytes).unwrap();
        }
    }
}


/// Generates a completely random action.
///
/// Returns:
///     A random `Action` (Left, Right, or Stay) with equal probability.
fn random_action () -> Action
{   
    let mut rng = rand::rng();
    let random_val:f32 = rng.random::<f32>();
    if random_val < 0.33
    {
        Action::Left
    }    
    else if random_val > 0.66
    {
        Action::Right
    }
    else
    {
        Action::Stay
    }

}