use std::fmt;

use crate::types::*;

//Intersection Display Trait
impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intersection::Player1 => write!(f, "Player1"),
            Intersection::Player2 => write!(f, "Player2"),
            Intersection::None => write!(f, "None"),
        }
    }
}

//Goban Display Trait
const UP_LEFT: char = '\u{2554}';
const UP_RIGHT: char = '\u{2557}';
const UP_SEP: char = '\u{2566}';
const HORI_SEP: char = '\u{2550}';
const VERTI_SEP: char = '\u{2551}';
const DOWN_LEFT: char = '\u{255A}';
const DOWN_RIGHT: char = '\u{255D}';
const DOWN_SEP: char = '\u{2569}';
const RET_LINE: char = '\n';
const MID_LEFT: char = '\u{2560}';
const MID_RIGHT: char = '\u{2563}';
const MID_SEP: char = '\u{256C}';
const PLAYER_1: char = 'X';
const PLAYER_2: char = 'O';
const PLAYER_NONE: char = ' ';
const ERROR: char = 'E';

#[inline]
fn first_line(buff: &mut String, size: Size) {
    buff.push(UP_LEFT);
    for x in 0..size {
        buff.push(HORI_SEP);
        if x != size - 1 {
            buff.push(UP_SEP);
        }
    }
    buff.push(UP_RIGHT);
    buff.push(RET_LINE);
}

#[inline]
fn last_line(buff: &mut String, size: Size) {
    buff.push(DOWN_LEFT);
    for x in 0..size {
        buff.push(HORI_SEP);
        if x != size - 1 {
            buff.push(DOWN_SEP);
        }
    }
    buff.push(DOWN_RIGHT);
    buff.push(RET_LINE);
}

#[inline]
fn seperation_line(buff: &mut String, size: Size) {
    buff.push(MID_LEFT);
    for x in 0..size {
        buff.push(HORI_SEP);
        if x != size - 1 {
            buff.push(MID_SEP);
        }
    }
    buff.push(MID_RIGHT);
    buff.push(RET_LINE);
}

#[inline]
fn goban_line(buff: &mut String, cur_line: Size, goban: &Goban) {
    buff.push(VERTI_SEP);
    for x in 0..goban.size {
        if let Some(value) = goban.board.get(cur_line * goban.size + x) {
            match value {
                Intersection::Player1 => buff.push(PLAYER_1),
                Intersection::Player2 => buff.push(PLAYER_2),
                Intersection::None => buff.push(PLAYER_NONE),
            }
        } else {
            buff.push(ERROR);
        }
        buff.push(VERTI_SEP);
    }
    buff.push(RET_LINE);
}

impl fmt::Display for Goban {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //alloc
        let vertical_size = self.size * 2 + 1;
        let horizontal_size = vertical_size + 1;
        let mut to_display: String = String::with_capacity(horizontal_size * vertical_size);

        //display
        first_line(&mut to_display, self.size);
        for x in 0..self.size {
            goban_line(&mut to_display, x, &self);
            if x != self.size - 1 {
                seperation_line(&mut to_display, self.size);
            }
        }
        last_line(&mut to_display, self.size);
        write!(f, "{}", to_display)
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

    //TODO : implement other case of invalid move detection
    fn is_move_valid(goban: &Goban, pos: Size) -> Result<(), &'static str> {
        match goban.board.get(pos) {
            Some(Intersection::None) => Ok(()),
            Some(_) => Err(ERR_NOT_EMPTY),
            None => return Err(ERR_OUTSIDE_BOARD),
        }
    }

    //pos Tuple => 0 = line | 1 = column
    pub fn play(&mut self, player: Intersection, pos: Move) -> Result<(), &'static str> {
        let board_pos = pos.0 * self.size + pos.1;

        Goban::is_move_valid(&self, board_pos)?;
        self.board[board_pos] = player;
        Ok(())
    }
}

//Tests
#[cfg(test)]
mod tests {
    use goban::*;
    use types::Intersection::*;
    use types::{Goban, GobanSize, Size};

    #[test]
    fn new_working() {
        let goban = Goban::new(GobanSize::Small);

        assert_eq!(goban.size, GobanSize::Small as Size);
        assert_eq!(
            None,
            *goban
                .board
                .get(GobanSize::Small as Size * GobanSize::Small as Size - 1)
                .unwrap()
        );
    }

    #[test]
    fn out_of_bound_play() {
        let mut goban = Goban::new(GobanSize::Medium);

        assert_eq!(goban.size, GobanSize::Medium as Size);
        assert_eq!(Err(ERR_OUTSIDE_BOARD), goban.play(Player1, (14, 0)));
    }

    #[test]
    fn valid_play() {
        let mut goban = Goban::new(GobanSize::Large);

        assert_eq!(goban.size, GobanSize::Large as Size);
        assert_eq!(Ok(()), goban.play(Player1, (10, 5)));
    }
}
