use anyhow::{anyhow, Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use crc::crc32;
use dhd_client::DhdClient;
use dhd_core::hashlist::{Hash, HashList};
use std::fs;

fn dhd_push(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("server").context("no server specified")?;
    let url = format!("{}/graphql", server);
    let filename = matches.value_of("file").context("no filename provided")?;
    let contents = fs::read_to_string(filename)?;
    let lines = contents.split('\n');
    let hashlist: HashList = lines
        .map(|x| crc32::checksum_ieee(x.as_bytes()) as Hash)
        .collect::<Vec<Hash>>()
        .into();

    let client = DhdClient::new(&url)?;
    let id = client.push(hashlist)?;
    println!("Created hash #{}!", id);

    Ok(())
}

fn dhd_pull(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("server").context("no server specified")?;
    let url = format!("{}/graphql", server);
    let id = matches
        .value_of("id")
        .context("no id specified")?
        .parse::<u32>()
        .context("invalid id")?;

    let client = DhdClient::new(&url)?;
    let hashlist = client.pull(id)?;

    for hash in <Vec<i32>>::from(hashlist) {
        println!("{}", hash as u32);
    }

    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("Distributed Hash Diff")
        .version("0.1")
        .about("A networked service for comparing files by line hashes.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("push")
                .about("Push a line hash of a local file.")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .value_name("FILE")
                        .help("Sets the input file.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .value_name("URL")
                        .help("The DHD server to use.")
                        .required(false)
                        .default_value("http://localhost:8000"),
                ),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull a line hash from the server.")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("Sets the ID of the hash to fetch.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .value_name("URL")
                        .help("The DHD server to use.")
                        .required(false)
                        .default_value("http://localhost:8000"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("push", Some(sub_matches)) => dhd_push(sub_matches),
        ("pull", Some(sub_matches)) => dhd_pull(sub_matches),
        _ => Err(anyhow!("Bad subcommand")),
    }
}
