use std::{io, time::Duration, thread};
use device_query::{DeviceQuery, DeviceState, Keycode};
use clearscreen;

const STDIN: io::Stdin = io::stdin();
const INPUT: &mut String = &mut String::new();
const DEVICE_STATE: DeviceState = DeviceState::new();

fn clear() { clearscreen::clear().expect("ERR_cleaning_screen"); }

struct Save {
    client: String,
    fleet: String,
}

fn main() {
    let mut option: u8;

    println!("NomadSim v0.2.0\n@msanlli - 2023\n--------------------\n\nWelcome to NomadSim!\n\n1. New Game\n\
    2. Load Game\n\n>>> ");

    loop {
        let input = STDIN.read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(num) => {
                option = num;
                if option == 1 {
                    new_game()
                } else if option == 2 {

                } else {
                    println!("ERR_option_not_found");
                    continue
                };
            },
            Err(_) => {println!("ERR_reading_line"); continue;},
        };
    };
}

fn new_game() {
    println!("\nYou accomplished your dreams and worked as a pilot for a major airline, but after\na couple years \
        you realised that the commercial aviation wasn't for you, you\nneeded an adventure.\n\nYou decided to quit \
        your job and start a company from scratch, you started to\npull some strings and one friend of yours told you \
        about a logistic shortage in\nIceland because of the difficult terrain and the lack of roads,so you decided \
        to\nstart a company that would help the locals transport goods quick and safely.\n\nPress any key to \
        continue...\n");

    thread::sleep(Duration::from_secs(1));

    loop {
        let keys: Vec<Keycode> = DEVICE_STATE.get_keys();
        if !keys.is_empty() { new_save(); break; };
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
    print!("*Phone ringing*\n\n???- Hello? Hey, I'm Noah, the owner of the airstrip's hangar! You'll have to\npardon \
    me but I forgot your name again... Can you remind it to me please?\n>>> ");

    loop {
        let input = STDIN.read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(str) => {
                let pilot_handle = str;
                // TODO! Initialise client.csv
                break
            },
            Err(_) => {
                println!("ERR_reading_line");
                continue
            },
        };
    };

    // TODO! Create client.csv file

    println!("Now it's time to select difficulty!:\n\n1. Easy\n    You start off at Keflavik International Airport \
    (BIKF) which is the biggest\n    airport in Iceland. Your initial aircraft will be a Cessna 208B and your\n    \
    initial debt is none, you'll even have 100.000€ in your account!\n"); // TODO! Create more modes

    loop {
        let input = STDIN.read_line(INPUT).expect("ERR_reading_line");
        match input.trim().parse() {
            Ok(num) => {
                let option: u8 = num;
                // TODO! Configure client and fleet
                break
            },
            Err(_) => {
                println!("ERR_reading_line");
                continue
            },
        };
    };
}
