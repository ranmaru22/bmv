#[macro_use]
extern crate clap;

mod argparse;
use argparse::Args;

mod walker;
use walker::Walker;

use std::{fs, io::Result};

fn main() -> Result<()> {
    let args = Args::new();
    let walker = Walker::new(&args);

    //println!("{}\n{:?}", args, walker);

    walker.walk();

    Ok(())
}
