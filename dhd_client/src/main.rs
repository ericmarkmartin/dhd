extern crate clap;
use clap::{Arg, ArgMatches, App, AppSettings, SubCommand};
use std::fmt;
use std::error;
use std::fs;
use crc::crc32;
use dhd_core::hashlist::{Hash, HashList};

#[derive(Debug)]
struct DhdError {
    reason: String
}

impl fmt::Display for DhdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DHD ran into an error: {}", &self.reason)
    }
}

impl error::Error for DhdError {
    fn description(&self) -> &str {
        &self.reason
    }
}

impl DhdError {
    pub fn new(reason: &str) -> Self {
        Self { reason: reason.to_string() }
    }
}

fn dhd_push(matches: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let server = matches.value_of("server").ok_or(DhdError::new("no server specified"))?;
    let filename = matches.value_of("file").ok_or(DhdError::new("no filename provided"))?;
    let contents = fs::read_to_string(filename)?;
    let lines = contents.split('\n');
    let hashes: HashList = HashList::from(lines.map(|x| crc32::checksum_ieee(x.as_bytes())).collect::<Vec<Hash>>());
    Ok(())
}

fn dhd_pull(_matches: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("Distributed Hash Diff")
        .version("0.1")
        .about("A networked service for comparing files by line hashes.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("push")
            .about("Push a line hash of a local file.")
            .arg(Arg::with_name("file")
                .short("f")
                .value_name("FILE")
                .help("Sets the input file.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("server")
                .short("s")
                .value_name("URL")
                .help("The DHD server to use.")
                .required(false)
                .default_value("localhost")))
        .subcommand(SubCommand::with_name("pull")
            .about("Pull a line hash from the server.")
            .arg(Arg::with_name("id")
                .value_name("ID")
                .help("Sets the ID of the hash to fetch.")
                .required(true)
                .index(1))
            .arg(Arg::with_name("server")
                .short("s")
                .value_name("URL")
                .help("The DHD server to use.")
                .required(false)
                .default_value("localhost")))
        .get_matches();

    match matches.subcommand() {
        ("push", Some(sub_matches)) => dhd_push(sub_matches),
        ("pull", Some(sub_matches)) => dhd_pull(sub_matches),
        _ => Err(DhdError::new("Bad subcommand").into())
    }
}
