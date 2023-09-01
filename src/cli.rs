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
            Self::prepare_command_buff(&mut command);
            Self::read_command(&mut command)?;
            self.handle_command(&command);
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

    fn handle_command(&mut self, command: &str) {
        match command {
            "getip" => command::handle_ip(&self.server),
//            "config" => command::handle_config(&mut self.server),
            "quit" => self.stop = true,
            _ => command::handle_unknown(command), 
        };
    }
}
