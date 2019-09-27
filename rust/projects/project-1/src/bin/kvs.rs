use clap::{App, AppSettings, Arg, SubCommand};
//use std::process::exit;

fn main() {
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
    let mut kv = kvs::KvStore::new();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            kv.set(
                _matches.value_of("KEY").unwrap().to_string(),
                _matches.value_of("VALUE").unwrap().to_string(),
            );
        }
        ("get", Some(_matches)) => {
            println!(
                "{:?}\n",
                kv.get(_matches.value_of("KEY").unwrap().to_string())
            );
        }
        ("rm", Some(_matches)) => {
            kv.remove(_matches.value_of("KEY").unwrap().to_string());
        }
        _ => unreachable!(),
    }
}
