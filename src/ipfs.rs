use serde_json;
use hyper::{self, Client, Url};
use multipart::client::lazy::Multipart;

use std::io;
use std::io::prelude::*;

use structs::Essay;

#[derive(Debug, Deserialize, Serialize)]
pub struct IpfsAddResponse {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Hash")]
    pub hash: String,
}

pub fn add(essay: Essay) -> Result<IpfsAddResponse, Error> {
    let mut binary = essay.text.as_bytes();
    let mut res = Multipart::new()
        .add_stream("file", &mut binary, None as Option<&str>, None)
        .client_request(&Client::new(), "http://go-ipfs:5001/api/v0/add?pin=true")?;

    assert_eq!(res.status, hyper::Ok);

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let result = serde_json::from_str(&body)?;
    Ok(result)
}

// it is recommended to proxy to go-ipfs instead in production
pub fn cat(hash: &str) -> Result<String, Error> {
    let client = Client::new();

    let url = Url::parse_with_params("http://go-ipfs:5001/api/v0/cat", &[("arg", hash)]).unwrap();
    let mut res = client.get(url).send()?;
    assert_eq!(res.status, hyper::Ok);

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    Ok(body)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Http(hyper::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}
