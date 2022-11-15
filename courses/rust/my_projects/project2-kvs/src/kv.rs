use failure::{format_err, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
struct KvData {
    op: u8,
    key: String,
    value: Option<String>,
}

#[derive()]
pub struct KvStore {
    map: HashMap<String, String>,
    data: Vec<KvData>,
    filepath: PathBuf,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> Result<KvStore> {
        Ok(KvStore {
            map: HashMap::new(),
            data: Vec::new(),
            filepath: PathBuf::from("datafile"),
        })
    }

    /// open a kvstore file
    pub fn open(file_path: &std::path::Path) -> Result<KvStore> {
        let filepath = file_path.join("datafile");
        if !filepath.exists() {
            File::create(&filepath)?;
        }
        let data = std::fs::read_to_string(&filepath)?;

        let mut kvdata: Vec<KvData> = Vec::new();
        if !data.is_empty() {
            kvdata = serde_json::from_str(&data)?;
        }
        // eprintln!("{:?}", kvdata);
        let mut map: HashMap<String, String> = HashMap::new();
        for item in kvdata.iter() {
            match item.op {
                0 => {
                    map.remove(&item.key);
                }
                1 => {
                    map.insert(
                        item.key.clone(),
                        item.value.clone().expect("Retriving log: False format."),
                    );
                }
                _ => return Err(format_err!("bad data.")),
            }
        }

        Ok(KvStore {
            map,
            data: kvdata,
            filepath: PathBuf::from(file_path.join("datafile")),
        })
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.data.push(KvData {
            op: 1,
            key: key.clone(),
            value: Some(value.clone()),
        });

        self.map.insert(key, value);
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).map(|s| s.to_string()))
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<String> {
        self.data.push(KvData {
            op: 0,
            key: key.clone(),
            value: None,
        });

        match self.map.remove(&key) {
            Some(v) => Ok(v), 
            None => Err(format_err!("Key not found")),
        }
    }

    /// compact the data
    fn compact(&mut self) -> Result<()> {
        self.data.clear();
        for pair in self.map.iter() {
            self.data.push(
                KvData {
                   op: 1,
                   key: pair.0.to_string(),
                   value: Some(pair.1.to_string()), 
                }
            )
        }
        Ok(())
    }

    /// commit the data, overwrite them all
    fn commit(&self) -> Result<()> {
        // let mut file = OpenOptions::new()
        //     .write(true)
        //     .open(self.filepath.as_path())
        //     .unwrap();
        let mut file = File::create(self.filepath.as_path())?;

        let new_json = serde_json::to_string(&self.data).unwrap();
        file.write(new_json.as_bytes()).unwrap();
        Ok(())
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        self.compact().unwrap();
        self.commit().unwrap();
    }
}
