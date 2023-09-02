use hyper::client::Client;
use hyper::body;
use hyper::Uri;

use tokio::runtime::Runtime;

use std::process::{Command, Child, Stdio};
use std::error::Error;
use std::str;
use std::fs::OpenOptions;

use bytes::Bytes;




//=================================================================
//======================   CONSTANTS & OOP   ======================
//=================================================================
const LOG_PATH: &str = "logs/server_jar.log";

pub struct Server {
    config: Option<ServerConfig>,
    state: State,
    process: Option<Child>,
}

pub struct ServerConfig {
    ip: String,
    // port: u16 = DEFAULT_PORT = 25565;
    mem_max: String,
    mem_init: String,
    gui: bool,
}

enum State {
    HOSTED(String),
    RUNNING,
    STOPPED,
}

pub enum ServerError {
    NO_CONFIG,
    RUNNING,
    HOSTED(String),
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
            State::RUNNING => Err(ServerError::RUNNING),
            State::HOSTED(host) => Err(ServerError::HOSTED(String::from(host))),
            State::STOPPED => { 
                self.execute_server_jar()
                .expect("Could not execute the server binary (server/server.jar)"); 
                self.state = State::RUNNING;
                Ok(())
            }
        }
    }

    // TODO: implement crate or module that handles all the "std::process::Command"
    // stuff, only needing to know the program to be executed, the directory, 
    // the output and the arguments.
    fn execute_server_jar(&mut self) -> Result<(), std::io::Error> {
        if let Some(config) = &(self.config) {
            let mut cmd_command = Command::new("java");
            
            // Stdio::from does not allow mutable references. That is,
            // it is impossible to pass something such as "self.log_file"
            cmd_command
            .current_dir("server/")
            .stdout(Stdio::from(
                OpenOptions::new().write(true).create(true)
                .open(LOG_PATH).expect("Failed opening log file.")     
            )).args([
                    format!("-Xmx{}", config.get_mem_max()),
                    format!("-Xms{}", config.get_mem_init()),
                    String::from("-jar"),
                    String::from("server.jar")
            ]);

            if !config.get_gui() { cmd_command.arg("--nogui"); }

            self.process = Some(cmd_command.spawn()?);
        }
        
        Ok(())
    }
}


// TODO: store configuration in a config file
impl ServerConfig {
    pub fn new(
        mem_max: String, 
        mem_init: String, gui: bool
    ) ->  Result<ServerConfig, Box<dyn Error>>{
        Ok(ServerConfig {
            ip: Self::find_public_ip()?,
            mem_max: mem_max,
            mem_init: mem_init,
            gui: gui,
        })
    }

    pub fn get_public_ip(&self) -> &str {
        self.ip.as_str()
    }

    pub fn get_mem_max(&self) -> &str {
        self.mem_max.as_str()
    }

    pub fn get_mem_init(&self) -> &str {
        self.mem_init.as_str()
    }

    pub fn get_gui(&self) -> &bool {
        &(self.gui)
    }

    fn find_public_ip() -> Result<String, Box<dyn Error>> {
        let rt = Runtime::new().unwrap();
        let client = Client::new();

        let public_ip = rt.block_on(async {
            let resp = client.get(Uri::from_static("http://api.ipify.org")).await?;
            let resp_body = body::to_bytes(resp.into_body()).await?;

            Ok::<Bytes, Box<dyn Error>>(resp_body)
        })?;

        let public_ip = String::from(str::from_utf8(&public_ip).unwrap());

        Ok(public_ip)
    }
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
pub fn get_error_msg(err: ServerError) -> String {
    match err {
        ServerError::NO_CONFIG => String::from("Server has not been configured yet!"),
        ServerError::RUNNING => String::from("Server is already running on your device!\n"),
        ServerError::HOSTED(host) => String::from(format!("Server is being hosted by {}", host.as_str()))
    }
}