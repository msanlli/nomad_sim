fn main() {
    let version_txt: &str = {
        "NomadSim v0.0d\n@msanlli - 2023\n\n--------------------\n\nWelcome \
        to NomadSim!\n\nChoose an option:\n1. New Game\n2. Load Game\n"
    };

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

    println!("{}\n{}", version_txt, tutorial_intro_txt);
}
