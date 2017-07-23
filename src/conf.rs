extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use serde_yaml::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  pub branch: String,
  pub users: BTreeMap<String, String>,
  pub tags: BTreeMap<String, String>,
}

impl Config {
  pub fn new(path: &str) -> Result<Config, Box<Error>> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let conf = serde_yaml::from_str(contents).unwrap();
    Ok(conf)
  }
}

