use std::error::Error;
use mncsrvr::cli;

fn main() -> Result <(), Box<dyn Error>> {
    let mut cli = cli::Cli::new();    
    cli.run()
}
