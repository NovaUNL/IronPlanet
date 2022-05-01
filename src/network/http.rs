use crate::errors::Error;

use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::http::request;
use hyper::{Body, Client, Method, Response as HyperResponse, StatusCode};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) struct Request {
    builder: request::Builder,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Request {
    pub(crate) fn new(url: &str) -> Request {
        let builder = request::Builder::new()
            .uri(url)
            .method(Method::GET)
            .header("iron_planet", env!("CARGO_PKG_VERSION"));

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Request { builder, client }
    }

    pub(crate) fn header(mut self, k: String, v: String) -> Request {
        self.builder = self.builder.header(k, v);
        self
    }

    pub(crate) fn attach_token(self, token: &str) -> Request {
        self.header("Authorization".to_string(), format!("Token {}", token))
    }

    pub(crate) fn method(mut self, method: Method) -> Request {
        self.builder = self.builder.method(method);
        self
    }

    pub(crate) async fn body(self, body: Body) -> Result<Response, Error> {
        let response = self
            .client
            .request(self.builder.body(body).map_err(|_| Error::Generic)?)
            .await
            .map_err(|_| Error::Network)?;

        check_error(response.status())?;

        Ok(Response(response))
    }

    pub(crate) async fn send(self) -> Result<Response, Error> {
        self.body(Body::default()).await
    }

    pub(crate) async fn send_serializable<T: Serialize>(
        self,
        data: &T,
    ) -> Result<Response, Error> {
        let json = serde_json::to_string(data)
            .map_err(|err| Error::Serialization(err))?;
        let body = Body::from(json);
        self.body(body).await
    }
}

pub(crate) struct Response(HyperResponse<Body>);

impl Response {
    pub(crate) async fn to_vec(self) -> Result<Vec<u8>, Error> {
        let data = hyper::body::to_bytes(self.0)
            .await
            .map_err(|_err| Error::Generic)?
            .to_vec();
        Ok(data)
    }

    pub(crate) async fn deserialize<T: DeserializeOwned>(
        mut self,
    ) -> Result<T, Error> {
        let body = hyper::body::aggregate(&mut self.0)
            .await
            .map_err(|_err| Error::Generic)?;

        let data: T = serde_json::from_reader(body.reader())
            .map_err(|err| Error::Deserialization(err))?;
        Ok(data)
    }

    pub(crate) fn code(&self) -> StatusCode {
        self.0.status()
    }
}

pub(crate) fn check_error(code: StatusCode) -> Result<(), Error> {
    let ucode = code.as_u16();

    if code.is_server_error() {
        Err(Error::Server)
    } else if ucode == 404 {
        Err(Error::ResourceMissing)
    } else if code.is_client_error() {
        Err(Error::Client)
    } else if code.is_redirection() {
        Err(Error::ResourceMissing)
    } else {
        Ok(())
    }
}
