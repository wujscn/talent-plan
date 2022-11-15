use clap::{App, AppSettings, Arg, SubCommand};
use std::{path::Path, process::exit};

use kvs::{KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let mut store = KvStore::open(Path::new(""))?;

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let value = _matches.value_of("VALUE").unwrap();
            let key = _matches.value_of("KEY").unwrap();
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap();
            let v = store.get(key.to_string())?;
            match v {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        ("rm", Some(_matches)) => {
            let key = _matches.value_of("KEY").unwrap();
            match store.remove(key.to_string()) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    exit(1);
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
