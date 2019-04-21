use crate::types::Intersection::Player1;
use crate::types::*;

impl GameArgs {
    pub fn new() -> GameArgs {
        Default::default()
    }
}

impl Default for GameArgs {
    fn default() -> GameArgs {
        GameArgs {
            board_size: GobanSize::Large,
            game_type: GameType::PlayerVsPlayer,
            start_cond: StartConditions::Standard,
            debug: false,
        }
    }
}

impl GameState {
    pub fn new(game_args: GameArgs) -> GameState {
        GameState {
            goban: Goban::new(game_args.board_size),
            game_type: game_args.game_type,
            start_cond: game_args.start_cond,
            turn: 0,
            player: Player1,
            p1_stone_taken: 0,
            p2_stone_taken: 0,
            debug: game_args.debug,
            used_intersection: 0,
        }
    }
}