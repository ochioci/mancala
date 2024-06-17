#![allow(warnings)]

use crate::ai::eval;

mod ai;
mod game;
mod test;

static CLEAR: bool = true;
static LEFT_AI: bool = true;
static RIGHT_AI: bool = false;
static AI_DEPTH: u8 = 7;
static THREAD_DEBUG: bool = true;
static AI_DEBUG: bool = false;

fn main() {
    if CLEAR {
        print!("{esc}c", esc = 27 as char);
    };
    // let mut test_game= game::Game::default();
    // test_game.play_mancala();
    test::test_ai(eval, eval);
}
