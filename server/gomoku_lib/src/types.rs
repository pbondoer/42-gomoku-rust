#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Intersection {
    Player1,
    Player2,
    None,
}

pub static STONE_TAKEN_MAX: Size = 10;

pub type Error = &'static str;
pub type Delta = isize;
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
    pub size: Size,
    pub board: Board,
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
    pub goban: Goban,
    pub game_type: GameType,
    pub start_cond: StartConditions,
    pub turn: Size,
    pub player: Intersection,
    pub p1_stone_taken: Size,
    pub p2_stone_taken: Size,
    pub debug: bool,
    pub used_intersection: Size,
}

pub struct GameArgs {
    pub board_size: GobanSize,
    pub game_type: GameType,
    pub start_cond: StartConditions,
    pub debug: bool,
}
