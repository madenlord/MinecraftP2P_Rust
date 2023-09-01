use crate::server as srvr;

// TODO: Implement the following commands:
//      + getip (returns your public IP)
//          - For now, handle_ip() will make a query to know our public IP.
//            Nevertheless, the idea is that ip returns the IP stored in the
//            server struct, that has already launched the get_public_ip() fn.
//      + port (returns the TCP port binded to the Minecraft server)
//      + host (owns the server or returns the current server host name)
//      + run (starts the server and asks for config if not specified
//              by args)
//      + state (returns the state of the server)
//      + stop (stops the server if running)
//      + config (builds the Server obj and asks for server config)

pub fn handle_ip(server: &srvr::Server) {
    if let Some(config) = server.get_config() {
        println!("Public IP address: {}", config.get_public_ip());
    } 
    else {
        println!("Server not configured yet!");
    }
}

//pub fn handle_config(mut server: &srvr::Server) {
//}

pub fn handle_unknown(command: &str) {
    println!("Received {:?}", command);
}