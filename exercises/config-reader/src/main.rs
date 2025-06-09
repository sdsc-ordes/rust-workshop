use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// An imaginary config file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    port: u16,
    base_url: &'a str,
    s3_path: &'a str,
    database_url: &'a str,
}

#[derive(Debug)]
/// Config deserialization error
pub enum Error {
    /// Something went wrong deserializing JSON
    Json(serde_json::Error),
    /// Something went wrong deserializing YAML
    Yaml(serde_yaml::Error),
}

trait ConfigDeserializer {
    /// Deserialize the contents into a `Config`
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error>;
}

struct YamlDeserializer {}
struct JsonDeserializer {}

impl ConfigDeserializer for YamlDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        serde_yaml::from_str::<Config<'a>>(contents).map_err(|e| Error::Yaml(e))
    }
}

impl ConfigDeserializer for JsonDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        serde_json::from_str::<Config<'a>>(contents).map_err(|e| Error::Json(e))
    }
}

fn main() {
    let mut args = std::env::args();

    // Unwrapping is OK here, as UTF-8 Strings can always be converted to PathBufs
    let Some(path) = args.nth(1).map(|a| PathBuf::try_from(a).unwrap()) else {
        eprintln!("Please specify the input path");
        return;
    };

    // Unwrapping is Ok as `path` was created from UTF-8 string, and so is the extension
    let extension = path.extension().map(|o| o.to_str().unwrap());
    let file_contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            // `path` was created from an UTF-8 string, so can be converted to one
            eprintln!(
                "Error reading file at path {}: {}",
                path.to_str().unwrap(),
                e
            );
            return;
        }
    };

    let deserializer: &dyn ConfigDeserializer = match extension {
        Some("yml") | Some(".yaml") => &YamlDeserializer {},
        Some("json") => &JsonDeserializer {},
        Some(c) => {
            eprintln!("Cannot deserialize config with extension '{c}'.");
            return;
        }
        _ => {
            eprintln!("Cannot detect extension '{path:?}'.");
            return;
        }
    };

    let config: Config = match deserializer.deserialize(&file_contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error deserializing config. {:?}", e);
            return;
        }
    };

    println!("Config was: {config:?}");
}
