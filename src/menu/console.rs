use std::io::{self, Write};
use crate::game;

fn flush() {
    io::stdout().flush().expect("flush failed!");
}

fn clear_screen() {
    print!("\x1B[2J");
    flush();
}

fn print_header() {
    clear_screen();
    println!("        SOLITAIRE\n\n\n\n");
}

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    flush();

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()

}

pub fn print_game(game: &game::Game) {
    print_header();
    println!("{}", game);
}


pub fn print_main_menu() {
    clear_screen();
    print!("





        SOLITAIRE
     by Renato Vassão







");
    flush();
}

pub fn print_messages(messages: &mut Vec<String>) {
    messages.reverse();
    while let Some(message) = messages.pop() {
        println!("{}", message);
    }
}
