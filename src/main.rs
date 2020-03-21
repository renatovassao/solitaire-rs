use std::io::{self, Write};

mod deck;
mod game;
mod parser;

// use deck::Suit;
use game::{Game, DealSize};
use parser::{Parser, Command};

fn clear_screen() {
    print!("\x1B[2J");
}

fn print_header() {
    println!("        SOLITAIRE\n\n\n\n");
}

fn print_main_menu() {
    print!("





        SOLITAIRE
     by Renato Vassão








Choose deal size (1 or 3):");
    flush();
}

fn flush() {
    io::stdout().flush().expect("flush failed!");
}

fn main() {
    let mut deal_size: Option<DealSize> = None;

    while deal_size.is_none() {
        clear_screen();
        print_main_menu();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        deal_size = match input.trim().parse() {
            Ok(num) => {
                match num {
                    1 => Some(DealSize::One),
                    3 => Some(DealSize::Three),
                    _ => continue,
                }
            },
            Err(_) => continue,
        };
    }

    let mut game = Game::new(deal_size.unwrap());
    let mut message = String::from("");

    loop {
        clear_screen();
        print_header();
        println!("{}", game);
        println!("{}", message);
        print!("command: ");
        flush();

        let input = read_line();

        match Command::parse(&input) {
            Ok(command) => {
                match command {
                    Command::Deal => {
                        game.deal();
                        message = String::from("Dealt cards.");
                    },
                    Command::Help => {
                        message = String::from("
d or deal => deals cards
m or move => moves cards
h or help => display this help")
                    },
                    Command::WasteToFoundation(suit) => {
                        game.waste_to_foundation(suit);
                        message = String::from(format!("Moved from waste to foundation {}.", suit));
                    },
                    Command::WasteToTableau(tableau_number) => {
                        game.waste_to_tableau(tableau_number);
                        message = String::from(format!("Moved from waste to tableau {}.", tableau_number));
                    },
                    Command::TableauToFoundation(tableau_number, suit) => {
                        game.tableau_to_foundation(tableau_number, suit);
                        message = String::from(format!("Moved from tableau {} to foundation {}.", tableau_number, suit));
                    },
                    Command::TableauToTableau(from_tableau_number, to_tableau_number, size) => {
                        game.tableau_to_tableau(from_tableau_number, to_tableau_number, size);
                        message = String::from(format!("Moved from foundation {} to foundation {}.", from_tableau_number, to_tableau_number));
                    },
                    Command::FoundationToTableau(suit, tableau_number) => {
                        game.foundation_to_tableau(suit, tableau_number);
                        message = String::from(format!("Moved from foundation {} to tableau {}.", suit, tableau_number));
                    },
                }
            },
            Err(error) => message = error,
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()

}