use std::{
    process::{ExitCode, Termination},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failure during file read")]
    Io(#[from] std::io::Error),
    #[error("bad format in file, contents weren't valid f64s")]
    Parse(#[from] std::num::ParseFloatError),
}

impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        match self {
            Error::Io(..) => ExitCode::from(1),
            Error::Parse(..) => ExitCode::from(2),
        }
    }
}