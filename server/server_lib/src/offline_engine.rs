use std::process;
use std::io;
use std::io::prelude::*;

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

pub static ERR_INVALID_NUMBER : &'static str = "Error : Invalid Number";
pub static ERR_STDIN_FAILED : &'static str = "Error : failed to read stdin";

fn parse_nb_from_stdin() -> Result<Size, &'static str> {
	let stdin = io::stdin();
	let it = stdin.lock().lines().next(); 
	if let Ok(line) = it.unwrap() {
		if let Ok(nb) = line.parse::<Size>() {
			Ok(nb)
		} else {
			Err(ERR_INVALID_NUMBER)
		}
	} else {
		Err(ERR_STDIN_FAILED)
	}
}

fn parse_input() -> Move {
	let mut loop_read = true;
	let mut play : Move = (0, 0);
	
	while loop_read {
		println!("Input line");
		match parse_nb_from_stdin() {
			Ok(nb) => {	play.0 = nb;
						loop_read = false; },
			Err(err) => println!("{}", err),
		}
	}
	loop_read = true;
	while loop_read {
		println!("Input column");
		match parse_nb_from_stdin() {
			Ok(nb) => {	play.1 = nb;
						loop_read = false; },
			Err(err) => println!("{}", err),
		}
	}
	play
}

//TODO
fn check_victory(_goban: &Goban) -> bool {
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
            println!(
                "Winner player {} at turn {}",
                game_state.player, game_state.turn
            );
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
