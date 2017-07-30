use std::io;
use std::io::prelude::*;
use std::fs::File;
use serde_json;

use std::collections::HashMap;

type AssetMap = HashMap<String, Asset>;
type Asset = HashMap<String, String>;

pub fn find<'a>(map: &'a AssetMap, kind: &str, name: &str) -> Option<&'a String> {
    if let Some(asset) = map.get(name) {
        asset.get(kind)
    } else {
        None
    }
}

pub fn load(path: &str) -> Result<AssetMap, Error> {
    let mut f = File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(serde_json::from_str(&contents)?)
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}
