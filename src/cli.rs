mod command;

use std::error::Error;
use std::io::{self, Write, Result as IoResult};

const INIT_MSG: &str = "\n\
Welcome to the Minecraft P2P Server CLI!
Please write down the desired commands after the '>'.
If you need help, please type 'h' or help'";

static mut STOP: bool = false;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("{}", INIT_MSG);

    let mut command = String::new();
    unsafe {
        while !STOP {
            prepare_command_buff(&mut command);
            read_command(&mut command)?;
            handle_command(&command);
        }
    }

    Ok(())
}

fn read_command(command: &mut String) -> IoResult<()> {
    print!("> ");
    io::stdout().flush()?;
    io::stdin()
        .read_line(command)?;

    *command = command.trim().to_string();

    Ok(())
}

fn prepare_command_buff(command: &mut String) {
    if !command.is_empty() { command.clear(); }
}

fn handle_command(command: &str) {
    match command {
        "quit" => command::handle_quit(),
        _ => command::handle_unknown(command), 
    };
}