use thiserror::Error;

#[derive(Error, Debug)]
pub enum FairCodingApiError {
    #[error("Event parsing error : {0}")]
    EventParsing(String),
    #[error("Error handling Repository::{action:?} cause {msg:?}")]
    RepositoryEvent { action: String, msg: String },
}
