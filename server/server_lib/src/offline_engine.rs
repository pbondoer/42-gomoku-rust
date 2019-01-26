use std::io;
use std::io::prelude::*;
use std::process;

use crate::types::Intersection::*;
use crate::types::*;

impl GameArgs {
	pub fn new() -> GameArgs {
		GameArgs {
			board_size : GobanSize::Large,
			game_type : GameType::PlayerVsPlayer,
			start_cond : StartConditions::Standard,
			debug : false,
		}
	}
}

impl GameState {
    pub fn new(game_args : GameArgs) -> GameState {
        GameState {
            goban: Goban::new(game_args.board_size),
            game_type: game_args.game_type,
            start_cond: game_args.start_cond,
            turn: 0,
            player: Player1,
			p1_stone_taken : 0,
			p2_stone_taken : 0,
			debug : game_args.debug,
			used_intersection : 0,
        }
    }
}

pub static ERR_INVALID_NUMBER: &'static str = "Error : Invalid Number";
pub static ERR_STDIN_FAILED: &'static str = "Error : failed to read stdin";
pub static ERR_EMPTY_LINE: &'static str = "Error : empty string";

#[inline]
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

#[inline]
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
fn check_diagonal_down_right(goban: &Goban, array_index: Size) -> bool {
    let player = goban.board.get(array_index).unwrap();
    let start_line = array_index / goban.size;
    for i in 1..5 {
        let new_index = array_index + i * goban.size + i;
        if (start_line + i) != (new_index / goban.size) {
            return false;
        }
        match goban.board.get(new_index) {
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
fn check_diagonal_down_left(goban: &Goban, array_index: Size) -> bool {
    let player = goban.board.get(array_index).unwrap();
    let start_line = array_index / goban.size;
    for i in 1..5 {
        let new_index = array_index + i * goban.size - i;
        if (start_line + i) != (new_index / goban.size) {
            return false;
        }
        match goban.board.get(array_index + i * goban.size - i) {
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
fn check_board_victory(goban: &Goban) -> bool {
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
        if check_diagonal_down_right(goban, i) == true {
            return true;
        }
        if check_diagonal_down_left(goban, i) == true {
            return true;
        }
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
		//TODO : change this when capture detection added
		game_state.used_intersection += 1;
		if (game_state.player == Player1 && game_state.p1_stone_taken >= STONE_TAKEN_MAX) || 
			(game_state.player == Player1 && game_state.p2_stone_taken >= STONE_TAKEN_MAX) {
            println!("Winner {} at turn {}", game_state.player, game_state.turn);
            process::exit(1)
		}
        if check_board_victory(&game_state.goban) == true {
            println!("Winner {} at turn {}", game_state.player, game_state.turn);
            process::exit(1)
        }
		if game_state.used_intersection >= game_state.goban.size {
            println!("Draw at turn {}", game_state.turn);
            process::exit(1)
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
    use crate::offline_engine::*;

    #[test]
    fn correct_line() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i] = Player1;
        }
        //println!("{}", goban);
        assert_eq!(true, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_1() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i] = Player1;
        }
        //println!("{}", goban);
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_2() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 1..6 {
            goban.board[i] = Player1;
        }
        //println!("{}", goban);
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_3() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i] = Player1;
        }
        goban.board[2] = Player2;
        //println!("{}", goban);
        assert_eq!(false, check_line_right(&goban, 0))
    }

    #[test]
    fn incorrect_line_4() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 7..20 {
            goban.board[i] = Player1;
        }
        //println!("{}", goban);
        assert_eq!(false, check_line_right(&goban, 7))
    }

    #[test]
    fn correct_column() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(true, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_1() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i * goban.size] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_2() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_column_down(&goban, goban.size))
    }

    #[test]
    fn incorrect_column_3() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size] = Player2;
        }
        goban.board[goban.size * 2] = Player1;
        //println!("{}", goban);
        assert_eq!(false, check_column_down(&goban, 0))
    }

    #[test]
    fn incorrect_column_4() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 5..8 {
            goban.board[i * goban.size] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_column_down(&goban, 5 * goban.size))
    }

    #[test]
    fn correct_diag_down_right() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size + i] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(true, check_diagonal_down_right(&goban, 0))
    }

    #[test]
    fn incorrect_diag_down_right_1() {
        let mut goban = Goban::new(GobanSize::Small);
        let start_pos = 8;
        for i in 0..5 {
            goban.board[i * goban.size + start_pos + i] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_right(&goban, start_pos))
    }

    #[test]
    fn incorrect_diag_down_right_2() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i * goban.size + i] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_right(&goban, 0))
    }

    #[test]
    fn incorrect_diag_down_right_3() {
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size + i] = Player2;
        }
        goban.board[goban.size * 2 + 2] = Player1;
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_right(&goban, 0))
    }

    #[test]
    fn incorrect_diag_down_right_4() {
        let mut goban = Goban::new(GobanSize::Small);
        let start_line = 6;
        for i in 0..3 {
            goban.board[(start_line + i) * goban.size + (i + start_line)] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(
            false,
            check_diagonal_down_right(&goban, start_line * goban.size)
        )
    }

    #[test]
    fn correct_diag_down_left() {
        let start_pos = 8;
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size - i + start_pos] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(true, check_diagonal_down_left(&goban, start_pos))
    }

    #[test]
    fn incorrect_diag_down_left_1() {
        let mut goban = Goban::new(GobanSize::Small);
        let start_pos = 0;
        for i in 0..5 {
            goban.board[i * goban.size + start_pos - i] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_left(&goban, start_pos))
    }

    #[test]
    fn incorrect_diag_down_left_2() {
        let start_pos = 8;
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..4 {
            goban.board[i * goban.size - i + start_pos] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_left(&goban, start_pos))
    }

    #[test]
    fn incorrect_diag_down_left_3() {
        let start_pos = 8;
        let mut goban = Goban::new(GobanSize::Small);
        for i in 0..5 {
            goban.board[i * goban.size - i + start_pos] = Player2;
        }
        goban.board[goban.size * 2 - 2 + start_pos] = Player1;
        //println!("{}", goban);
        assert_eq!(false, check_diagonal_down_left(&goban, start_pos))
    }

    #[test]
    fn incorrect_diag_down_left_4() {
        let mut goban = Goban::new(GobanSize::Small);
        let start_line = 6;
        let start_column = 8;
        for i in 0..3 {
            goban.board[(start_line + i) * goban.size - i + start_column] = Player2;
        }
        //println!("{}", goban);
        assert_eq!(
            false,
            check_diagonal_down_right(&goban, start_line * goban.size)
        )
    }
}
