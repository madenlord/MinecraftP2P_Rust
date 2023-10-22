pub mod servercfg;
mod repo;
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
    NOT_FOUND,
    IO_ERROR(String),
    REPO_FAIL
}

impl Server {
    pub fn new() -> Server {
        let server_config: Option<ServerConfig>;

        match ServerConfig::load_config() {
            Ok(cfg) => server_config = Some(cfg),
            Err(_) => server_config = None,
        };

        Server { 
            config: server_config,
            state: State::STOPPED,
            process: None, 
        }
    }

    // =========== SETTERS AND GETTERS ===========
    pub fn get_config(&self) -> &Option<ServerConfig> {
        &self.config
    }

    pub fn set_configuration(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn get_state(&self) -> String {
        match &self.state {
            State::HOSTED(host) => format!("Hosted by {}\n", host),
            State::RUNNING(pid) => format!("Server running (PID = {})\n", pid),
            State::STOPPED => String::from("Server is not running.\n"), 
        }
    }

    pub fn get_host(&self) -> &str {
        match &self.state {
            State::RUNNING(_) => self.config.as_ref().unwrap().get_username(),
            State::HOSTED(host) => {
                let host = host.as_str();
                host
            }
            _ => ""
        }
    }

    // =========== FUNCTIONALITY ===========
    pub fn is_configured(&self) -> bool {
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
                let hosting = self.host()?;

                if hosting {
                    let pid = self.execute_server_jar()?;
                    self.state = State::RUNNING(pid);
                    Ok(())
                }
                else {
                    Err(ServerError::HOSTED(String::from(self.get_host())))
                }
            }
        }
    }

    // TODO: REPAIR BUG!! When executing the function,
    // the "kill()" is not executed. Instead, the next commands
    // of this CLI are passed to the stdin of the Minecraft Server .jar .
    //
    // When using "run", then "state" and finally "stop" command, 
    // the function works... Why? 
    pub fn stop(&mut self) -> Result<(), ServerError> {
        match &mut self.process {
            Some(ref mut child) => {
                child.kill().expect("Could not stop Child process (Minecraft Server).");

                self.state = State::STOPPED;
                Ok(())
            },
            None => {
                Err(ServerError::NOT_FOUND)
            }
        }
    }

    // =========== PRIVATE FUNCTIONS ===========
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

            let command_result = ioutils::terminal::spawn_process(
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

    fn host(&mut self) -> Result<bool, ServerError> {
        let mut hosting: bool = false;

        let update_found = match repo::download_updates() {
            Ok(found) => found,
            Err(_) => return Err(ServerError::REPO_FAIL)
        };

        let mut current_host: String = String::new();
        if update_found {
            current_host = match repo::get_current_host() {
                Ok(username) => username,
                Err(_) => return Err(ServerError::IO_ERROR(repo::get_hostfile_path()))
            };

        }  

        if current_host.is_empty() {
            let user = self.config.as_ref().unwrap().get_username();
            match repo::update_host(user) {
                Err(_) => return Err(ServerError::REPO_FAIL),
                _ => ()
            }
            hosting = true;
        }
        else {
            self.state = State::HOSTED(current_host);
        }

        Ok(hosting)        
    }
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
pub fn get_error_msg(err: ServerError) -> String {
    match err {
        ServerError::NO_CONFIG => String::from("Server has not been configured yet!\n"),
        ServerError::RUNNING(pid) => format!("Server is already running! (PID = {})\n", pid),
        ServerError::HOSTED(host) => format!("Server is being hosted by {}\n", host.as_str()),
        ServerError::JAR_FAIL => String::from("mojang/server.jar execution failed!\n"),
        ServerError::NOT_FOUND => String::from("Server instance could not be found!\n"),
        ServerError::IO_ERROR(file) => format!("I/O ERROR: could not read '{}'!\n", file),
        ServerError::REPO_FAIL => String::from("Could not reach the world's data repository!\n"),
    }
}