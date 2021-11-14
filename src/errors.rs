#[derive(Debug)]
pub enum Error {
    // TODO get rid of this one
    Generic,
    // -----------
    Server,
    Decode,
    Parsing,
    ResourceMissing,
    MissingAuthentication,
    Authentication,
    Client,
    Network,
}
