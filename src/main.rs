mod menu;
mod structs;

use std::io::stdin;

const INPUT: &mut String = &mut String::new();

struct Game {
    save: structs::Save,
}

fn main() {
    let mut option: u8;

    println!("NomadSim v0.2.0\n@msanlli - 2023\n--------------------\n\nWelcome to NomadSim!\n\n1. New Game\n\
    2. Load Game\n\n>>> ");

    loop {
        let input = stdin().read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(num) => {
                option = num;
                if option == 1 {
                    menu::new_game()
                } else if option == 2 {} else {
                    println!("ERR_option_not_found");
                    continue;
                };
            }
            Err(_) => {
                println!("ERR_reading_line");
                continue;
            }
        };
    };
}


