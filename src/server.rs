pub mod servercfg;
mod ioutils;

use std::process::Child;

use servercfg::ServerConfig;




//=================================================================
//======================   CONSTANTS & OOP   ======================
//=================================================================
const LOG_PATH: &str = "logs/server_jar.log";

pub struct Server {
    config: Option<ServerConfig>,
    state: State,
    process: Option<Child>,
}

enum State {
    HOSTED(String),
    RUNNING(u32),
    STOPPED,
}

pub enum ServerError {
    NO_CONFIG,
    RUNNING(u32),
    HOSTED(String),
    JAR_FAIL,
}

impl Server {
    pub fn new() -> Server {
        Server { 
            config: None,
            state: State::STOPPED,
            process: None, 
        }
    }

    pub fn configure(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn get_config(&self) -> &Option<ServerConfig> {
        &self.config
    }

    pub fn is_config(&self) -> bool {
        if let None = self.config { false }
        else { true }
    }

    pub fn run(&mut self) -> Result<(), ServerError> {
        if let None = self.config {
            return Err(ServerError::NO_CONFIG)
        }
        match &(self.state) {
            State::RUNNING(pid) => Err(ServerError::RUNNING(*pid)),
            State::HOSTED(host) => Err(ServerError::HOSTED(String::from(host))),
            State::STOPPED => { 
                let pid = self.execute_server_jar()?;
                self.state = State::RUNNING(pid);
                Ok(())
            }
        }
    }

    fn execute_server_jar(&mut self) -> Result<u32, ServerError> {
        if let Some(config) = &(self.config) {
            let program = "java";
            let dir = "mojang/";
            let mut args = [
                format!("-Xmx{}", config.get_mem_max()),
                format!("-Xms{}", config.get_mem_init()),
                String::from("-jar"),
                String::from("server.jar"),
                String::from("")
            ];

            if !config.get_gui() { args[4] = String::from("--nogui"); }

            let command_result = ioutils::terminal::execute_command(
                program, args, dir, LOG_PATH
            );

            match command_result {
                Ok(child) => {
                    let pid = child.id();
                    self.process = Some(child);
                    Ok(pid)
                }
                Err(_) => Err(ServerError::JAR_FAIL)
            } 
        }
        else {
            Err(ServerError::NO_CONFIG)
        }
    }
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
pub fn get_error_msg(err: ServerError) -> String {
    match err {
        ServerError::NO_CONFIG => String::from("Server has not been configured yet!"),
        ServerError::RUNNING(pid) => String::from(format!("Server is already running! (PID = {})\n", pid)),
        ServerError::HOSTED(host) => String::from(format!("Server is being hosted by {}", host.as_str())),
        ServerError::JAR_FAIL => String::from("mojang/server.jar execution failed!"),
    }
}