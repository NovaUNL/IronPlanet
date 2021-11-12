use std::collections::HashMap;
use crate::errors::Error;

#[derive(Default)]
pub struct HTTPClient;

impl HTTPClient {
    // pub(crate) fn set_credentials(&mut self, credentials: Credentials) {
    //     self.credentials = Some(credentials)
    // }

    // async fn validate_token(token: &AuthToken) -> Result<String, Error> {
    //     HTTPClient::noauth_get(&Endpoint::TokenValidation.to_string());
    //     Ok("result".to_string())
    // }

    // async fn noauth_get(url: &str) -> Result<Data<'_>, Error> {
    //     Err(Error::NamelessError)
    // }

    pub(crate) fn send(&self, req: Request) -> Result<String, Error> {
        let mut request = match req.method {
            Method::GET => ureq::get(&req.url),
            // Method::POST => ureq::post(&req.url),
            // Method::PUT => ureq::put(&req.url),
            // Method::DELETE => ureq::delete(&req.url)
        };

        for (k, v) in req.headers.iter() {
            request = request.set(k, v);
        }

        let response = request.call().map_err(|_| Error::ServerError)?;

        Ok(response.into_string().map_err(|_| Error::NamelessError)?)
    }
}

pub(crate) struct RequestBuilder<'a> {
    url: &'a str,
    headers: HashMap<String, String>,
    method: Method,
    body: Option<String>,
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

    pub(crate) fn build(&self) -> Request {
        Request {
            url: self.url.to_string().clone(),
            headers: self.headers.clone(),
            method: self.method,
            _body: self.body.clone(),
        }
    }
}


pub(crate) struct Request {
    url: String,
    headers: HashMap<String, String>,
    method: Method,
    _body: Option<String>,
}


// pub(crate) struct Response {
//     headers: HashMap<String, String>,
//     code: u16,
//     body: Option<String>,
// }

#[derive(Copy, Clone)]
pub(crate) enum Method {
    GET,
    // POST,
    // PUT,
    // DELETE,
}

// pub(crate) type Headers = HashMap<String, String>;