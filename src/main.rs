mod args;
mod client;
mod server;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = args::Inp::parse();
    let addr = PathBuf::from("/tmp/remacro-socket");

    if args.server {
        server::launch_server(addr)?;
    } else if let Some(args) = args.input {
        match client::launch_client(args) {
            Ok(_) => (),
            Err(e) => println!("FAILED to transmit with error: {e}"),

        }
    } else {
        println!("Invalid arguments provided.")
    }

    Ok(())
}
