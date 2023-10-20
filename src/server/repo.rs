const SAVE_PATH: &str = "conf/repo.conf";

pub fn download_updates() -> Result<bool, std::io::Error>{
    let mut update_found: bool = false;
    
    // Get latest changes in Git repo
    let update_res = git::fetch()?;

    // If updates in the world data was found
    if !update_res.stdout.is_empty() || !update_res.stderr.is_empty() {
        update_found = true;
        git::pull_no_ff()?;     // Merge update with local data
    }

    Ok(update_found)
}

mod git {
    use std::process::Output;    

    use crate::server::ioutils::terminal;

    // =========== GET AND UPDATE STATE ===========
    pub fn status() -> Result<Output, std::io::Error>{
        execute_git_command("status", vec![])
    }

    pub fn fetch() -> Result<Output, std::io::Error>{
        execute_git_command("fetch", vec![])
    }

    pub fn pull() -> Result<Output, std::io::Error>{
        execute_git_command("pull", vec![])
    }

    pub fn pull_no_ff() -> Result<Output, std::io::Error>{
        execute_git_command("pull", vec!["--no-ff"])
    }



    // =========== PUSH ===========
    pub fn add(files: Vec<&str>) -> Result<Output, std::io::Error>{
        execute_git_command("add", files)
    }


    pub fn commit(message: &str) -> Result<Output, std::io::Error>{
        execute_git_command("commit", vec!["-m", message])
    }

    pub fn push() -> Result<Output, std::io::Error>{
        execute_git_command("push", vec![])
    }



    // =========== SETTERS ===========
    pub fn set_origin(url: &str) -> Result<Output, std::io::Error> {
        execute_git_command("remote", vec!["set-url", "origin", url])
    }

    

    // =========== PRIVATE ===========
    fn execute_git_command<'a>(command: &'a str, mut args: Vec<&'a str>) -> Result<Output, std::io::Error>{ 
        let mut git_args: Vec<&str> = vec![command];
        git_args.append(&mut args);
        terminal::execute_command("git", git_args, "./mojang")
    }
}