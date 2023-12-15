mod args;
mod client;
mod server;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = args::Inp::parse();

    if args.server {
        server::launch_server()?;
    } else if let Some(args) = args.input {
        client::launch_client(args);
    } else {
        println!("Invalid arguments provided.")
    }

    Ok(())
}
