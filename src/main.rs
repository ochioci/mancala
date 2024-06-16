#![allow(warnings)]

use crate::ai::{best_move_search, eval, eval_2, next_positions};

mod game;
mod ai;
mod test;

static CLEAR: bool = true;
static LEFT_AI: bool = true;
static RIGHT_AI: bool = true;
static AI_DEPTH: u8 = 6;
static THREAD_DEBUG: bool = false;
static AI_DEBUG : bool = true;

fn main() {
    if CLEAR { print!("{esc}c", esc = 27 as char); };
    let mut test_game= game::Game::default();
    test_game.play_mancala();
    // test::test_ai(eval, eval_2);


}







