use std::{
    fs::read_to_string,
    path::PathBuf,
    os::unix::net::UnixStream, io::Write,
};
use anyhow::Result;

pub fn launch_client(path: PathBuf) -> Result<()> {
    let data = read_to_string(path)?;

    // Send data through socket.
    UnixStream::connect("/tmp/remacro-socket")?.write_all(data.as_bytes())?;
    Ok(())
}
