#![allow(warnings)]

use crate::ai::{best_move, eval};
use crate::game::Game;
use std::env;
use crate::test::test_ai;


mod ai;
mod game;
mod test;

static CLEAR: bool = true;
static LEFT_AI: bool = true;
static RIGHT_AI: bool = false;
static AI_DEPTH: u8 = 9;
static THREAD_DEBUG: bool = false;
static AI_DEBUG: bool = false;

fn main() {
    if CLEAR {
        print!("{esc}c", esc = 27 as char);
    };
    let args: Vec<String> = env::args().collect();
    let mut prevArg = String::from("");
    let mut doCli = false;
    let mut cliInput = String::from("");
    for a in &args {
        if prevArg == "cli" {
            // println!("{}", a);
            doCli = true;
            cliInput = a.clone();
        }
        prevArg = a.clone();
    }
    if doCli {
        let g = Game::from(&cliInput);
        println!("{:?}", best_move(g, eval))
    } else {
        test_ai(eval, eval);
    }
}
