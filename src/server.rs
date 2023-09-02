use hyper::client::Client;
use hyper::body;
use hyper::Uri;

use tokio::runtime::Runtime;

use std::error::Error;
use std::str;

use bytes::Bytes;


pub struct Server {
    config: Option<ServerConfig>,
    state: State
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

impl Server {
    pub fn new() -> Server {
        Server { 
            config: None,
            state: State::STOPPED 
        }
    }

    pub fn configure(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn get_config(&self) -> &Option<ServerConfig> {
        &self.config
    }
}

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