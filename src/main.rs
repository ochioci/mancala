#![allow(warnings)]

mod game;
static CLEAR: bool = true;
fn main() {
    if CLEAR { print!("{esc}c", esc = 27 as char); };
    let mut test_game= game::Game::small();
    test_game.play_mancala();

}







