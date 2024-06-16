use std::num::ParseIntError;
use crate::ai::{display_moves, predict_complexity};
use crate::{AI_DEPTH, CLEAR, LEFT_AI, RIGHT_AI};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Clone, Debug)]
pub(crate) struct Game {
    game_state: State,
    pub(crate) board: [u8; 14],
    pub(crate) move_index: usize
}
#[derive(Default, Debug, Clone)]
pub(crate) enum State {
    #[default]
    LeftToMove,
    RightToMove,
    LeftWins,
    RightWins,
    Draw
}
impl Game {

    pub fn get_state(&self) -> &State {
        &self.game_state
    }

    pub fn eval(&self) -> i16 {
        (self.board[6] as i16) - ( self.board[13] as i16)
    }
    pub fn get_moves(&self) -> Vec<usize> {
        match self.game_state {
            State::LeftToMove => {
                let mut a = vec![];
                for i in 0..6 {
                    if self.board[i] > 0 {a.push(i);};
                };
                a
            },
            State::RightToMove => {
                let mut a = vec![];
                for i in 7..13 {
                    if self.board[i] > 0 {a.push(i);};
                };
                a
            }
            _ => vec![]
        }
    }
    pub fn play_mancala(&mut self) {
        self.handle_turn();
        println!("{}", match self.game_state {
            State::LeftWins => "Top player wins!",
            State::RightWins => "Bottom player wins!",
            State::Draw => "Draw",
            _ => "This shouldn't be happening!"
        })
    }

    fn game_over_check (&mut self) {
        if self.board[0..6].into_iter().sum::<u8>() == 0 || self.board[7..13].into_iter().sum::<u8>() == 0 {
            self.board[6] += self.board[0..6].into_iter().sum::<u8>();
            self.board[13] += self.board[7..13].into_iter().sum::<u8>();
            if self.board[13] > self.board[6] {
                self.game_state = State::RightWins;
            } else if self.board[13] == self.board[6] {
                self.game_state = State::Draw;
            } else {
                self.game_state = State::LeftWins;
            }
        }
        // self.display();
    }

    fn handle_turn(&mut self) {
        self.game_over_check();
        println!("AI Thinking... \nDepth: {}", AI_DEPTH);
        match self.game_state {
            State::LeftToMove => {
                if LEFT_AI {
                    let t = SystemTime::now();
                    (display_moves(self.clone()));
                    match t.elapsed() {
                        Ok(elapsed) => {
                            println!("Finished in: {:?} seconds.", elapsed);
                        },
                        _ => {
                            println!("time error????");
                        }
                    }
                };
                println!("Top (LEFT) to move");
                println!("You go in this direction --->>>");
                println!("  1  2  3  4  5  6  ");
                println!("  |  |  |  |  |  |  ");
                self.display();
                "top"
            },
            State::RightToMove => {
                if RIGHT_AI {
                    let t = SystemTime::now();
                    (display_moves(self.clone()));
                    match t.elapsed() {
                        Ok(elapsed) => {
                            println!("Finished in: {:?} seconds.", elapsed);
                        },
                        _ => {
                            println!("time error????");
                        }
                    }
                };
                println!("Bottom (RIGHT) to move");
                println!("<<<--- You go in this direction");
                self.display();
                println!("  |  |  |  |  |  |  ");
                println!("  1  2  3  4  5  6  ");
                "bottom"
            },
            _ => {
                self.display();
                return; }
        };

        println!("Input your move:");
        let mut line = String::new();
        let mut input = std::io::stdin().read_line(&mut line);
        match input {
            Ok(_) => {
                let l: Result<i32, ParseIntError> = line.trim().parse::<i32>();
                match l {
                    Ok(l) => {
                        match self.game_state {
                            State::LeftToMove => {
                                if (0..6).contains(&(l-1)) && self.board[(l-1) as usize] > 0 {
                                    self.make_move((l-1) as u8);
                                }
                                if CLEAR { print!("{esc}c", esc = 27 as char); };
                                self.handle_turn();
                            },
                            State::RightToMove => {
                                if (7..13).contains(&(13-l)) && self.board[(13-l) as usize] > 0 {
                                    self.make_move((13-l) as u8);
                                }
                                if CLEAR { print!("{esc}c", esc = 27 as char) };
                                self.handle_turn();
                            }
                            _ => {
                                println!("game over!");
                            }
                        };
                    },
                    Err(_) => {
                        print!("{esc}c", esc = 27 as char);
                        println!("Invalid input!");
                        self.handle_turn();
                    }
                }
            },
            _ => {
                print!("{esc}c", esc = 27 as char);
                self.handle_turn();
            }
        }


    }

    pub(crate) fn display(&self) {
        let mut top = String::from("  ");
        let mut bottom = String::from("  ");
        for i in 0..6 {
            top += &self.board[i].to_string();
            top += &"  ";
            bottom += &self.board[12-i].to_string();
            bottom += &"  ";
        }
        println!("{}", top);
        println!("{}                  {}", self.board[13], self.board[6]);
        println!("{}", bottom);
    }

    pub(crate) fn swap_turn(&mut self) {
        match self.game_state {
            State::LeftToMove => self.game_state = State::RightToMove,
            State::RightToMove => self.game_state = State::LeftToMove,
            _ => return
        }
    }

    pub(crate) fn make_move(&mut self, target : u8) {
        let mut marbles = self.board[target as usize];
        self.board[target as usize] = 0;
        let mut index : i8 = target as i8;
        match self.game_state {
            State::LeftToMove => index+=1,
            State::RightToMove => index+=1,
            _ => return
        }
        let (pit, real_opp_pit) = match self.game_state { // 1-indexed
            State::LeftToMove => (7, 13),
            State::RightToMove => (0, 6),
            _ => return
        };
        while (marbles > 0) {
            if index != real_opp_pit {
                self.board[index as usize] +=1;
                marbles -=1;
            }

            if (index == 13) {
                index = 0;
            } else {
                index +=1;
            }
        };
        let landing_index:usize = if index > 0 {
            (index-1) as usize
        } else {
            13
        };
        let real_pit: usize = 13 - &pit;
        // println!("index: {}, pit: {}, value in index: {}", landing_index, pit, self.board[landing_index as usize]);
        if (self.board[landing_index] == 1 && landing_index != real_pit) {
            // println!("stealable");
            let (valid_range, opposite_landing_index) = match self.game_state {
                State::LeftToMove => (0..6, 12-landing_index),
                State::RightToMove => (7..13, 12-landing_index),
                _ => return
            };
            if valid_range.contains(&landing_index) && self.board[opposite_landing_index] > 0{
                // println!("stealing");
                self.board[real_pit] += self.board[landing_index] + self.board[opposite_landing_index];
                self.board[opposite_landing_index] = 0;
                self.board[landing_index] = 0;


            }
            // println!("stealable!!");
        }



        if (index as usize != pit) { //free turn mechanic
            self.swap_turn();
        }
    }

    pub(crate) fn small() -> Self {
        Self {
            board: [1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0],
            game_state: State::default(),
            move_index: 100
        }
    }

    pub(crate) fn sparse() -> Self {
        Self {
            board: [1,0,1,0,1,0,0,1,0,1,0,1,0,0],
            game_state: State::default(),
            move_index: 100
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
            game_state: State::default(),
            move_index: 100
        }
    }
}