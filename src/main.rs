#![allow(warnings)]

use crate::ai::{best_move_search, next_positions};

mod game;
mod ai;

static CLEAR: bool = true;
static LEFT_AI: bool = true;
static RIGHT_AI: bool = true;
static AI_DEPTH: u8 = 8;
fn main() {
    if CLEAR { print!("{esc}c", esc = 27 as char); };
    let mut test_game= game::Game::default();
    test_game.play_mancala();



}







