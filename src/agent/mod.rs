// agent RL
use crate::game::{GameState, FIELD_HEIGHT, FIELD_WIDTH};
use std::collections::HashMap;

const NUM_ZONES: u32 = 10;		// Num zones
const ZONE_RELATION_H: i32 = (FIELD_HEIGHT / NUM_ZONES) as i32 ;
const ZONE_RELATION_W: i32 = (FIELD_WIDTH / NUM_ZONES) as i32 ;

struct Agent 
{
    q_table: HashMap<(i32,i32,i32,i32), [f32; 3]>,  //a Q-table (zone_x_ball, zone_y_ball, dir_y, zone_x_racket)
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

    fn discretize (&mut self, state: &GameState) -> (i32,i32,i32,i32)
    {
        
        let zone_x_ball:i32 = state.ball_pos.x as i32 / ZONE_RELATION_W;
        let zone_y_ball:i32 = state.ball_pos.y as i32 / ZONE_RELATION_H;
        let dir_y:i32 = state.ball_speed.speed_y.signum(); 
        let zone_x_racket:i32 = state.racket.racket_position.x as i32 / ZONE_RELATION_W;
        (zone_x_ball, zone_y_ball, dir_y, zone_x_racket)


    }
}