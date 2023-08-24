use std::error::Error;

fn main() -> Result <(), Box<dyn Error>> {
    mncsrvr::run()
}
