use game::{Game, DealSize};
// use deck::{Suit};

fn main() {
    let mut game = Game::new(DealSize::One);
    let mut oks: Vec<bool> = Vec::new();

    for _ in 0..2 {
        game.deal();
    }
    // for _ in 0..10 {
    //     oks.push(game.waste_to_foundation(Suit::Diamonds));
    // }

    oks.push(game.waste_to_tableau(2));

    println!("{:#?}", game);
    println!("oks = {:#?}", oks);
}
