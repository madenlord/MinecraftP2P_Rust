pub fn handle_quit() {
    unsafe { super::STOP = true; }
}

pub fn handle_unknown(command: &str) {
    println!("Received {:?}", command);
}