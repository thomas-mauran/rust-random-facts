use thiserror::Error;

#[derive(Error,Debug)]
pub enum ApiError{
    #[error("Error fetching fact")]
    FetchError(#[from] reqwest::Error),

    #[error("Error while converting the payload")]
    JsonError(#[from] serde_json::Error)
}