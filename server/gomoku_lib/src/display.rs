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
