use std::fmt;

use crate::types::*;

//Intersection Display Trait
impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intersection::Player1 => write!(f, "1"),
            Intersection::Player2 => write!(f, "2"),
            Intersection::None => write!(f, "None"),
        }
    }
}

//Goban Display Trait
const UP_LEFT: char = '\u{250C}';
const UP_RIGHT: char = '\u{2510}';
const UP_SEP: char = '\u{252C}';
const UP: [char; 3] = [UP_LEFT, UP_RIGHT, UP_SEP];

const MID_LEFT: char = '\u{251C}';
const MID_RIGHT: char = '\u{2524}';
const MID_SEP: char = '\u{253C}';
const MID: [char; 3] = [MID_LEFT, MID_RIGHT, MID_SEP];

const DOWN_LEFT: char = '\u{2514}';
const DOWN_RIGHT: char = '\u{2518}';
const DOWN_SEP: char = '\u{2534}';
const DOWN: [char; 3] = [DOWN_LEFT, DOWN_RIGHT, DOWN_SEP];

const HORI_SEP: char = '\u{2500}';
const VERTI_SEP: char = '\u{2502}';

const RET_LINE: char = '\n';

const PLAYER_1: char = '\u{25CF}'; // black
const PLAYER_2: char = '\u{25CB}'; // white

const SPACE: char = ' ';

#[inline]
fn draw_intersection(x: Size, size: Size, chars: [char; 3]) -> char {
    match x {
        0 => chars[0],
        _ if (x == size - 1) => chars[1],
        _ => chars[2],
    }
}

impl fmt::Display for Goban {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display: String = String::new();

        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(value) = self.board.get(y * self.size + x) {
                    match value {
                        Intersection::Player1 => display.push(PLAYER_1),
                        Intersection::Player2 => display.push(PLAYER_2),
                        Intersection::None => {
                            if y == 0 {
                                display.push(draw_intersection(x, self.size, UP));
                            } else if y == self.size - 1 {
                                display.push(draw_intersection(x, self.size, DOWN));
                            } else {
                                display.push(draw_intersection(x, self.size, MID));
                            }
                        }
                    }
                } else {
                    panic!("This should not be possible");
                }
                display.push(HORI_SEP);
            }
            display.pop();
            display.push(RET_LINE);

            // draw separator
            if y != self.size - 1 {
                for _ in 0..self.size {
                    display.push(VERTI_SEP);
                    display.push(SPACE);
                }
                display.pop();
                display.push(RET_LINE);
            }
        }

        write!(f, "{}", display)
    }
}

//Goban Methods
pub static ERR_OUTSIDE_BOARD: &'static str = "Move outside goban board";
pub static ERR_NOT_EMPTY: &'static str = "Intersection is not empty";

impl Goban {
    pub fn new(size: GobanSize) -> Goban {
        Goban {
            size: size as Size,
            board: vec![Intersection::None; size as Size * size as Size],
        }
    }

    fn is_move_valid(&self, pos: Move) -> Result<(), &'static str> {
        // out of bounds check
        if pos.0 >= self.size || pos.1 >= self.size {
            return Err(ERR_OUTSIDE_BOARD);
        }

        // TODO: implement other case of invalid move detection
        //  - double-three case

        // check if it is empty
        match self.get(pos) {
            Some(Intersection::None) => Ok(()),
            Some(_) => Err(ERR_NOT_EMPTY),
            None => Err(ERR_OUTSIDE_BOARD),
        }
    }

    //pos Tuple => 0 = line | 1 = column
    pub fn set(&mut self, pos: Move, player: Intersection) {
        self.board[pos.0 * self.size + pos.1] = player;
    }

    pub fn get(&self, pos: Move) -> Option<&Intersection> {
        self.board.get(pos.0 * self.size + pos.1)
    }

    pub fn play(&mut self, player: Intersection, pos: Move) -> Result<Size, &'static str> {
        self.is_move_valid(pos)?;
        // TODO: captures
        self.set(pos, player);
        Ok(1)
    }
}

//Tests
#[cfg(test)]
mod tests {
    use crate::goban::*;
    use crate::types::Intersection::*;
    use crate::types::{Goban, GobanSize, Size};

    #[test]
    fn new_working() {
        let goban = Goban::new(GobanSize::Small);

        assert_eq!(goban.size, GobanSize::Small as Size);
        assert_eq!(
            None,
            goban.board[GobanSize::Small as Size * GobanSize::Small as Size - 1]
        );
    }

    #[test]
    fn out_of_bound_play_x() {
        let mut goban = Goban::new(GobanSize::Medium);

        assert_eq!(goban.size, GobanSize::Medium as Size);
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.play(Player1, (14, 0)));
    }

    #[test]
    fn out_of_bound_play_y() {
        let mut goban = Goban::new(GobanSize::Medium);

        assert_eq!(goban.size, GobanSize::Medium as Size);
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.play(Player1, (0, 14)));
    }

    #[test]
    fn valid_play() {
        let mut goban = Goban::new(GobanSize::Large);

        assert_eq!(goban.size, GobanSize::Large as Size);
        assert_eq!(Ok(1), goban.play(Player1, (10, 5)));
    }
}
