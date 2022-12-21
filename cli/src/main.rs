mod run;

use clap::Parser;
use run::Run;

#[derive(Parser)]
pub enum SubCommand {
    Run(Run),
}

impl SubCommand {
    pub fn run(&self) {
        match self {
            Self::Run(run) => run.run(),
        }
    }
}

#[derive(Parser)]
#[clap(version)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

pub fn main() {
    let args = Args::parse();

    args.subcommand.run();
}
