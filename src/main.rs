use std::error::Error;
use mncsrvr::cli;

fn main() -> Result <(), Box<dyn Error>> {
    cli::run()
}
