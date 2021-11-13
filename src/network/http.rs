use crate::errors::Error;
use std::collections::HashMap;
use ureq::{ErrorKind, SerdeValue};

#[derive(Default)]
pub(crate) struct HTTPClient;

impl HTTPClient {
    pub(crate) fn send(&self, req: Request) -> Result<String, Error> {
        let mut request = match req.method {
            Method::GET => ureq::get(&req.url),
            Method::POST => ureq::post(&req.url),
            Method::PUT => ureq::put(&req.url),
            Method::DELETE => ureq::delete(&req.url),
        };

        request = request.set("iron_planet", env!("CARGO_PKG_VERSION"));

        for (k, v) in req.headers.iter() {
            request = request.set(k, v);
        }

        let response = if let Some(value) = req.body {
            request.send_json(value)
        } else {
            request.call()
        }
        .map_err(|e| match e.kind() {
            ErrorKind::InvalidUrl | ErrorKind::UnknownScheme | ErrorKind::Io => Error::ClientError,
            ErrorKind::ConnectionFailed | ErrorKind::TooManyRedirects => Error::ServerError,
            ErrorKind::Dns
            | ErrorKind::InvalidProxyUrl
            | ErrorKind::ProxyConnect
            | ErrorKind::ProxyUnauthorized => Error::NetworkError,
            ErrorKind::HTTP => Error::ClientError,
            _ => Error::ServerError,
        })?;

        let code = response.status();

        match code {
            100..=399 => Ok(response.into_string().map_err(|_| Error::DecodeError)?),
            400..=499 => Err(Error::ResourceMissingError),
            500..=699 => Err(Error::ServerError),
            _ => Err(Error::NamelessError),
        }
    }
}

pub(crate) struct RequestBuilder<'a> {
    url: &'a str,
    headers: HashMap<String, String>,
    method: Method,
    body: Option<SerdeValue>,
}

impl<'a> RequestBuilder<'a> {
    pub(crate) fn new(url: &'a str) -> Self {
        RequestBuilder {
            url,
            headers: HashMap::default(),
            method: Method::GET,
            body: None,
        }
    }

    pub(crate) fn add_header(&mut self, key: String, value: String) -> &mut RequestBuilder<'a> {
        self.headers.insert(key, value);
        self
    }

    pub(crate) fn set_method(&mut self, method: Method) -> &mut RequestBuilder<'a> {
        self.method = method;
        self
    }

    pub(crate) fn set_body(&mut self, value: SerdeValue) -> &mut RequestBuilder<'a> {
        self.body = Some(value);
        self
    }

    pub(crate) fn build(&self) -> Request {
        Request {
            url: self.url.to_string().clone(),
            headers: self.headers.clone(),
            method: self.method,
            body: self.body.clone(),
        }
    }
}

pub(crate) struct Request {
    url: String,
    headers: HashMap<String, String>,
    method: Method,
    body: Option<SerdeValue>,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

// pub(crate) type Headers = HashMap<String, String>;
