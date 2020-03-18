use game::{Game, DealSize};
use deck::{Suit};
use std::io;

fn main() {
    let mut game = Game::new(DealSize::One);
    let mut oks: Vec<bool> = Vec::new();

    for _ in 0..11 {
        game.deal();
    }
    for _ in 0..1 {
        oks.push(game.waste_to_foundation(Suit::Diamonds));
    }
    for _ in 0..133 {
        game.deal();
    }
    for _ in 0..13 {
        oks.push(game.waste_to_foundation(Suit::Clubs));
    }

    loop {
        print!("\x1B[2J");
        println!("SOLITAIRE\n\n\n\n\n");
        println!("{}", game);
        println!("Type a command: ");

        let mut command = String::new();

        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
    }
    // println!("oks = {:#?}", oks);
}
