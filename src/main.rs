use std::{io::Write, path::Path};

use clap::Parser;
use t_test::{error::Error, observations::{Observations, parse}};

use crate::{cli::Args};

mod cli;

fn write_results(observations: &Observations, path: &Path) -> std::io::Result<()> {
    if path.try_exists()? {
        std::fs::remove_file(path)?;
    }
    let mut file = std::fs::File::create_new(path)?;
    file.write("{".as_bytes())?;
    file.write(observations.to_json().as_bytes())?;
    file.write("\n}\n".as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let Args {
        input,
        output,
        mean,
        quiet,
    }: Args = Args::parse();

    let observations = parse(&input)?;

    let timings: Observations = Observations::new(observations, mean);
    if let Some(path) = output {
        write_results(&timings, &path)?;
    }
    if !quiet {
        println!("{timings}");
    }
    Ok(())
}
