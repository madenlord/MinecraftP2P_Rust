use std::error::Error;
use std::str;

use super::ioutils;

const CONF_FILEPATH: &str = "conf/server.conf";

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

    pub fn to_string(&self) -> String {
        let string = format!("Public IP = {}\n\
        Max memory = {}\n\
        Initial memory = {}\n\
        GUI = {}\
        ", self.ip, self.mem_max, self.mem_init, self.gui);

        string
    }

    fn find_public_ip() -> Result<String, Box<dyn Error>> {
        ioutils::internet::get_req("http://api.ipify.org")
    }

    
}