use crate::argparse::Args;
use colored::Colorize;
use std::fs;
use std::io::{stdout, Error, ErrorKind, Result, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Debug)]
pub struct Walker<'a> {
    args: &'a Args,
    matching_files: Vec<&'a str>,
    targets: Vec<String>,
}

impl<'a> Walker<'a> {
    pub fn new(args: &'a Args) -> Result<Self> {
        if let Ok(match_regex) = args.from_as_regex() {
            let to = args.to();
            let matching_files: Vec<_> = args.files().filter(|f| match_regex.is_match(f)).collect();
            let targets = matching_files
                .iter()
                .map(|f| match_regex.replace_all(f, to).to_string())
                .collect();

            Ok(Self {
                args,
                matching_files,
                targets,
            })
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid regular expession",
            ))
        }
    }

    fn print_all_filenames(&self) -> Result<()> {
        let mut stdout = stdout().into_raw_mode()?;

        // SAFETY: We've already deternmined that it's a valid expression in the constructor.
        let re = self.args.from_as_regex().unwrap();
        let to = self.args.to();

        let print_coloured = |text: &str, repl: &str, colour: &str| {
            let mut split = re.split(text).peekable();
            while let Some(part) = split.next() {
                print!("{}", part);

                // Don't print the replacement on the last match. Otherwise it will
                // append all strings with it.
                if split.peek().is_some() {
                    print!("{}", repl.color(colour));
                }
            }
        };

        for file in self.matching_files.iter() {
            let from = re.find(file).unwrap().as_str();
            let to = re.replace(from, to);

            print_coloured(file, &from, "yellow");
            print!(" -> ");
            print_coloured(file, &to, "blue");
            print!("\r\n");
        }

        stdout.flush()?;

        Ok(())
    }

    fn ask_confirm(&mut self) -> Result<bool> {
        let mut stdout = stdout().into_raw_mode()?;
        let stdin = termion::async_stdin();

        println!("The following files will be renamed:\r\n");
        self.print_all_filenames()?;
        println!("\r\nIs that ok? (y/n)\r");

        stdout.flush()?;

        let mut keys = stdin.keys();

        loop {
            let input = keys.next();
            match input {
                Some(Ok(key)) => match key {
                    // Handle C-c like saying 'no'
                    Key::Char('n') | Key::Ctrl('c') => break Ok(false),
                    Key::Char('y') => break Ok(true),
                    _ => (),
                },
                Some(Err(e)) => break Err(e),
                _ => (),
            };
        }
    }

    fn rename_files(&mut self) -> Result<()> {
        for (from, to) in self.matching_files.iter().zip(self.targets.iter()) {
            fs::rename(from, to)?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if self.matching_files.is_empty() {
            println!("No matches.");
        } else if self.args.should_say_yes() {
            self.rename_files()?;
            println!("The following files have been renamed:");
            self.print_all_filenames()?;
        } else if let Ok(true) = self.ask_confirm() {
            self.rename_files()?;
            println!("Done!");
        }

        Ok(())
    }
}
