mod repo {
    const SAVE_PATH: &str = "conf/repo.conf";

    mod git {
        use std::process::ExitStatus;    

        use crate::server::ioutils::terminal;

        // =========== GET AND UPDATE STATE ===========
        pub fn status() -> Result<ExitStatus, std::io::Error>{
            execute_git_command("status", vec![])
        }

        pub fn fetch() -> Result<ExitStatus, std::io::Error>{
            execute_git_command("fetch", vec![])
        }

        pub fn pull() -> Result<ExitStatus, std::io::Error>{
            execute_git_command("pull", vec![])
        }



        // =========== PUSH ===========
        pub fn add(files: Vec<&str>) -> Result<ExitStatus, std::io::Error>{
            execute_git_command("add", files)
        }


        pub fn commit(message: &str) -> Result<ExitStatus, std::io::Error>{
            execute_git_command("commit", vec!["-m", message])
        }

        pub fn push() -> Result<ExitStatus, std::io::Error>{
            execute_git_command("push", vec![])
        }



        // =========== SETTERS ===========
        pub fn set_origin(url: &str) -> Result<ExitStatus, std::io::Error> {
            execute_git_command("remote", vec!["set-url", "origin", url])
        }



        fn execute_git_command<'a>(command: &'a str, mut args: Vec<&'a str>) -> Result<ExitStatus, std::io::Error>{ 
            let mut git_args: Vec<&str> = vec![command];
            git_args.append(&mut args);
            terminal::execute_command("git", git_args, ".")
        }
    }
}


