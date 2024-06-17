use std::cmp::PartialEq;
use std::thread;
use std::thread::JoinHandle;

use crate::game::{Game, State};
use crate::{AI_DEBUG, AI_DEPTH, THREAD_DEBUG};

impl PartialEq<State> for &State {
    fn eq(&self, other: &State) -> bool {
        match self {
            State::LeftToMove => match other {
                State::LeftToMove => true,
                _ => false,
            },
            State::RightToMove => match other {
                State::RightToMove => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub fn predict_complexity(game: Game) -> i32 {
    let mut count: i32 = 0;
    let options = next_positions(game);
    for o in options {
        count += next_positions(o).len() as i32;
    }
    // (18/(count as f32).sqrt() as i32) + AI_DEPTH as i32
    AI_DEPTH.into()
}

pub fn next_positions(game: Game) -> Vec<Game> {
    //Returns the game state after every possible turn is taken
    //accounts for extra turns
    let mut moves: Vec<usize> = game.get_moves();
    let mut options: Vec<Game> = vec![];
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

fn best_index(evals: Vec<i16>, state: &State) -> usize {
    let mut best = 0;
    for i in 0..evals.len() {
        if (match state {
            State::LeftToMove => evals[i] < evals[best],
            State::RightToMove => evals[i] > evals[best],
            _ => false,
        }) {
            best = i;
        }
    }
    best
}
pub fn best_move_search(
    game: Game,
    depth: u8,
    eval_func: fn(&Game) -> i16,
) -> (Vec<Game>, Vec<i16>) {
    let next = next_positions(game);
    let mut threads: Vec<JoinHandle<(Vec<Game>, Vec<i16>)>> = vec![];
    for n in next {
        if THREAD_DEBUG {
            (println!("Thread Created!"))
        };
        let d = depth.clone();
        threads.push(thread::spawn(move || {
            best_move_search_helper(n, d, eval_func, -1000, 1000)
        }));
    }

    let mut out = vec![];
    for a in threads {
        match a.join() {
            Ok(a) => {
                if THREAD_DEBUG {
                    (println!("Thread Finished!"))
                };
                out.push(a)
            }
            Err(a) => {
                println!("{:?}", a);
            }
        }
    }
    let mut result_games = vec![];
    let mut result_evals = vec![];
    for (games, evals) in out {
        let t = games.get(0);
        match t {
            Some(t) => {
                let local_best = best_index(evals.clone(), games[0].get_state()); //crash here
                result_games.push(games[local_best].clone());
                result_evals.push(evals[local_best].clone());
            }
            _ => {
                std::process::exit(0);
                // return (vec![], vec![]);
            }
        }
    }

    for i in 0..result_games.len() {
        if AI_DEBUG {
            println!(
                "Move: {}, Eval: {}",
                result_games[i].move_index, result_evals[i]
            )
        };
    }

    (result_games, result_evals)
}
pub fn eval(game: &Game) -> i16 {
    (game.board[6] as i16) - (game.board[13] as i16)
}

pub fn eval_2(game: &Game) -> i16 {
    let mut o = (game.board[6] as i16) - (game.board[13] as i16);
    let (left, right): ([u8; 6], [u8; 6]) = (
        game.board[0..6].try_into().unwrap(),
        game.board[7..13].try_into().unwrap(),
    );
    let mut adjust: i16 = 0;
    for i in 0..left.len() {
        let reqForFreeTurn: i16 = (6 - i as i16);
        adjust = adjust - i16::abs(left[i] as i16 - reqForFreeTurn);
    }
    for i in 0..right.len() {
        let reqForFreeTurn: i16 = (i as i16 + 1);
        adjust = adjust + i16::abs((right[i] as i16 - reqForFreeTurn));
    }
    o += (adjust / 10);
    o
}

pub fn best_move_search_helper(
    game: Game,
    depth: u8,
    eval_func: fn(&Game) -> i16,
    mut alpha: i16,
    mut beta: i16,
) -> (Vec<Game>, Vec<i16>) {
    if (depth < 1) {
        let mut moves = next_positions(game);
        let evals: Vec<i16> = moves.iter().map(|m| eval_func(m)).collect();
        (moves, evals)
    } else {
        let maximizing = game.get_state() == State::LeftToMove;

        let mut moves = next_positions(game);
        let mut newMoves = vec![];
        let mut ct = 0;
        let mut evals2: Vec<i16> = vec![];
        for m in moves.clone() {
            let d = eval_func(&m);
            let o = best_move_search_helper(m.clone(), depth - 1, eval_func, alpha, beta).1;
            let bestEvalOption: Option<&i16> = match m.get_state() {
                State::LeftToMove => o.iter().max(),
                State::RightToMove => o.iter().min(),
                State::LeftWins => Some(&100),
                State::RightWins => Some(&-100),
                State::Draw => Some(&0),
            };
            let bestEval = match bestEvalOption {
                Some(bestEval) => bestEval,
                _ => &d,
            };
            if (maximizing) {
                if (bestEval > &alpha) {
                    evals2.push(*bestEval);
                    newMoves.push(moves[ct].clone());
                    alpha = i16::max(*bestEval, alpha)
                }
            } else {
                if (bestEval < &beta) {
                    evals2.push(*bestEval);
                    newMoves.push(moves[ct].clone());
                    beta = i16::min(*bestEval, beta)
                }
            };
            ct+=1;
        }
        (newMoves, evals2)
    }
}

pub fn best_move(game: Game, eval_func: fn(&Game) -> i16) -> (u8, i16) {
    let (games, evals) = best_move_search(game.clone(), AI_DEPTH, eval_func);
    let mut moves = match game.get_state() {
        State::LeftToMove => [-999; 6],
        _ => [999; 6],
    };

    for g in 0..games.len() {
        let index = match game.get_state() {
            State::LeftToMove => {
                // moves = [-999; 6];
                games[g].move_index
            }
            State::RightToMove => {
                // moves = [999; 6];
                12 - games[g].move_index
            }
            _ => {
                return (100, 999);
            }
        };
        match game.get_state() {
            State::LeftToMove => {
                if evals[g] > moves[index] {
                    moves[index] = evals[g];
                }
            }
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
            }
            State::RightToMove => {
                if moves[i] < moves[max] {
                    max = i
                }
            }
            _ => {}
        };
    }
    return ((max + 1) as u8, moves[max]);
}

pub fn display_moves(game: Game) {
    let o = best_move(game, eval);
    println!("Best move: {}, Evaluation: {}", o.0, o.1)
}
