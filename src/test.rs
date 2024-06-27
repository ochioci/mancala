use std::time::SystemTime;

use crate::ai::best_move;
use crate::AI_DEPTH;
use crate::game::{Game, State};

pub(crate) fn test_ai(left_eval: fn(&Game) -> i16, right_eval: fn(&Game) -> i16) {
    let mut my_game: Game = Game::default();
    loop {
        my_game.display();
        let t = SystemTime::now();
        my_game.game_over_check();
        match my_game.get_state() {
            State::LeftToMove => {
                let my_move = best_move(my_game.clone(), left_eval, AI_DEPTH);
                println!("Best move: {}, Evaluation: {}", my_move.0, my_move.1);
                my_game.make_move(my_move.0 - 1);
            }
            State::RightToMove => {
                let my_move = best_move(my_game.clone(), right_eval, AI_DEPTH);
                println!("Best move: {}, Evaluation: {}", my_move.0, my_move.1);
                my_game.make_move(13 - my_move.0);
            }
            _ => {
                println!("game oiver?????");

                return;
            }
        }
        match t.elapsed() {
            Ok(elapsed) => {
                println!("Finished in: {:?}.", elapsed);
            }
            _ => {
                println!("time error????");
            }
        }
    }
}
