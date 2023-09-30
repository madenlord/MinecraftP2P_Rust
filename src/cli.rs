mod command;

use std::error::Error;
use std::io::{self, Write, Result as IoResult};
use super::server as srvr;

const INIT_MSG: &str = "\n\
Welcome to the Minecraft P2P Server CLI!
Please write down the desired commands after the '>'.
If you need help, please type 'h' or help'";

pub struct Cli {
    server: srvr::Server,
    stop: bool,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            server: srvr::Server::new(),
            stop: false,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}", INIT_MSG);

        let mut command = String::new();

        while !self.stop {
            read_command(&mut command)?;
            self.handle_command(&command);
        }

        Ok(())
    }

    fn handle_command(&mut self, command: &str) {
        match command {
            "config" => command::handle_config(&mut self.server)
                        .expect("Error building configuration."), 
            "run" => command::handle_run(&mut self.server),
            "stop" => command::handle_stop(&mut self.server),
            "state" => command::handle_state(&self.server),
            "getpublicip" => command::handle_public_ip(&self.server),
            "getconfig" => command::handle_get_config(&self.server),
            "quit" => self.stop = true,
            _ => command::handle_unknown(command), 
        };
    }
}

pub fn read_command(command: &mut String) -> IoResult<()> {
    prepare_command_buff(command);
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