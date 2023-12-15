#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity
)]
#![warn(clippy::pedantic, clippy::perf, clippy::style, clippy::cargo)]
#![allow(clippy::cargo_common_metadata)]

mod args;

use anyhow::Result;
use enigo::{Enigo, KeyboardControllable};
use std::{env::args, fs::File, io::Read};
use thiserror::Error;

#[derive(Debug, Error)]
enum MainError {
    #[error("ERROR: No file provided.")]
    Arg,
}

fn main() -> Result<()> {
    let mut args = args();
    let mut input = String::new();
    File::open(args.nth(1).ok_or(MainError::Arg)?)?.read_to_string(&mut input)?;

    Enigo::new().key_sequence(&input);

    Ok(())
}
