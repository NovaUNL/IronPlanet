pub enum Error {
    ServerFailure { code: u16, url: String },
    NamelessError,
    ServerError,
    ParsingError,
    ResourceMissingError,
    AuthenticationError,
    ClientError,
    NetworkError,
    FooError(i32),
}
