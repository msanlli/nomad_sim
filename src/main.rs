use std::io;

fn main() {
    let mut option: String = String::new();

    let version_txt: &str = {
        "NomadSim v0.0d\n@msanlli - 2023\n\n--------------------\n\nWelcome \
        to NomadSim!\n\nChoose an option:\n1. New Game\n2. Load Game\n"
    };

    println!("{}", version_txt);

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    if option == "1" {
        let tutorial_intro_txt: &str = {
            "You accomplished your dreams and worked as a pilot for a major \
            airline, but after\na couple years you realised that the comercial \
            aviation wasn't for you, you\nneeded an adventure.\n\nYou decided to \
            quit your job and start a company froms scratch, you started to\npull \
            some strings and one friend of yours told you about a logistic \
            shortage in\nIceland because of the difficult terrain and the lack of \
            roads,so you decided to\nstart a company that would help the locals \
            to transport goods quick and safely."
        };

        println!("{}", tutorial_intro_txt)
    } else if option == "2" {
        println!("ERR_1: not implemented")
    }

    
}
