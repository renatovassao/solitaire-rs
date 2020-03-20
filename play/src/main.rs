use game::{Game, DealSize};
use deck::{Suit};
// use std::io;

fn main() {
    let mut game = Game::new(DealSize::Three);
    let mut _oks: Vec<bool> = Vec::new();

    game.deal();
    for _ in 0..7 {
        game.deal();
    }

    // for _ in 0..11 {
    //     game.deal();
    // }
    // for _ in 0..13 {
    //     game.deal();
    // }
    for _ in 0..13 {
        _oks.push(game.waste_to_foundation(Suit::Clubs));
    }
    for _ in 0..1 {
        _oks.push(game.waste_to_foundation(Suit::Diamonds));
    }
    game.deal();
    game.deal();

    game.tableau_to_tableau(2, 5, 1);
    game.tableau_to_tableau(5, 1, 2);
    game.tableau_to_tableau(2, 5, 1);
    game.tableau_to_tableau(1, 2, 3);
    game.tableau_to_tableau(5, 1, 2);
    game.tableau_to_foundation(5, Suit::Spades);
    game.tableau_to_foundation(5, Suit::Spades);
    game.tableau_to_foundation(5, Suit::Spades);
    game.tableau_to_tableau(1, 5, 2);
    game.foundation_to_tableau(Suit::Clubs, 1);
    game.foundation_to_tableau(Suit::Clubs, 1);
    game.tableau_to_tableau(7, 1, 1);

    // loop {
        print!("\x1B[2J");
        println!("SOLITAIRE\n\n\n\n\n");
        println!("{}", game);
        println!("Type a command: ");

        // let mut command = String::new();

        // io::stdin().read_line(&mut command)
        //     .expect("Failed to read line");
    // }
    // println!("_oks = {:#?}", _oks);
}
