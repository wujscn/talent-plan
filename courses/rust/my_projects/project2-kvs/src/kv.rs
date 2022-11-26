use failure::{format_err, Error};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader, BufWriter, SeekFrom};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

const COMPACT_LIMIT: u64 = 1024 * 8;

#[derive(Serialize, Deserialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

pub struct CommandEntry {
    pos: u64,
    len: u64,
}

#[derive()]
pub struct KvStore {
    filepath: PathBuf,
    index: HashMap<String, CommandEntry>,
    writer: BufWriter<File>,
    cmd_num: u64,
    pos: u64,
}

impl KvStore {
    /// open a kvstore file
    pub fn open(file_path: &std::path::Path) -> Result<KvStore> {
        let filepath = file_path.join("datafile");
        if !filepath.exists() {
            File::create(&filepath)?;
        }

        let file = File::open(filepath.clone())?;
        let mut reader = BufReader::new(&file);
        let mut pos = reader.seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();

        let mut index = HashMap::new();
        let mut cmd_num = 0;

        while let Some(cmd) = stream.next() {
            let cmd = cmd?;
            let new_pos = stream.byte_offset() as u64;
            let len = new_pos - pos;
            match cmd {
                Command::Set { key, .. } => {
                    index.insert(key, CommandEntry { pos, len });
                }
                Command::Remove { key } => {
                    index.insert(key, CommandEntry { pos, len });
                }
            }

            pos = new_pos;
            cmd_num += 1;
        }

        // println!("Init: {:?}", index.keys());

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filepath)?;
        let mut writer = BufWriter::new(file);

        writer.seek(SeekFrom::Start(pos))?;

        Ok(KvStore {
            filepath,
            index,
            writer,
            cmd_num,
            pos,
        })
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let payload = serde_json::to_string(&cmd)?;
        self.writer.write(payload.as_bytes())?;

        let new_pos = self.writer.stream_position()?;
        self.index.insert(
            key,
            CommandEntry {
                pos: self.pos,
                len: new_pos - self.pos,
            },
        );
        self.pos = new_pos;
        self.cmd_num += 1;

        self.compact()?;

        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.index.get(&key) {
            Some(cmd_entry) => {
                let file = OpenOptions::new().read(true).open(&self.filepath)?;
                let mut buf_reader = BufReader::new(file);
                match read_single(&mut buf_reader, cmd_entry.pos, cmd_entry.len)? {
                    Command::Set { key: _, value: v } => Ok(Some(v)),
                    Command::Remove { key: _ } => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<String> {
        let pre_value = match self.get(key.clone()).expect("Key not found") {
            None => {
                return Err(format_err!("Key not found"));
            }
            Some(v) => v,
        };

        let cmd = Command::Remove { key: key.clone() };
        let payload = serde_json::to_string(&cmd)?;
        self.writer.write(payload.as_bytes())?;

        let new_pos = self.writer.stream_position()?;
        self.pos = new_pos;
        self.cmd_num += 1;
        // self.index.insert(key, CommandEntry { pos: self.pos, len: new_pos - self.pos });
        self.index.remove(&key); // more efficient

        self.compact()?;

        Ok(pre_value)
    }

    /// compact the data
    fn compact(&mut self) -> Result<()> {
        if self.cmd_num < COMPACT_LIMIT {
            return Ok(());
        }

        let archaic_file = std::path::Path::new("archaic");
        let file = OpenOptions::new().read(true).open(&self.filepath)?;
        let mut reader = BufReader::new(file);
        fs::rename(&self.filepath, archaic_file)?;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.filepath)?;
        self.writer = BufWriter::new(file);

        let mut index = HashMap::new();
        self.pos = 0;
        self.cmd_num = 0;

        for cmd_entry in self.index.values() {
            match read_single(&mut reader, cmd_entry.pos, cmd_entry.len)? {
                Command::Set {
                    key,
                    value,
                } => {
                    let cmd = Command::Set {
                        key: key.clone(),
                        value: value.clone(),
                    };
                    let payload = serde_json::to_string(&cmd)?;
                    self.writer.write(payload.as_bytes())?;

                    let new_pos = self.writer.stream_position()?;
                    index.insert(
                        key,
                        CommandEntry {
                            pos: self.pos,
                            len: new_pos - self.pos,
                        },
                    );
                    self.pos = new_pos;
                    self.cmd_num += 1;
                }
                Command::Remove { key: _ } => (),
            }
        }
        fs::remove_file(archaic_file)?;
        Ok(())
    }
}

fn read_single<R: Read + Seek>(
    buf_reader: &mut BufReader<R>,
    pos: u64,
    len: u64,
) -> Result<Command> {
    buf_reader.seek(SeekFrom::Start(pos as u64))?;
    let taker = buf_reader.take(len as u64);
    let cmd: Command = serde_json::from_reader(taker)?;
    Ok(cmd)
}
