#![allow(warnings)]

use crate::ai::{best_move, eval};
use crate::game::{Game, State};
use std::env;
use crate::test::test_ai;


mod ai;
mod game;
mod test;

static CLEAR: bool = false;
static LEFT_AI: bool = true;
static RIGHT_AI: bool = false;
static AI_DEPTH: u8 = 6;
static THREAD_DEBUG: bool = false;
static AI_DEBUG: bool = false;

fn main() {
    if CLEAR {
        print!("{esc}c", esc = 27 as char);
    };
    let args: Vec<String> = env::args().collect();
    let mut prev_arg = String::from("");
    let mut play_best_move = false;
    let mut play_input_move = false;
    let mut cli_input = String::from("");
    let mut depth = 6;
    let mut moveToMake: u8 = 14; //sentinel value please kill me
    for a in &args {
        let possibleDepth = a.parse::<u8>();
        if prev_arg == "/d" {
            match possibleDepth {
                Ok(d) => {
                    depth = d;
                },
                Err(E) => {}
            }
        }
        if prev_arg == "/pbm" {
            // println!("{}", a);
            play_input_move = false;
            play_best_move = true;
            cli_input = a.clone();
        }
        if prev_arg == "/pim" {
            play_best_move = false;
            play_input_move = true;
            cli_input = a.clone()
        }
        if prev_arg == "/m" {
            moveToMake = a.clone().parse().unwrap();
        }
        prev_arg = a.clone();
    }
    if play_input_move {
        let mut g = Game::from(&cli_input);
        match g.get_state() {
            State::LeftToMove => {
                g.make_move(moveToMake - 1);
            }
            State::RightToMove => {
                g.make_move(13- moveToMake);
            }
            _ => {}
        }
        println!("{}", String::from(&g));
    }
    else if play_best_move {
        let mut g = Game::from(&cli_input);
        let bestMove = best_move(g.clone(), eval, depth);
        // println!("{}", String::from(&g));
        // println!("Depth: {}", depth);
        println!("{}", bestMove.0);
        println!("{}", bestMove.1);
        match g.get_state() {
            State::LeftToMove => {
                g.make_move(bestMove.0 - 1);
            }
            State::RightToMove => {
                g.make_move(13- bestMove.0);
            }
            _ => {}
        }
        println!("{}", String::from(&g));
    } else {
        test_ai(eval, eval);
    }
}
