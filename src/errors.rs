use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // TODO get rid of this one
    #[error("Something possibly bad failed not to happen")]
    Generic,
    // -----------
    #[error("An error occurred on the server")]
    Server,
    #[error("Failed to decode the received message")]
    Decode,
    #[error("Received a message that didn't match the expected format:\n{0}")]
    Parsing(String),
    #[error("A resource is missing on the server")]
    ResourceMissing,
    #[error("Attempted to access a protected resource without providing credentials")]
    MissingAuthentication,
    #[error("Failed to authenticate to the server")]
    Authentication,
    #[error("Client-side error")]
    Client,
    #[error("Network failure")]
    Network,
}
