use std::{
    fs::File,
    io::{BufRead, Write},
    path::Path,
    time::Duration,
};

use clap::Parser;
use t_test::differences::Differences;

use crate::{cli::Args, error::Error};

mod cli;
mod error;

fn write_results(timings: &Differences, path: &Path) -> std::io::Result<()> {
    if path.try_exists()? {
        std::fs::remove_file(path)?;
    }
    let mut file = std::fs::File::create_new(path)?;
    file.write("{".as_bytes())?;
    file.write(timings.to_json().as_bytes())?;
    file.write("\n}\n".as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let Args {
        first,
        second,
        output,
        quiet,
    }: Args = Args::parse();

    let first_times = parse(&first)?;
    let second_times = parse(&second)?;

    let timings: Differences = Differences::new(
        first_times
            .into_iter()
            .zip(second_times.into_iter())
            .map(|(a, b)| a.as_secs_f64() - b.as_secs_f64())
            .collect(),
    );
    if let Some(path) = output {
        write_results(&timings, &path)?;
    }
    if !quiet {
        println!("{timings}");
    }
    Ok(())
}
