#![allow(warnings)]

fn main() {
    println!("Hello, world!");
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

        pub fn display(&self) {
            let mut top = String::from("  ");
            let mut bottom = String::from("  ");
            for i in 0..6 {
                top += &self.board[i].to_string();
                top += &"  ";
                bottom += &self.board[12-i].to_string();
                bottom += &"  ";
            }
            println!("{:?}", self.game_state);
            println!("{}", top);
            println!("{}                  {}", self.board[13], self.board[6]);
            println!("{}", bottom);
            println!();
        }


    }

    impl Game {

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
            let (pit, opp_pit) = match self.game_state {
                State::LeftToMove => (6, 13),
                State::RightToMove => (13, 6),
                _ => return
            };
            while (marbles > 0) {
                if index != opp_pit {
                    self.board[index as usize] +=1;
                }
                marbles -=1;
                if (index == 0) {
                    index = 13;
                } else if (index == 13) {
                    index = 0
                } else {
                    match self.game_state {
                        State::LeftToMove => index+=1,
                        State::RightToMove => index+=1,
                        _ => return
                    }
                }
            };
            match self.game_state {
                State::LeftToMove => index+=1,
                State::RightToMove => index+=1,
                _ => return
            }
            if (index != pit) { //free turn mechanic
                self.swap_turn();
            }
            // println!("{}     {}", self.board[index as usize], index);
            if self.board[index as usize] == 1 &&
                (match self.game_state {
                    State::RightToMove => ((0..6).contains(&index)),
                    State::LeftToMove => ((7..13).contains(&index)),
                    _ => return

                }) { //steal mechanic
                self.board[pit as usize] += self.board[index as usize] + self.board[(index - 12).abs() as usize];
                self.board[index as usize] = 0;
                self.board[(index-12).abs() as usize] = 0;
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
    test_game.display();
    // test_game.board[4] = 0;
    test_game.make_move(2);
    test_game.display();
    // test_game.make_move(9);
    test_game.display();

}







