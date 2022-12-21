use std::{io, path::PathBuf};

use athame::{
    error::{Error, ErrorFormatter},
    sources::Sources,
    span::Span,
};
use clap::Parser;

#[derive(Parser)]
pub struct Run {
    /// The path to the file to run.
    #[clap(default_value = "main.ath")]
    pub path: PathBuf,
}

impl Run {
    pub fn run(&self) {
        let mut sources = Sources::new();
        let id = sources.open(&self.path);

        let mut stdout = io::stdout();
        let mut formatter = ErrorFormatter::new(&sources, &mut stdout);
        formatter.format_error(&error).unwrap();
    }
}
