extern crate clap; 
extern crate ring; 
extern crate anyhow; 

use clap::{App, Arg}; 
use ring::aead; 
use std::fs;
use anyhow::Result;

fn main () -> Result<()> {
    let matches = App::new("Rust Encryption Tool")
    .version("1.0")
    .author("Jackson Stephens")
    .about("Encrypts and decrypts files using AES-GCM")
    .arg(Arg::with_name("encrypt")
        .short("e")
        .long("encrypt")
        .value_name("FILE")
        .help("Encrypts the specified file.")
        .takes_value(true))
    .arg(Arg::with_name("decrypt")
        .short("d")
        .long("decrypt")
        .value_name("FILE")
        .help("Decrypts the specified file.")
        .takes_value(true))
    .get_matches();

    
}