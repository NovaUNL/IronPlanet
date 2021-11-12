

pub enum Error {
    ServerFailure { code: u16, url: String },
    NamelessError,
    ServerError,
    ParsingError,
    FooError(i32),
}