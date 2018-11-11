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

//TODO
fn parse_input() -> Move {
    println!("Always 0,0");
    (0, 0)
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
