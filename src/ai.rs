use std::cmp::PartialEq;
use crate::{AI_DEPTH, CLEAR};
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

pub fn predict_complexity(game : Game) -> i32 {
    let mut count : i32 = 0;
    let options = next_positions(game);
    for o in options {
        count += next_positions(o).len() as i32;
    }
    // (18/(count as f32).sqrt() as i32) + AI_DEPTH as i32
    AI_DEPTH.into()
}

pub fn next_positions(game : Game) -> Vec<Game> {
    //Returns the game state after every possible turn is taken
    //accounts for extra turns
    let mut moves : Vec<usize> = game.get_moves();
    let mut options : Vec<Game> = vec![];
    let to_move: State = game.get_state().clone();
    for m in moves {
        let mut b: Game = game.clone();
        b.make_move(m as u8);
        if (b.move_index == 100) {
            b.move_index = m;
        }
        // println!("{}", m);
        if (b.get_state() == to_move) {
            // println!("turn didn't change!");
            options.append(&mut next_positions(b.clone()))
        } else {
            options.push(b);
        }

    }
    options
}

pub fn best_move_search(game: Game, depth: u8) -> (Vec<Game>, Vec<i16>) {
    if (depth < 1) {
        let mut moves = next_positions(game);
        let evals : Vec<i16> = moves.iter().map(|m| m.eval()).collect();
        (moves, evals)
    } else {
        let mut moves = next_positions(game);
        let evals : Vec<i16> = moves.iter().map(|m| {
            let d = m.eval();
            let o = best_move_search(m.clone(), depth-1).1;
            let bestEval: Option<&i16> = match m.get_state() {
                State::LeftToMove => {
                    o.iter().max()
                },
                State::RightToMove => {
                    o.iter().min()
                }
                State::LeftWins => {
                    Some(&100)
                }
                State::RightWins => {
                    Some(&-100)
                }
                State::Draw => {
                    Some(&0)
                }
            };
            // let bestEval : Option<&i16> = o.iter().max();
            match bestEval {
                Some(bestEval) => bestEval,
                _ => &d
            }.clone()
        }).collect();
        (moves, evals)
    }

}

pub fn best_move_test(game : Game, depth : u8) {
    let moves = next_positions(game);
    for m in moves {
        println!("Move: {}", m.move_index);
        m.display();
    }
}

pub fn best_move(game: Game) -> (u8, i16) {
    let (games, evals) = best_move_search(game.clone(), predict_complexity(game.clone()) as u8);
    let mut moves= match game.get_state() {
        State::LeftToMove => {
            [-999; 6]
        },
        _ => {
            [999; 6]
        }
    };

    for g in 0..games.len() {
        let index = match game.get_state(){
            State::LeftToMove => {
                // moves = [-999; 6];
                games[g].move_index
            },
            State::RightToMove => {
                // moves = [999; 6];
                12-games[g].move_index
            }
            _ => {return (100,999); }
        };
        match game.get_state() {
            State::LeftToMove => {
                if evals[g] > moves[index] {
                    moves[index] = evals[g];
                }
            },
            State::RightToMove => {
                if evals[g] < moves[index] {
                    moves[index] = evals[g];
                }
            }
            _ => {}
        }

    }


    let mut max = 0;
    for i in 1..moves.len() {
        match game.get_state() {
            State::LeftToMove => {
                if moves[i] > moves[max] {
                    max = i
                }
            },
            State::RightToMove => {
                if moves[i] < moves[max] {
                    max = i
                }
            }
            _ => {}
        };
    };

    return ((max + 1) as u8, moves[max]);


    // for m in 0..moves.len() {
    //     println!("Move: {}, Eval: {}", m+1, moves[m]);
    // }

    // for g in 0..games.len() {
    //     // games[g].display();
    //     println!("Move: {}, Eval: {}", match game.get_state(){
    //         State::LeftToMove => {
    //             games[g].move_index+1
    //         },
    //         State::RightToMove => {
    //             13-games[g].move_index
    //         }
    //         _ => return
    //     }, evals[g]);
    // }
}

pub fn display_moves(game: Game) {
    let o = best_move(game);
    println!("Best move: {}, Evaluation: {}", o.0, o.1)
}