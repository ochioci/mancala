use std::cmp::PartialEq;
use crate::CLEAR;
use crate::game::{Game, State};

impl PartialEq<State> for &State {
    fn eq(&self, other: &State) -> bool {
       match self {
           State::LeftToMove => {
               match other {
                   State::LeftToMove => true,
                   _ => false
               }
           }
           State::RightToMove => {
               match other {
                   State::RightToMove => true,
                   _ => false
               }
           }
           _=> false
       }
    }
}

pub fn dfs(game : Game, depth : u8) -> Vec<Game> {
    //Returns the game state after every possible turn is taken
    //accounts for extra turns
    let mut moves : Vec<usize> = game.get_moves();
    let mut options : Vec<Game> = vec![];
    let to_move: State = game.get_state().clone();
    for m in moves {
        let mut b: Game = game.clone();
        b.make_move(m as u8);
        // println!("{}", m);
        if (b.get_state() == to_move) {
            println!("turn didn't change!");
            options.append(&mut dfs(b.clone(), depth))
        } else {
            options.push(b);
        }

    }
    options
}