mod console;
mod parser;

use parser::{Command, MoveSpec};
use crate::game::{Game, DealSize};

pub fn start() {
    let mut deal_size: Option<DealSize> = None;

    while deal_size.is_none() {
        console::print_main_menu();

        let input = console::read_line("Choose deal size (1 or 3):");

        deal_size = match input.parse() {
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
    let mut messages: Vec<String> = Vec::new();

    loop {
        console::print_game(&game);
        console::print_messages(&mut messages);
        let input = console::read_line("command: ");

        match Command::parse(&input) {
            Ok(command) => {
                match command {
                    Command::Deal => {
                        game.deal();
                        messages.push(String::from("Dealt cards."));
                    },
                    Command::Help => {
                        messages.push(String::from("d or deal => deals cards"));
                        messages.push(String::from("m or move => moves cards"));
                        messages.push(String::from("h or help => display this help"));
                    },
                    Command::Move(spec) => {
                        match spec {
                            MoveSpec::WasteToFoundation(suit) => {
                                let success = game.waste_to_foundation(suit);
                                if success {
                                    messages.push(String::from(format!("Moved from waste to foundation {}.", suit)));
                                } else {
                                    messages.push(String::from(format!("Cannot move from waste to foundation {}.", suit)));
                                }
                            },
                            MoveSpec::WasteToTableau(n) => {
                                let success = game.waste_to_tableau(n);
                                if success {
                                    messages.push(String::from(format!("Moved from waste to tableau {}.", n)));
                                } else {
                                    messages.push(String::from(format!("Cannot move move from waste to tableau {}.", n)));
                                }
                            },
                            MoveSpec::TableauToFoundation(n, suit) => {
                                let success = game.tableau_to_foundation(n, suit);
                                if success {
                                    messages.push(String::from(format!("Moved from tableau {} to foundation {}.", n, suit)));
                                } else {
                                    messages.push(String::from(format!("Cannot move from tableau {} to foundation {}.", n, suit)));
                                }
                            },
                            MoveSpec::TableauToTableau(n, m, size) => {
                                let success = game.tableau_to_tableau(n, m, size);
                                if success {
                                    messages.push(String::from(format!("Moved from foundation {} to foundation {}.", n, m)));
                                } else {
                                    messages.push(String::from(format!("Cannot move from foundation {} to foundation {}.", n, m)));
                                }
                            },
                            MoveSpec::FoundationToTableau(suit, n) => {
                                let success = game.foundation_to_tableau(suit, n);
                                if success {
                                    messages.push(String::from(format!("Moved from foundation {} to tableau {}.", suit, n)));
                                } else {
                                    messages.push(String::from(format!("Cannot move from foundation {} to tableau {}.", suit, n)));
                                }
                            }
                        }
                    }
                }
            },
            Err(message) => messages.push(message),
        }
    }
}