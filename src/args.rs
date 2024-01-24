use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Inp {
    #[arg(short, long)]
    pub server: bool,

    #[arg(short, long)]
    pub input: Option<PathBuf>,

    #[arg(short, long)]
    pub message: Option<String>,
}
