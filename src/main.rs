#[macro_use]
extern crate clap;

mod argparse;
use argparse::Args;

mod walker;
use walker::Walker;

use std::io::Result;

fn main() -> Result<()> {
    let args = Args::new();
    let mut walker = Walker::new(&args);

    walker.run()
}
