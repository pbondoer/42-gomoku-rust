#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Intersection {
	Player1,
	Player2,
	None,
}

pub type Size = usize;
pub type Board = Vec<Intersection>;

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
