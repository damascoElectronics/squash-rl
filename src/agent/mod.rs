// agent RL
use crate::game::{GameState, FIELD_HEIGHT, FIELD_WIDTH, Action};
use std::collections::HashMap;
use rand::prelude::*;


const NUM_ZONES: u32 = 10;		// Num zones
const ZONE_RELATION_H: i32 = (FIELD_HEIGHT / NUM_ZONES) as i32 ;
const ZONE_RELATION_W: i32 = (FIELD_WIDTH / NUM_ZONES) as i32 ;

pub struct Agent 
{
    // Q-table (zone_x_ball, zone_y_ball, dir_x, dir_y, zone_x_racket)
    q_table: HashMap<(i32,i32,i32,i32,i32), [f32; 3]>,  
    alpha: f32,                                     //  learning rate
    gamma: f32,                                     // discount factor 
    epsilon: f32,                                   // exploration factor

}

impl Agent
{
    pub fn new() -> Self
    {
        Agent
        {
            q_table: HashMap::new(),
            alpha: 0.1,
            gamma: 0.95,
            epsilon: 1.0, // exoliration at 100
        }
    }

    fn discretize (&self, state: &GameState) -> (i32,i32,i32,i32,i32)
    {
        
        let zone_x_ball:i32 = state.ball_pos.x as i32 / ZONE_RELATION_W;
        let zone_y_ball:i32 = state.ball_pos.y as i32 / ZONE_RELATION_H;
        let dir_y:i32 = state.ball_speed.speed_y.signum(); 
        let dir_x:i32 = state.ball_speed.speed_x.signum(); 
        let zone_x_racket:i32 = state.racket.racket_position.x as i32 / ZONE_RELATION_W;
        (zone_x_ball, zone_y_ball, dir_x, dir_y, zone_x_racket)
    }

    pub fn decide(&self, state: &GameState) -> Action
    {
        let mut rng = rand::rng();
        if (rng.random::<f32>()) < self.epsilon
        {
            random_action()
        }
        else
        {
            let key = self.discretize(state);
            let q_values = self.q_table.get(&key);

            match q_values
            {
                None => random_action(),
                Some(values) =>
                {
                let max_index = values
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                
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
    

    pub fn learn(&mut self, state_before: &GameState, action: &Action, reward: f32, state_after: &GameState)
        {
            
            let key_before = self.discretize(&state_before);
            let key_after = self.discretize(&state_after);
            let q_values_after = self.q_table.get(&key_after).copied().unwrap_or([0.0, 0.0, 0.0]);
            let q_values_before = self.q_table.entry(key_before).or_insert([0.0, 0.0, 0.0]);

            let action_index = match action {
                Action::Left => 0,
                Action::Right => 1,
                Action::Stay => 2,
            };

            let current_q = q_values_before[action_index];
            let max_future_q = q_values_after.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let new_q = current_q + self.alpha * (reward + self.gamma * max_future_q - current_q);
            q_values_before[action_index] = new_q;
            

        }

}


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