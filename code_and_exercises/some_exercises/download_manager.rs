/*
To do: {

Single-threaded, blocking downloader
- [ ]  M1- Single-threaded, blocking downloader
- CLI that takes a URL and saves to disk.
- Use reqwest::blocking or ureq.
- Show bytes download and total bytes if Content-Length present.
}
1. Build the request parser using reqwest..
2. Use cli to take the url..

*/

use clap::Parser;
use std::fs::File;
use std::io::{self, copy};

#[derive(Parser, Debug)]
#[command(version, about= "Download Manager", long_about= None)]
struct Cli{
    #[arg(short, long)]
    url: String,
}

/*
The return type: std::result::Result<(), Box<dyn std::error::Error>> 
is used to dynamically handle the error generated. There are two types of error 
generated in the current scenario: reqwest error and error due to the io::Error. 
Box<dyn ...> is used to dynamically handle the error generated without explictly 
mentioning the error return type, since the Result takes up two arguments <T, E> and 
error is of two types File::io::Error and reqwest. 

*/
fn download_file(url: String, file_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut dest = File::create(file_path)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}
fn main() {
    let cli= Cli::parse();
    //println!("Url: {}", cli.url);

    //let client = reqwest::blocking::Client::new();
    //let url_parser = client.get(cli.url).send();
    //let url_parser= reqwest::blocking::get(cli.url);
    match download_file(cli.url, "./file.png"){
        Ok(_)=> println!("File downloaded successfully..."),
        Err(e)=> println!("{}",e)
    };
     
}
