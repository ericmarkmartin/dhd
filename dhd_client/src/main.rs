use anyhow::{anyhow, Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use crc::crc32;
use dhd_client::client::DhdClient;
use dhd_core::hashlist::{Hash, HashList};
use std::fs::{self, File};
use std::io::{self, Read, Write};

fn read_input(filename: Option<&str>) -> Result<String> {
    let contents = if let Some(filename) = filename {
        fs::read_to_string(filename)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };
    Ok(contents)
}

fn dhd_push(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("server").context("no server specified")?;
    let url = format!("{}/graphql", server);
    let filename = matches.value_of("input");
    let username = matches
        .value_of("username")
        .context("no username provided")?;
    let contents = read_input(filename).context("could not read input")?;
    let hashlist: HashList = contents
        .split('\n')
        .map(|x| crc32::checksum_ieee(x.as_bytes()) as Hash)
        .collect::<Vec<Hash>>()
        .into();

    let client = DhdClient::new(&url)?;
    let id = client.push(username, hashlist)?;
    println!("Created hash '{}'!", id);

    Ok(())
}

fn write_output(filename: Option<&str>, contents: String) -> Result<()> {
    if let Some(filename) = filename {
        let mut file = File::create(filename)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
    } else {
        io::stdout().write_all(contents.as_bytes())?;
        io::stdout().write_all(b"\n")?;
    };
    Ok(())
}

fn dhd_pull(matches: &ArgMatches) -> Result<()> {
    let server = matches.value_of("server").context("no server specified")?;
    let url = format!("{}/graphql", server);
    let filename = matches.value_of("output");
    let username = matches
        .value_of("username")
        .context("no username provided")?;

    let client = DhdClient::new(&url)?;
    let hashlist = client.pull(username)?;
    let contents = hashlist.to_delimited_string();
    write_output(filename, contents)?;

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
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("INPUT")
                        .help("Sets the input file.")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .long("server")
                        .value_name("URL")
                        .help("The DHD server to use.")
                        .env("DHD_SERVER")
                        .required(true),
                )
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .value_name("USERNAME")
                        .help("The username to save the hash under.")
                        .env("DHD_USERNAME")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull a line hash from the server.")
                .arg(
                    Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .value_name("USERNAME")
                        .help("The username of the hash to download.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .long("server")
                        .value_name("URL")
                        .help("The DHD server to use.")
                        .env("DHD_SERVER")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Sets the output file.")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("push", Some(sub_matches)) => dhd_push(sub_matches),
        ("pull", Some(sub_matches)) => dhd_pull(sub_matches),
        _ => Err(anyhow!("Bad subcommand")),
    }
}
