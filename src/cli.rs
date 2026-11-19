use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub(crate) struct Args {
    /// The path for the first approach's timings, newline separated.
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub(crate) first: PathBuf,
    /// The path for the second approach's timings, newline separated.
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub(crate) second: PathBuf,
    /// The path for the full results, in JSON format.
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub(crate) output: Option<PathBuf>,
    /// The path for a pretty-formatted result output.
    /// Quiet output, don't print the results to STDOUT.
    #[arg(short, long)]
    pub(crate) quiet: bool,
}
