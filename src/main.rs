use std::{io, time::Duration, thread};
use device_query::{DeviceQuery, DeviceState, Keycode};
use clearscreen;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    let option: u8;
    let device_state = DeviceState::new();

    clearscreen::clear().expect("ERR_cleaning_screen");

    println!("NomadSim v0.1.0\n@msanlli - 2023\n\n--------------------\n\nWelcome to NomadSim!\n\nChoose an option:\n \
    1. New Game\n2. Load Game\n");

    loop {
        input.clear();
        stdin.read_line(input).expect("ERR_reading_file");
        match input.trim().parse() {
            Ok(num) => {option = num; break},
            Err(_) => continue,
        };
    };

    if option == 1 {
        println!("You accomplished your dreams and worked as a pilot for a major airline, but after\na couple years \
        you realised that the commercial aviation wasn't for you, you\nneeded an adventure.\n\nYou decided to quit \
        your job and start a company from scratch, you started to\npull some strings and one friend of yours told you \
        about a logistic shortage in\nIceland because of the difficult terrain and the lack of roads,so you decided \
        to\nstart a company that would help the locals transport goods quick and safely.\n\n Press any key to \
        continue...\n");

        thread::sleep(Duration::from_secs(1));

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if !keys.is_empty() {
                menu();
                break;
            };
        };

        } else if option == 2 {
            println!("ERR_not_implemented");
        } else {
        println!("ERR_option_not_found");
    };
}

fn menu() {
    println!("ERR_not_implemented")
}