use game::{Game, DealSize};

fn main() {
    let mut game = Game::new(DealSize::Three);
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    game.deal();
    println!("{:#?}", game);
}
