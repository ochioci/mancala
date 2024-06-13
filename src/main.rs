#![allow(warnings)]

use crate::ai::dfs;

mod game;
mod ai;

static CLEAR: bool = true;
fn main() {
    if CLEAR { print!("{esc}c", esc = 27 as char); };
    let mut test_game= game::Game::default();
    // test_game.play_mancala();
    // test_game.display();
    // test_game.swap_turn();
    let a = dfs(test_game, 0);
    for g in a {
        g.display();
        println!("----------------");
    }

}







