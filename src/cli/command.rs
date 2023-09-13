use std::error::Error;
use regex::Regex;

use crate::server::Server;
use crate::server::servercfg::ServerConfig;

// TODO: Implement the following commands:
//      + getprivateip (returns private IP)
//      + host (owns the server or returns the current server host name)




//=================================================================
//====================   COMMAND FUNCTIONS   ======================
pub fn handle_config(server: &mut Server) -> Result<(), Box<dyn Error>>{
    let regex_mem: &str = "[1-9][0-9]*[MG]";
    let regex_gui: &str = "[YN]";

    println!("\nTo introduce memory data, please refer to units with its initial");
    println!("(That is, 500 MB => 500M, 5 GB => 5G...)");

    println!("\nIntroduce initial memory (default = 512M)");
    let mem_init = read_config_input(regex_mem, "512M").unwrap();
    println!("\nIntroduce max memory (default = 2G)");
    let mem_max = read_config_input(regex_mem, "2G").unwrap();
    println!("\nDo you want the GUI to be visible? (Y/N)");
    let gui_string = read_config_input(regex_gui, "Y").unwrap();

    let gui: bool;
    match gui_string.as_str() {
        "Y" => gui = true,
        _ => gui = false,
    }

    println!("\nBuilding server configuration...");

    let server_config = ServerConfig::new(mem_max,mem_init,gui);

    match server_config {
        Ok(_) => {
            println!("\nServer configured!");
            server.configure(server_config.unwrap());
            Ok(())
        },
        Err(boxdyn) => {
            println!("\nCould not retrieve public IP of the current device.");
            Err(boxdyn)
        },
    }
}

pub fn handle_run(server: &mut Server) {
    if !server.is_config() {
        println!("\nServer has not been configured yet!");
        println!("Starting configuration command...");
        handle_config(server).expect("Error building configuration");
    }
    let result = server.run();

    match result {
        Ok(_) => println!("\nServer initialized successfully!\n"), 
        Err(err) => println!("{}", crate::server::get_error_msg(err).as_str()),
    }
}

pub fn handle_stop(server: &mut Server) {
    println!("Stopping server...");

    let result = server.stop();

    match result {
        Ok(_) => println!("Server stopped successfully!\n"),
        Err(err) => println!("{}", crate::server::get_error_msg(err).as_str()),
    }
}

pub fn handle_state(server: &Server) {
    println!("{}", server.get_state().as_str());
}

pub fn handle_public_ip(server: &Server) {
    if let Some(config) = server.get_config() {
        println!("Public IP address: {}", config.get_public_ip());
    } 
    else {
        println!("Server not configured yet!");
    }
}

pub fn handle_unknown(command: &str) {
    println!("Received {:?}", command);
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
fn read_config_input(regex_input: &str, default: &str) -> Result<String, std::io::Error> {
    let mut input: String = String::from("");
    let re = Regex::new(regex_input).unwrap();

    let mut valid: bool = false;
    let mut mem: &str;
    let mut is_match: bool;
    while !valid {
        super::read_command(&mut input)?;
        mem = input.trim();
        is_match = re.is_match(mem);

        if is_match || mem.is_empty() {
            valid = true; 
            match is_match {
                true => input = String::from(re.captures(mem)
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .as_str()),
                false => input = String::from(default)
            }
        }
        else {
            println!("Value not valid. Please, enter a valid value.");
        }
    }

    Ok(input)
}