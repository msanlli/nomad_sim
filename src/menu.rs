use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use clearscreen::clear;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::structs::Save;

const INPUT: &mut String = &mut String::new();

pub fn new_game() {
    clear.expect("ERR_clearing_screen");

    println!("\nYou accomplished your dreams and worked as a pilot for a major airline, but after\n\
    a couple years you realised that the commercial aviation wasn't for you, you\nneeded an \
    adventure.\n\nYou decided to quit our job and start a company from scratch, you started to\n\
    pull some strings and one friend of yours told you about a logistic shortage in\nIceland \
    because of the difficult terrain and the lack of roads,so you decided to\nstart a company that \
    would help the locals transport goods quick and safely.\n\nPress any key to continue...\n");

    sleep(Duration::from_secs(1));

    loop {
        let keys: Vec<Keycode> = DeviceState::new().get_keys();
        if !keys.is_empty() {
            new_save();
            break;
        };
    };

    /*
    for client_file, fleet_file = new_save(){
         let mut save = Save {
             client: client_file,
             fleet: fleet_file,
         };
     }
     */
}

fn new_save() { // -> (csv file, csv file)
    print!("*Phone ringing*\n\n???- Hello? Hey, I'm Noah, the owner of the airstrip's hangar! \
    You'll have to\npardon me but I forgot your name again... Can you remind it to me please?\n>>> \
    ");

    let pilot_handle = loop {
        let input = stdin().read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(str) => {
                break str;
            }
            Err(_) => {
                println!("ERR_reading_line");
                continue;
            }
        };
    };

    // TODO! Create client.csv file

    println!("Now it's time to select difficulty!:\n\n1. Easy\n    You start off at Keflavik \
    International Airport (BIKF) which is the biggest\n    airport in Iceland. Your initial \
    aircraft will be a Cessna 208B and your\n    initial debt is none, you'll even have 100.000€ \
    in your account!\n"); // TODO! Create more modes

    loop {
        let input = stdin().read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(num) => {
                let option: u8 = num;
                if option == 1 {
                    let wtr = csv::Writer::from.path("{:?}.csv", pilot_handle);

                    wtr.write_record(&[
                        pilot_handle, "100000", "1", "0", "false"
                    ]).expect("ERR_writing_csv");
                };
                // Other difficulties
            }
            Err(_) => {
                println!("ERR_reading_line");
                continue;
            }
        };
    };

}
