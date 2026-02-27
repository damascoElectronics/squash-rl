// mod.rs: traks the state of the game


/***** cosntants *****/

const FIELD_WIDTH: u32 = 800; 	// area of the game
const FIELD_HEIGHT: u32 = 600;	// area of the game
const RACKET_LENGTH: u32 = 20;	// dimetnions of the racket
const RACKET_HEIGHT: u32 = 4;	// dimentions of the racket
const SPACE: u32 = 5;		// sapce between racket and botton of the game area

/***** Structures and enums area *****/

/* speed information */
pub struct Speed{
	speed_x: i32,
	speed_y: i32,
	angle: f32,
}

/* position of the ball in the game */ 
pub struct Position{
	x:u32,
	y:u32,
}

/* Racket Infromation */
pub struct Racket{
	length:u32,
	height:u32,
	curve:u32,
	racket_speed: i32,
	racket_position: Position,
}

/* general infromation of the game */
pub struct GameState
{
	ball_pos:Position,
	ball_speed: Speed,
	racket: Racket,
	score: i32, 
 	active: bool,
}

pub enum Action {
	Left,pub fn move_racket(&mut self, dir:Action){

                match dir{
                        Left => {
                                let new_x_left = self.racket.racket_position.x as i32 - (RACKET_LENGTH as i32 /2) - self.racket.racket_speed;
                                if new_x_left <= 0 {
                                        self.racket.racket_position.x = self.racket.racket_position.x - self.racket.racket_speed;
                                }
                        },
                        Right => {
                                let new_x_right = self.racket.racket_position.x as i32 + (RACKET_LENGTH as i32 /2)) + self.racket.racket_speed;
                                if new_x_right >= FIELD_WIDTH{
                                        self.racket.racket_position.x = self.racket.racket_position.x + self.racket.racket_speed;
                                }
                        },
                        Stay => {
                                return;
                        },
                        _=> return;,
                }
        }
	Right,
	Stay,
}

/* methods and implementations */

impl GameState {
	
	// fn new: intialitate the game with the inital game data
	pub fn new() -> Self {
        	GameState{
			// initial state of the ball's position
			ball_pos: Position {
				x: FIELD_WIDTH/2,
				y: FIELD_HEIGHT - SPACE - RACKET_HEIGHT - 1,
			},
			// initial state of the ball's speed
			ball_speed: Speed{
				speed_x: 1,
				speed_y: 1,
				angle: 0.785398,
			},
			// initial state of the racket 
			racket: Racket {
				length: RACKET_LENGTH,
				height: RACKET_HEIGHT,
				curve: 0,
				racket_speed:1,
				racket_position: Position{
					x: FIELD_WIDTH/2,
					y: FIELD_HEIGHT - SPACE - RACKET_HEIGHT,
				},
			},
			score: 0, 	// score initialitation 
			active: false,	// game start in stop, to move active changes to true
	}
	
	// fn update: create the muvementes for the ball
	pub fn update(&mut self){
		
		if self.active == true{
			// calculation of the new postiton of the ball
                        let new_x:i32 = self.ball_pos.x as i32 + self.ball_speed.speed_x;
                        let new_y:i32 = self.ball_pos.y as i32 + self.ball_speed.speed_y;
			// verification if the ball is LEFT side of the screen, if is there, changes direction
			if new_x == 0 
			{
				self.ball_speed.speed_x = -self.ball_speed.speed_x.abs(); 
			}
                        // verification if the ball is RIGTH side of the screen, if is there, changes direction
			if new_x == FIELD_WIDTH
			{
				self.ball_speed.speed_x = -self.ball_speed.speed_x; 
			}
			// verification if the ball is TOP side of the screen, if is there, changes direction
			if new_y == 0
			{
                                self.ball_speed.speed_y = -self.ball_speed.speed_y;
                        }
                        // verification if the ball has TOUCH THE RACKET , if is touching, changes direction
			if (new_x >= (self.racket.racket_position.x as i32 - (RACKET_LENGTH as i32 /2))) && 
			   (new_x <= (self.racket.racket_position.x as i32 + (RACKET_LENGTH as i32 /2))) && 
			   (new_y == (FIELD_HEIGHT as i32 - SPACE as i32 - RACKET_HEIGHT as i32 - 1))
			{
				self.ball_speed.speed_y = -self.ball_speed.speed_y;
				self.score += 1;
			}
                        // verification if the ball is BOTTOM side of the screen, if is there, END GAME
			if new_y >= FIELD_HEIGHT as i32
			{
				self.active = false;
				return;
			}
			// updating the position of the ball
			self.ball_pos.x = new_x as u32;
			self.ball_pos.y = new_y as u32;
		}
	}
	// fn move_racket: to update the raquet state:
	pub fn move_racket(&mut self, dir:Action){
		
		match dir{
			Left => {
				let new_x_left:i32 = self.racket.racket_position.x as i32 - (RACKET_LENGTH as i32 /2) - self.racket.racket_speed;
				if new_x_left > 0 {
					self.racket.racket_position.x = self.racket.racket_position.x - self.racket.racket_speed as u32;	
				}
			},
    			Right => {
				let new_x_right:i32 = self.racket.racket_position.x as i32 + (RACKET_LENGTH as i32 /2) + self.racket.racket_speed;
				if new_x_right < FIELD_WIDTH as i32{
					self.racket.racket_position.x = self.racket.racket_position.x + self.racket.racket_speed as u32;
				}
			},
			Stay => {
				return;		
			},
		}
	}

}

