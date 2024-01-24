use std::{
    path::Path,
    os::unix::net::UnixStream,
    io::Write,
};
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
enum ClientError {
    #[error("Socket does not exist! Is the server running?")]
    Socket,
}

pub fn client(data: &str, path: &Path) -> Result<()> {

    if !path.exists() {
        return Err(ClientError::Socket.into());
    };

    // Send data through socket.
    UnixStream::connect(path)?.write_all(data.as_bytes())?;
    Ok(())
}
