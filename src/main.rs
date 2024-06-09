fn main() {
    println!("Hello, world!");
    struct Game {
        game_state: State,
        left: [u8; 6],
        right: [u8; 6],
        left_pit: u8,
        right_pit: u8,
    }
    #[derive(Default)]
    enum State {
        #[default]
        LeftToMove,
        RightToMove,
        GameOver
    }

    impl Game {
        pub fn display(&self) {
            println!("    {:?}    ", self.left_as_string());
            println!("  {:?}                    {:?}", self.left_pit, self.right_pit);
            println!("    {:?}    ", self.right_as_string());
        }

        fn left_as_string(&self) -> String {
            let mut out = String::from("");
            for i in 0..(self.left.len()) {
                out += &self.right[i].to_string();
                out += "  ";
            }
            out
        }
        fn right_as_string(&self) -> String {
            let mut out = String::from("");
            for i in 0..(self.left.len()) {
                out += &self.right[i].to_string();
                out += "  ";
            }
            out
        }

        pub fn leftMove(&self) {

        }
    }
    impl Default for Game {
        fn default() -> Self {
            Self {
                left: [4;6],
                right: [4;6],
                left_pit: 0,
                right_pit: 0,
                game_state: State::default()
            }
        }
    }

    let test_game : Game = Default::default();
    test_game.display();

}







