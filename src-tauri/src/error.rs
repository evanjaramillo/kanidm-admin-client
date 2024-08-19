use std::fmt::Debug;

use kanidm_client::ClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
    #[error("Missing required value: {0}")]
    ValueMissing(String),
    #[error("Kanidm client error")]
    KanidmClientError(ClientError)
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(err.to_string())
    }
}

impl From<ClientError> for Error {
    fn from(value: ClientError) -> Self {
        Error::KanidmClientError(value)
    }
}