use crate::errors::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
use ureq::ErrorKind;

#[derive(Default)]
pub(crate) struct HTTPClient;

impl HTTPClient {
    #[allow(clippy::unused_self)]
    pub(crate) fn send(&self, req: Request) -> Result<String, Error> {
        let mut request = match req.method {
            Method::Get => ureq::get(&req.url),
            Method::Post => ureq::post(&req.url),
            Method::Put => ureq::put(&req.url),
            Method::Delete => ureq::delete(&req.url),
        };

        request = request.set("iron_planet", env!("CARGO_PKG_VERSION"));

        for (k, v) in &req.headers {
            request = request.set(k, v);
        }

        let response = if let Some(value) = req.body {
            request.send_json(value)
        } else {
            request.call()
        }
        .map_err(|e| match e.kind() {
            ErrorKind::InvalidUrl | ErrorKind::UnknownScheme | ErrorKind::Io => Error::Client,
            ErrorKind::ConnectionFailed
            | ErrorKind::TooManyRedirects
            | ErrorKind::BadStatus
            | ErrorKind::BadHeader
            | ErrorKind::HTTP => Error::Server,
            ErrorKind::Dns
            | ErrorKind::InvalidProxyUrl
            | ErrorKind::ProxyConnect
            | ErrorKind::ProxyUnauthorized => Error::Network,
        })?;

        let code = response.status();

        match code {
            100..=399 => Ok(response.into_string().map_err(|_| Error::Decode)?),
            400..=499 => Err(Error::ResourceMissing),
            500..=699 => Err(Error::Server),
            _ => Err(Error::Generic),
        }
    }

    pub(crate) fn fetch_bytes(&self, req: Request) -> Result<Vec<u8>, Error> {
        let mut request = ureq::get(&req.url).set("iron_planet", env!("CARGO_PKG_VERSION"));

        for (k, v) in &req.headers {
            request = request.set(k, v);
        }

        let response = if let Some(value) = req.body {
            request.send_json(value)
        } else {
            request.call()
        }
        .map_err(|e| match e.kind() {
            ErrorKind::InvalidUrl | ErrorKind::UnknownScheme | ErrorKind::Io => Error::Client,
            ErrorKind::ConnectionFailed
            | ErrorKind::TooManyRedirects
            | ErrorKind::BadStatus
            | ErrorKind::BadHeader
            | ErrorKind::HTTP => Error::Server,
            ErrorKind::Dns
            | ErrorKind::InvalidProxyUrl
            | ErrorKind::ProxyConnect
            | ErrorKind::ProxyUnauthorized => Error::Network,
        })?;

        let code = response.status();

        match code {
            100..=399 => {
                let mut bytes: Vec<u8> = Vec::new();
                response
                    .into_reader()
                    .read_to_end(&mut bytes)
                    .map_err(|_| Error::Decode)?;
                Ok(bytes)
            }
            400..=499 => Err(Error::ResourceMissing),
            500..=699 => Err(Error::Server),
            _ => Err(Error::Generic),
        }
    }
}

pub(crate) struct RequestBuilder<'a> {
    url: &'a str,
    headers: HashMap<String, String>,
    method: Method,
    body: Option<Value>,
}

impl<'a> RequestBuilder<'a> {
    pub(crate) fn new(url: &'a str) -> Self {
        RequestBuilder {
            url,
            headers: HashMap::default(),
            method: Method::Get,
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

    pub(crate) fn set_body(&mut self, value: Value) -> &mut RequestBuilder<'a> {
        self.body = Some(value);
        self
    }

    pub(crate) fn build(&self) -> Request {
        Request {
            url: self.url.to_string(),
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
    body: Option<Value>,
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum Method {
    Get,
    Post,
    Put,
    Delete,
}

// pub(crate) type Headers = HashMap<String, String>;
