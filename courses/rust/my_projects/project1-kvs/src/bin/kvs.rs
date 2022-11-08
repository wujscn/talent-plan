use clap::{arg, command};
use std::process::exit;

use kvs::KvStore;

fn main() {
    let matches = command!()
        .arg(arg!([op] "Operation: rm, set, get"))
        .arg(arg!([key] "Key"))
        .arg(arg!([value] "Value"))
        .get_matches();

    let mut kvs = KvStore::new();

    if let Some(op) = matches.get_one::<String>("op") {
        let key = match matches.get_one::<String>("key") {
            Some(k) => k,
            None => panic!("No key specified!"),
        };
        match op.as_str() {
            "rm" => {
                kvs.remove(key.to_string());
                eprintln!("unimplemented");
                exit(1)
            }
            "get" => match kvs.get(key.to_string()) {
                Some(v) => println!("{}: {}", key, v),
                None => {
                    eprintln!("unimplemented");
                    exit(1)
                }
            },
            "set" => {
                let value = match matches.get_one::<String>("value") {
                    Some(v) => v,
                    None => panic!("No value specified!"),
                };
                kvs.set(key.to_string(), value.to_string());
                eprintln!("unimplemented");
                exit(1)
            }
            _ => exit(1),
        }
    }

    exit(1)
}
