#![allow(warnings)]

use std::num::ParseIntError;
static CLEAR: bool = true;
fn main() {
    if CLEAR { print!("{esc}c", esc = 27 as char); };

    struct Game {
        game_state: State,
        board: [u8; 14],
    }
    #[derive(Default, Debug)]
    enum State {
        #[default]
        LeftToMove,
        RightToMove,
        GameOver
    }

    impl Game {

        pub fn handleTurn(&mut self) {
            let playerSide = match self.game_state {
                State::LeftToMove => {
                    println!("Top to move");
                    println!("You go in this direction --->>>");
                    println!("  1  2  3  4  5  6  ");
                    println!("  |  |  |  |  |  |  ");
                    self.display();
                    "top"
                },
                State::RightToMove => {
                    println!("Bottom to move");
                    println!("<<<--- You go in this direction");
                    self.display();
                    println!("  |  |  |  |  |  |  ");
                    println!("  1  2  3  4  5  6  ");
                    "bottom"
                },
                _ => "smth has gone horribly wrong"
            };

            println!("Input your move:");
            let mut line = String::new();
            let mut input = std::io::stdin().read_line(&mut line);
            match input {
                Ok(mut input) => {
                    let l: Result<i32, ParseIntError> = line.trim().parse::<i32>();
                    match l {
                        Ok(l) => {
                            match self.game_state {
                                State::LeftToMove => {
                                    self.make_move((l-1) as u8);
                                    if CLEAR { print!("{esc}c", esc = 27 as char); };
                                    self.handleTurn();
                                },
                                State::RightToMove => {
                                    self.make_move((13-l) as u8);
                                    if CLEAR { print!("{esc}c", esc = 27 as char) };
                                    self.handleTurn();
                                }
                                State::GameOver => {
                                    println!("game over!");
                                }
                            };
                        },
                        Err(l) => {
                            print!("{esc}c", esc = 27 as char);
                            println!("Invalid input!");
                            self.handleTurn();
                        }
                    }
                },
                _ => {
                    print!("{esc}c", esc = 27 as char);
                    self.handleTurn();
                }
            }


        }

        pub fn display(&self) {
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

        fn swap_turn(&mut self) {
            match self.game_state {
                State::LeftToMove => self.game_state = State::RightToMove,
                State::RightToMove => self.game_state = State::LeftToMove,
                _ => return
            }
        }

        fn make_move(&mut self, target : u8) {
            let mut marbles = self.board[target as usize];
            self.board[target as usize] = 0;
            let mut index : i8 = target as i8;
            match self.game_state {
                State::LeftToMove => index+=1,
                State::RightToMove => index+=1,
                _ => return
            }
            let (pit, opp_pit, real_opp_pit) = match self.game_state { // 1-indexed
                State::LeftToMove => (7, 0, 13),
                State::RightToMove => (0, 7, 6),
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
    }

    impl Default for Game {
        fn default() -> Self {
            Self {
                board: [4,4,4,4,4,4,0,4,4,4,4,4,4,0],
                game_state: State::default()
            }
        }
    }

    let mut test_game: Game = Default::default();
    test_game.handleTurn();

}







