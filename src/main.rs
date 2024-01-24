#![deny(
    clippy::all,
    clippy::pedantic,
)]
#![warn(clippy::nursery)]

mod args;
mod client;
mod server;

use std::{
    fs::read_to_string,
    path::PathBuf,
};

use anyhow::{Result, Context};
use clap::Parser;

fn main() -> Result<()> {
    let args = args::Inp::parse();
    let addr = PathBuf::from("/tmp/remacro-socket");

    if args.server {
        server::launch(&addr)?;
    } else if let Some(input) = args.input {
        let data = read_to_string(input).context("Input file does not exist.")?;

        match client::client(&data, &addr) {
            Ok(()) => (),
            Err(e) => println!("FAILED to transmit with error: {e}"),
        }
    } else if let Some(message) = args.message {
        match client::client(&message, &addr) {
            Ok(()) => (),
            Err(e) => println!("FAILED to transmit with error: {e}"),
        }
    } else {
        println!("Invalid arguments provided.");
    }

    Ok(())
}
