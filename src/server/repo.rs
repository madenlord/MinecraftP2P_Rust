
// TODO: Implement the following functions:
//      + host: to host, the user will need to verify
//              the server is not already hosted. That
//              means, pulling the latest changes and
//              checking the content of the .lock file

pub mod git {
    use std::process::ExitStatus;    

    use crate::server::ioutils::terminal;

    // =========== GET AND UPDATE STATE ===========
    pub fn status() -> Result<ExitStatus, std::io::Error>{
        execute_git_command("status", vec![])
    }

    pub fn fetch() {

    }

    pub fn pull() {

    }

    

    // =========== PUSH ===========
    pub fn add() {

    }


    pub fn commit() {

    }

    pub fn push() {

    }

    fn execute_git_command<'a>(command: &'a str, mut args: Vec<&'a str>) -> Result<ExitStatus, std::io::Error>{ 
        let mut git_args: Vec<&str> = vec![command];
        git_args.append(&mut args);
        terminal::execute_command("git", git_args, ".")
    }
}