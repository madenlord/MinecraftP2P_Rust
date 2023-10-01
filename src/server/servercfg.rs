use std::error::Error;
use std::str;
use serde::{Serialize, Deserialize};

use super::ioutils;

const CONFIG_FILEPATH: &str = "conf/server.conf";

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    ip: String,
    mem_max: String,
    mem_init: String,
    gui: bool,
}

// TODO: store configuration in a config file
// Maybe separate Server and ServerConfig in different
// files / modules?
impl ServerConfig {
    pub fn new(
        mem_max: String, 
        mem_init: String, gui: bool
    ) ->  Result<ServerConfig, Box<dyn Error>>{
        let server_config = ServerConfig {
            ip: Self::find_public_ip()?,
            mem_max: mem_max,
            mem_init: mem_init,
            gui: gui,
        };

        ioutils::file::write(CONFIG_FILEPATH, server_config.to_json()?.as_str())?;

        Ok(server_config)
    }

    pub fn load_config() -> Result<ServerConfig, Box<dyn Error>>{
        let json_config = ioutils::file::read(CONFIG_FILEPATH)?;
        let server_config: ServerConfig = serde_json::from_str(json_config.as_str())?;

        Ok(server_config)
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

    pub fn to_string(&self) -> String {
       format!(
        "Public IP = {}\n\
        Max memory = {}\n\
        Initial memory = {}\n\
        GUI = {}\n\
        ", self.ip, self.mem_max, self.mem_init, self.gui
        )
    }

    fn find_public_ip() -> Result<String, Box<dyn Error>> {
        ioutils::internet::get_req("http://api.ipify.org")
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let self_json = serde_json::to_string(&self)?;
        
        Ok(self_json)
    } 
}