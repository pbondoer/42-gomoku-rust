extern crate server_lib;

use server_lib::offline_engine;
use server_lib::types::*;

fn main() {
    let game_state = GameState::new(
        GobanSize::Small,
        GameType::PlayerVsPlayer,
        StartConditions::Standard,
    );

    offline_engine::game_loop(game_state);
}
