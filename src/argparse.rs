use clap::{Arg, ArgMatches, Command, Values};
use regex::{Error, Regex};
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct Args(ArgMatches);

impl Args {
    pub fn new() -> Self {
        Self(
            Command::new("bmv")
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .help("Don't ask for confirmation"),
                )
                .arg(Arg::new("from").required(true).takes_value(true))
                .arg(Arg::new("to").required(true).takes_value(true))
                .arg(
                    Arg::new("files")
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                )
                .get_matches(),
        )
    }

    pub fn should_say_yes(&self) -> bool {
        self.0.is_present("yes")
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

    pub fn get_files(&self) -> Values {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.values_of("files").unwrap()
    }
}

// For Debug purposes only
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "from: {}\nto: {}\n\nfiles {:?}",
            self.get_from(),
            self.get_to(),
            self.get_files().collect::<Vec<&str>>(),
        )
    }
}
