extern crate server_lib;

use std::env;
use std::process;

use server_lib::offline_engine;
use server_lib::types::*;

static STR_HELP : &'static str = "Gomoku offline args :

Generic parameters :
	--help | -h : Displays help
	--debug | -v : Displays debug [default = OFF]

Goban size :
	--small : 9x9 Goban
	--medium : 14x14 Goban
	--large : 19x19 Goban [default]

Ruleset :
	--std : Standard rule set [default]
	--pro : Pro rule set
	--lrop : LongPro rule set
	--swap : Swap rule set
	--swap2 : Swap2 rule set

Game type :
	--PlayerVsPlayer : Two players game [default]
	--PlayerVsComputer : One player game
	--ComputerVsComputer : Zero player game";

fn parse_args(args : &[String]) -> GameState {
	let mut game_args = GameArgs::new();

	for s in args.iter() {
		match s.as_str() {
			"--help" | "-h" => {
				println!("{}", STR_HELP);
				process::exit(0); },
			"--debug" | "-v" => game_args.debug = true,
			"--PlayerVsPlayer" => game_args.game_type = GameType::PlayerVsPlayer,
			"--PlayerVsComputer" => game_args.game_type = GameType::PlayerVsComputer,
			"--ComputerVsComputer" => game_args.game_type = GameType::ComputerVsComputer,
			"--std" => game_args.start_cond = StartConditions::Standard,
			"--pro" => game_args.start_cond = StartConditions::Pro,
			"--lpro" => game_args.start_cond = StartConditions::LongPro,
			"--swap" => game_args.start_cond = StartConditions::Swap,
			"--swap2" => game_args.start_cond = StartConditions::Swap2,
			"--small" => game_args.board_size = GobanSize::Small,
			"--medium" => game_args.board_size = GobanSize::Medium,
			"--large" => game_args.board_size = GobanSize::Large,
			_ => {},
		}
	}
    GameState::new(game_args)
}

fn main() {
	let args : Vec<String> = env::args().collect();
	let game_state = parse_args(&args[1..args.len()]);
    offline_engine::game_loop(game_state);
}
