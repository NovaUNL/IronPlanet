#[derive(Debug)]
pub enum Error {
    ServerFailure { code: u16, url: String },
    NamelessError,
    ServerError,
    DecodeError,
    ParsingError,
    ResourceMissingError,
    MissingAuthenticationError,
    AuthenticationError,
    ClientError,
    NetworkError,
    FooError(i32),
}
