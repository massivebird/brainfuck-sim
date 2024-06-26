use crate::computer::Computer;
use crate::inst::Inst;

mod computer;
mod inst;

fn main() {
    let matches = clap::command!()
        .arg(
            clap::Arg::new("src")
                .required(true)
                .value_hint(clap::ValueHint::FilePath)
                .value_name("FILE")
                .value_parser(clap::value_parser!(String))
                .help("Brainfuck source code file path"),
        )
        .get_matches();

    let bf = std::fs::read_to_string(matches.get_one::<String>("src").unwrap()).unwrap();

    let mut computer = Computer::new(30000);

    let instructions = Inst::from_str(&bf);

    computer.execute(&instructions);

    #[cfg(debug_assertions)] // run except in release mode
    dbg!(&computer);
}
