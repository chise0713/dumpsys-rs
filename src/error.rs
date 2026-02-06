use std::io;

use rsbinder::error::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    DumpStatus(#[from] StatusCode),
    #[error("invalid method call for current `Dumpsys` pipe-type")]
    InvalidMethod,
    #[error("service not exist")]
    ServiceNotExist,
    #[error("no such entry found in `Dumpsys`")]
    NoEntryFound,
}
