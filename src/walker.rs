use crate::argparse::Args;
use std::io::{Result, stdout};
use std::fs;
use regex::Regex;
use termion::{raw::IntoRawMode, input::TermRead, event::Key};

#[derive(Debug)]
pub struct Walker<'a> {
    args: &'a Args,
    matching_files: Vec<&'a str>,
    targets: Vec<String>,
}

impl<'a> Walker<'a> {
    pub fn new(args: &'a Args) -> Self {
        let match_regex = args.from_as_regex();
        let to = args.to();
        let matching_files: Vec<_> = args.files().filter(|f| match_regex.is_match(f)).collect();
        let targets = matching_files
            .iter()
            .map(|f| match_regex.replace_all(f, to).to_string())
            .collect();

        Self {
            args,
            matching_files,
            targets,
        }
    }

    fn print_all_filenames(&self) {
        for (file, target) in self.matching_files.iter().zip(self.targets.iter()) {
            println!("{} -> {}\r", file, target);
        }
    }

    fn ask_confirm(&mut self) -> Result<bool> {
        let _stdout = stdout().into_raw_mode()?;
        let stdin = termion::async_stdin();

        if self.matching_files.is_empty() {
            println!("No matches\r");
            return Ok(false);
        }

        println!("The following files will be renamed:\r\n");
        self.print_all_filenames();
        println!("\r\nIs that ok? (y/n)\r");

        let mut keys = stdin.keys();

        loop {
            let input = keys.next();
            match input {
                Some(Ok(key)) => match key {
                    // Handle C-c like saying 'no'
                    Key::Char('n') | Key::Ctrl('c') => break Ok(false),
                    Key::Char('y') => break Ok(true),
                    _ => (),
                }
                Some(Err(e)) => break Err(e),
                _ => (),
            };
        }
    }

    fn rename_files(&mut self) -> Result<()> {
        for (from,to) in self.matching_files.iter().zip(self.targets.iter()) {
            fs::rename(from, to)?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if self.args.should_say_yes() {
            self.rename_files()?;
            println!("The following files have been renamed:");
            self.print_all_filenames();
        } else if let Ok(true) = self.ask_confirm() {
            self.rename_files()?;
            println!("Done!");
        }

        Ok(())
    }
}
