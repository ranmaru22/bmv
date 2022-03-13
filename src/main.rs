#[macro_use]
extern crate clap;

mod argparse;
use argparse::Args;


fn main() {
    let args = Args::new();
    let from = args.get_from();
    let to = args.get_to();
    let debug_input = args.get_input()[0];

    if let Ok(re) = args.get_from_as_regex() {
        let out = re.replace_all(debug_input, to);
        println!("{}", out);
    }
}
