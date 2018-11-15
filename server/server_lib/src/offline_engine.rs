use std::io;
use std::io::prelude::*;
use std::process;

use types::Intersection::*;
use types::*;

impl GameState {
    pub fn new(size: GobanSize, game_type: GameType, start_cond: StartConditions) -> GameState {
        GameState {
            goban: Goban::new(size),
            game_type: game_type,
            start_cond: start_cond,
            turn: 0,
            player: Player1,
        }
    }
}

pub static ERR_INVALID_NUMBER: &'static str = "Error : Invalid Number";
pub static ERR_STDIN_FAILED: &'static str = "Error : failed to read stdin";
pub static ERR_EMPTY_LINE: &'static str = "Error : empty string";

fn parse_nb_from_stdin() -> Result<Size, &'static str> {
    let stdin = io::stdin();
    let it = stdin.lock().lines().next();
    match it {
        Some(line) => match line {
            Ok(s) => match s.parse::<Size>() {
                Ok(nb) => Ok(nb),
                Err(_) => Err(ERR_INVALID_NUMBER),
            },
            Err(_) => Err(ERR_STDIN_FAILED),
        },
        Option::None => Err(ERR_EMPTY_LINE),
    }
}

fn parse_input() -> Move {
    let mut loop_read = true;
    let mut play: Move = (0, 0);

    while loop_read {
        println!("Input line");
        match parse_nb_from_stdin() {
            Ok(nb) => {
                play.0 = nb;
                loop_read = false;
            }
            Err(err) => println!("{}", err),
        }
    }
    loop_read = true;
    while loop_read {
        println!("Input column");
        match parse_nb_from_stdin() {
            Ok(nb) => {
                play.1 = nb;
                loop_read = false;
            }
            Err(err) => println!("{}", err),
        }
    }
    play
}

#[inline]
fn check_line_right(goban: &Goban, array_index: Size) -> bool {
    let player = goban.board.get(array_index).unwrap();
    if array_index / goban.size != ((array_index + 5) / goban.size) {
        return false;
    }
    for i in array_index + 1..array_index + 5 {
        match goban.board.get(i) {
            Some(val) => {
                if val != player {
                    return false;
                }
            }
            Option::None => return false,
        }
    }
    true
}

#[inline]
fn check_column_down(goban: &Goban, array_index: Size) -> bool {
    let player = goban.board.get(array_index).unwrap();
    for i in 1..5 {
        match goban.board.get(array_index + i * goban.size) {
            Some(val) => {
                if val != player {
                    return false;
                }
            }
            Option::None => return false,
        }
    }
    true
}

#[inline]
fn check_victory(goban: &Goban) -> bool {
    for i in 0..goban.size {
        if let Some(Intersection::None) = goban.board.get(i) {
            continue;
        }
        if check_line_right(&goban, i) == true {
            return true;
        }
        if check_column_down(&goban, i) == true {
            return true;
        }
        /*if check_diagonal_down_right(goban, i) == true {
			return true;
		}
		if check_diagonal_dowm_left(goban, i) == true {
			return true;
		}*/
    }
    false
}

//TODO : handling GameType, StartConditions
pub fn game_loop(mut game_state: GameState) {
    loop {
        let mut loop_read_input = true;

        println!("Player {} turn.\n{}", game_state.player, game_state.goban);
        while loop_read_input {
            match game_state.goban.play(game_state.player, parse_input()) {
                Ok(()) => loop_read_input = false,
                Err(err) => println!("Error : {}", err),
            }
        }
        if check_victory(&game_state.goban) == true {
            println!("Winner {} at turn {}", game_state.player, game_state.turn);
            break;
        }
        match game_state.player {
            Player1 => game_state.player = Player2,
            Player2 => game_state.player = Player1,
            None => {
                println!("GameState player should not be at none, exiting");
                process::exit(1)
            }
        }
        game_state.turn += 1;
    }
}

//Tests
#[cfg(test)]
mod tests {
    use offline_engine::*;

    #[test]
    fn correct_line() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i] = Player1;
        }
        assert_eq!(true, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_1() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i] = Player1;
        }
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_2() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 1..6 {
            goban.board[i] = Player1;
        }
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_3() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i] = Player1;
        }
        goban.board[2] = Player2;
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_4() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 7..20 {
            goban.board[i] = Player1;
        }
        assert_eq!(false, check_line_right(&goban, 7))
    }

    #[test]
    fn correct_column() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        assert_eq!(true, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_1() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i * goban.size] = Player2;
        }
        assert_eq!(false, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_2() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        assert_eq!(false, check_column_down(&goban, goban.size))
    }

    #[test]
    fn incorrect_column_3() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        goban.board[goban.size * 2] = Player1;
        assert_eq!(false, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_4() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 5..8 {
            goban.board[i * goban.size] = Player2;
        }
        assert_eq!(false, check_column_down(&goban, 5 * goban.size))
    }
}
