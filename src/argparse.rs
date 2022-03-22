use clap::{Arg, ArgMatches, Command, Values};
use regex::Regex;
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

    pub fn from(&self) -> &str {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.value_of("from").unwrap()
    }

    pub fn from_as_regex(&self) -> Regex {
        Regex::from_str(self.from()).expect("Invalid input path")
    }

    pub fn to(&self) -> &str {
        // SAFETY: Argument is required, it cannot be empty.
        self.0.value_of("to").unwrap()
    }

    pub fn files(&self) -> Values {
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
            self.from(),
            self.to(),
            self.files().collect::<Vec<&str>>(),
        )
    }
}
