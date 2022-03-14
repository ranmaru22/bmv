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
                    Arg::new("include-hidden")
                        .short('i')
                        .help("Includen hidden files in wildcards")
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
                .get_matches()
        )
    }

    pub fn should_say_yes(&self) -> bool {
        self.0.is_present("yes")
    }

    pub fn should_include_hidden(&self) -> bool {
        self.0.is_present("include-hidden")
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
}

// For Debug purposes only
impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "from: {}\nto: {}\n\nyes? {}",
            self.from(),
            self.to(),
            self.should_say_yes(),
        )
    }
}
