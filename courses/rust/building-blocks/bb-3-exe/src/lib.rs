
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RedisCmd {
    SimpleString {data: String},
    Error {data: String},
    Integer {data: u64},
    BulkString {data: String},
    Array {data: Vec<String>},
}