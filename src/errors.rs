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
    #[error("Unable to serialize data:\n{0}")]
    Serialization(serde_json::Error),
    #[error("Unable to deserialize data:\n{0}")]
    Deserialization(serde_json::Error),
    #[error("Received a message that didn't match the expected format:\n{0}")]
    Parsing(serde_json::Error, String),
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
    #[error("There was an error completing a request")]
    Request,
}
