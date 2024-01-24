#![deny(
    clippy::all,
    clippy::pedantic,
)]
#![warn(clippy::nursery)]

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
        server::launch(&addr)?;
    } else if let Some(args) = args.input {
        match client::client(args) {
            Ok(()) => (),
            Err(e) => println!("FAILED to transmit with error: {e}"),
        }
    } else {
        println!("Invalid arguments provided.");
    }

    Ok(())
}
