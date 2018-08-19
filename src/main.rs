// Setup {{{
// #[macro_use]
extern crate clap;
// #[macro_use]
extern crate duct;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate lazy_static;
extern crate csv;
// extern crate regex;
use std::fs::File;
// use std::io::prelude::*;
use clap::{App, Arg, SubCommand};
use duct::cmd;
// use regex::Regex;
use std::error::Error;
// use std::io::Write;
use std::process;
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
#[derive(Debug, Deserialize)]
struct Record {
    alias: String,
    ip: String,
    user: String,
    pass: String,
}
// lazy_static! {
//     pub static ref ROS: Regex = Regex::new(r#"(CentOS.*)\r\n"#).unwrap();
//     pub static ref RPHP: Regex = Regex::new(r#"(PHP.*)\r\nC"#).unwrap();
//     pub static ref RSSL: Regex = Regex::new(r#"(OpenSSL.*)\r\n"#).unwrap();
// }
// }}}
fn main() {
    // read csv {{{
    // init vecs
    let mut connections = vec![];
    if let Err(err) = read(&mut connections) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    // }}}
    // clap {{{
    let matches = App::new("wel")
        .version("1.0")
        .author("fish")
        .about("info cli")
        // Arguments:
        .arg(Arg::with_name("plane").help("function to exec").index(1))
        // Subcommands
        .subcommand(
            SubCommand::with_name("lp")
                .about("postsuper d")
                .arg(Arg::with_name("ips").required(true).min_values(1)),
        )
        .get_matches();
    // }}}
    // subcommands: {{{
    if let Some(matches) = matches.subcommand_matches("lp") {
        let ips: Vec<_> = matches.values_of("ips").unwrap().collect();
        if let Err(err) = lp(ips) {
            println!("{}", err);
            process::exit(1);
        }
    }
    if let Some(input) = matches.value_of("plane") {
        println!("INPUT: {}", input);
        // lookup input info from file
        let t = &connections
            .into_iter()
            .filter(|i| i.0 == input)
            .collect::<Vec<_>>();
        // get vars
        let ip = t[0].1.to_string();
        let user = t[0].2.to_string();
        let pass = t[0].3.to_string();
        // build ssh expect:
        if let Err(err) = ssh(ip, user, pass, "interact") {
            println!("{}", err);
            process::exit(1);
        }
    }
    // }}}
}
// ssh {{{
fn ssh(ip: String, user: String, pass: String, route: &str) -> Result<(), Box<Error>> {
    println!("{} {} {} {}", ip, user, pass, route);
    let test = vec_of_strings![
            "set prompt {[#|%|>|$] $}\n",
            format!("spawn ssh $env({})@{}", user, ip),
            "expect \"assword\"",
            format!("send \"$env({})\n\"", pass),
            "expect $prompt",
            "send \"mailq | grep Aug | grep 'test@aqua-vit.net' | awk '{print \\$1}' | postsuper -d -\n\"",
            "expect $prompt",
            "send \"exit\n\"",
            "expect eof",
            "exit"
        ];
    println!("{}", test.join(";"));
    let args = &["-c", &test.join(";")];
    let stdout = cmd("expect", args).read().unwrap();
    println!("{}", stdout);
    Ok(())
}
// }}}
// lp {{{
fn lp(ips: Vec<&str>) -> Result<(), Box<Error>> {
    println!("IPS: {:?}", ips);
    for i in ips {
        let test = vec_of_strings![
            "set prompt {[#|%|>|$] $}\n",
            format!("spawn ssh $env({})@{}", "UR", i),
            "expect \"assword\"",
            format!("send \"$env({})\n\"", "PG"),
            "expect $prompt",
            "send \"mailq | grep Aug | grep 'test@aqua-vit.net' | awk '{print \\$1}' | postsuper -d -\n\"",
            "expect $prompt",
            "send \"exit\n\"",
            "expect eof",
            "exit"
        ];
        println!("{}", test.join(";"));
        let args = &["-c", &test.join(";")];
        let stdout = cmd("expect", args).read().unwrap();
        println!("{}", stdout);
    }
    Ok(())
}
// }}}
// read file {{{
fn read(connections: &mut Vec<((String, String, String, String))>) -> Result<(), Box<Error>> {
    let file = File::open("list")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        connections.push((record.alias, record.ip, record.user, record.pass))
    }
    Ok(())
}
// }}}
