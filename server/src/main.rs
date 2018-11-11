extern crate server_lib;

use server_lib::types::{*};
use server_lib::types::Intersection::{*};

fn main() {
	let mut goban = Goban::new(GobanSize::Small);

	goban.play(Player1, (5, 5)).unwrap();
	println!("{}", goban);
}
