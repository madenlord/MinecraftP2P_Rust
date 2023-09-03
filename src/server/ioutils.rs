pub mod terminal {
    use std::process::{Command, Child, Stdio};
    use std::ffi::OsStr;

    pub fn execute_command<I, S>(
        program: &str,
        args: I,
        dir: &str,
        stdout_filepath: &str,
    ) -> Result<Child, std::io::Error> 
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    { 
        let mut process: Command = Command::new(program);
        process
        .current_dir(dir)
        .stdout(Stdio::from(
            super::file::open_file(stdout_filepath)
            .expect("Failed opening stdout file")
        )).args(args);

        process.spawn()
    }
}

pub mod internet {
    use std::error::Error;
    use std::str;

    use hyper::client::Client;
    use hyper::body;
    use hyper::Uri;

    use tokio::runtime::Runtime;

    use bytes::Bytes;

    pub fn get_req(uri: &str) -> Result<String, Box<dyn Error>> {
        let rt = Runtime::new().unwrap();
        let client = Client::new();

        let body_content = rt.block_on(async {
            let uri = uri.parse::<Uri>().unwrap();
            let resp = client.get(uri).await?;
            let resp_body = body::to_bytes(resp.into_body()).await?;

            Ok::<Bytes, Box<dyn Error>>(resp_body)
        })?;

        let body_content = String::from(str::from_utf8(&body_content).unwrap());

        Ok(body_content)
    }
}

pub mod file {
    use std::fs::{File, OpenOptions};

    pub fn open_file(filepath: &str) -> Result<File, std::io::Error> {
        OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath)
    }
}