use std::{fmt, str::FromStr};
use clap::{Arg, Command, ArgMatches};
use regex::{Regex, Error};

#[derive(Debug)]
pub struct Args(ArgMatches);

impl Args {
    pub fn new() -> Self {
        Self(
            Command::new("brep")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .help("Don't ask for confirmation")
                )
                .arg(
                    Arg::new("dryrun")
                        .short('d')
                        .long("dryrun")
                        .help("Dry run, don't actually rename anything")
                )
                .arg(
                    Arg::new("from")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("to")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("input")
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true)
                )
                .get_matches()
        )
    }

    pub fn is_yes(&self) -> bool {
        self.0.is_present("yes")
    }

    pub fn is_dryrun(&self) -> bool {
        self.0.is_present("dryrun")
    }

    pub fn get_from(&self) -> &str {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.value_of("from").unwrap()
    }

    pub fn get_from_as_regex(&self) -> Result<Regex, Error> {
        Regex::from_str(self.get_from())
    }

    pub fn get_to(&self) -> &str {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.value_of("to").unwrap()
    }

    pub fn get_input(&self) -> Vec<&str> {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.values_of("input").unwrap().collect()
    }
}

// For Debug purposes only
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "from: {}\nto: {}\ninput: {:?}\nyes? {}",
            self.get_from(),
            self.get_to(),
            self.get_input(),
            self.is_yes(),
        )
    }
}
