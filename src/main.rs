#[macro_use]
extern crate clap;

mod argparse;
use argparse::Args;

mod walker;
use walker::Walker;

use colored::Colorize;
use std::io::{Error, ErrorKind, Result};

fn handle_error(err: &Error) -> Result<()> {
    let msg = match err.kind() {
        ErrorKind::InvalidInput => "Invalid regular expression.",
        _ => "Oops, something went wrong.",
    };

    println!("{} {}", "error:".red().bold(), msg);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::new();
    let walker = Walker::new(&args);

    match walker {
        Ok(mut w) => w.run(),
        Err(ref err) => handle_error(err),
    }
}
