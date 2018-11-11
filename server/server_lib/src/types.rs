#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Intersection {
	Player1,
	Player2,
	None,
}

pub type Size = usize;
pub type Board = Vec<Intersection>;
pub type Move = (Size, Size);

#[derive(Copy, Clone)]
pub enum GobanSize {
	Small = 9,
	Medium = 14,
	Large = 19, 
}

pub struct Goban {
	pub size : Size,
	pub board : Board,
}

pub enum StartConditions {
	Standard,
	Pro,
	LongPro,
	Swap,
	Swap2,
}

pub enum GameType {
	PlayerVsPlayer,
	PlayerVsComputer,
	ComputerVsComputer,
}

pub struct GameState {
	pub goban : Goban,
	pub game_type : GameType,
	pub start_cond : StartConditions,
	pub turn : Size,
	pub player : Intersection,
}
