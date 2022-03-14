use crate::argparse::Args;
use ignore::{WalkBuilder, WalkState};
use globset::Glob;
use std::{thread, sync::mpsc, io::Result};

#[derive(Debug)]
pub struct Walker<'a> {
    args: &'a Args,
}

impl<'a> Walker<'a> {
    pub fn new(args: &'a Args) -> Self {
        Self { args }
    }

    pub fn walk(&self) {
        let root_path = &self.args.from()
            .split("*")
            .next()
            .unwrap_or(&self.args.from());

        let walker = WalkBuilder::new(root_path)
            .hidden(self.args.should_include_hidden())
            .build_parallel();

        let (tx, rx) = mpsc::channel::<String>();

        let collector = thread::spawn(move || -> Result<Vec<String>> {
            let mut files = Vec::new();
            for file in rx.iter() {
                println!("{}", file);
                files.push(file);
            }

            Ok(files)
        });

        walker.run(|| {
            let tx = tx.clone();

            Box::new(move |filename| {
                if let Ok(fp) = filename {
                    if !fp.path().is_file() {
                        WalkState::Continue
                    } else {
                        let file = fp.path().display().to_string();
                        let regex = self.args.from_as_regex().clone();

                        if regex.is_match(&file) {
                            match tx.send(file) {
                                Ok(_) => WalkState::Continue,
                                Err(_) => WalkState::Quit,
                            }
                        } else { WalkState::Continue }
                    }
                } else { WalkState::Continue }
            })
        });

        drop(tx);
    }
}
