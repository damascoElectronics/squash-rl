// state of the game

const FIELD_WIDTH: u32 = 800;
const FIELD_HEIGHT: u32 = 600;
const RACKET_LENGTH: u32 = 20;
const RACKET_HEIGHT: u32 = 4;
const SPACE: u32 = 5;

pub struct Speed{
	speed_x: i32,
	speed_y: i32,
	angle: f32,
}

pub struct Position{
	x:u32,
	y:u32,
}

pub struct Racket{
	length:u32,
	height:u32,
	curve:u32,
	racket_speed: i32,
	racket_position: Position,
}


pub struct GameState
{
	ball_pos:Position,
	ball_speed: Speed,
	racket: Racket,
	score: i32, 
 	active: bool,
}



impl GameState {
	pub fn new() -> Self {
        	GameState{
			ball_pos: Position {
				x: FIELD_WIDTH/2,
				y: FIELD_HEIGHT - SPACE - RACKET_HEIGHT - 1,
			},
			ball_speed: Speed{
				speed_x: 1,
				speed_y: 1,
				angle: 0.785398,
			},
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
			score: 0,
			active: false,	
		}
	

}
