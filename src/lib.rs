use std::error::Error;
use std::io::{self, Write};

const INIT_MSG: &str = "\n\
Welcome to the Minecraft P2P Server CLI!
Please write down the desired commands after the '>'.
If you need help, please type 'h' or help'";

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("{}", INIT_MSG);
    let mut command = String::new();

    while command != "quit" {
        prepare_command_buff(&mut command);
        read_command(&mut command)?;
        handle_command(&command);
    }

    Ok(())
}

fn read_command(command: &mut String) -> io::Result<()> {
    print!("> ");
    io::stdout().flush()?;
    io::stdin()
        .read_line(command)?;

    *command = command.trim().to_string();

    Ok(())
}

fn prepare_command_buff(command: &mut String) {
    if !command.is_empty() {command.clear(); }
}

fn handle_command(command: &String) {
    println!("Received {:?}", command);
}