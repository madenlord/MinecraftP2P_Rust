use hyper::client::Client;
use hyper::body;
use hyper::Uri;

use tokio::runtime::Runtime;

use std::error::Error;
use std::str;

use bytes::Bytes;

enum State {
    HOSTED(String),
    RUNNING,
    STOPPED,
}

pub struct Server {
    ip: String,
    // port: u16 = DEFAULT_PORT = 25565;
    mem_max: u64,
    mem_init: u64,
    gui: bool,
    state: State,
}

impl Server {
    pub fn build(
        mem_max: u64, mem_init: u64,
        gui: bool
    ) -> Server {
        Server {
            ip: String::from(""),
            mem_max: mem_max,
            mem_init: mem_init,
            gui: gui,
            state: State::RUNNING,
        }
    }
}

pub fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let rt = Runtime::new().unwrap();
    let client = Client::new();

    let public_ip = rt.block_on(async {
        let resp = client.get(Uri::from_static("http://api.ipify.org")).await?;
        let resp_body = body::to_bytes(resp.into_body()).await?;

        Ok::<Bytes, Box<dyn Error>>(resp_body)
    })?;

    let public_ip = String::from(str::from_utf8(&public_ip).unwrap());
    
    Ok(public_ip)
}