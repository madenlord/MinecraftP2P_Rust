use crate::server;

// TODO: Implement the following commands:
//      + ip (returns your public IP)
//          - For now, handle_ip() will make a query to know our public IP.
//            Nevertheless, the idea is that ip returns the IP stored in the
//            server struct, that has already launched the get_public_ip() fn.
//      + port (returns the TCP port binded to the Minecraft server)
//      + host (owns the server or returns the current server host name)
//      + run (starts the server and asks for config if not specified
//              by args)
//      + state (returns the state of the server)
//      + stop (stops the server if running)

pub fn handle_ip() {
    let public_ip = server::get_public_ip();
    println!("Public IP address: {}", public_ip.unwrap());
}

pub fn handle_quit() {
    unsafe { super::STOP = true; }
}

pub fn handle_unknown(command: &str) {
    println!("Received {:?}", command);
}