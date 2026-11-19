use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub(crate) struct Args {
    /// The path for the file of values. The values need to be parsable as [`f64`]s.
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub(crate) input: PathBuf,
    /// The path for the full results, in JSON format.
    #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
    pub(crate) output: Option<PathBuf>,
    /// The mean to compare against.
    #[arg(short, long,)]
    pub(crate) mean: f64,
    /// Quiet output, don't print the results to STDOUT.
    #[arg(short, long)]
    pub(crate) quiet: bool,
}
