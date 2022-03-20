use std;
use thiserror::Error;

use reqwest;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    HTTP(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
