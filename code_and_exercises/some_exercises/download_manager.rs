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
fn download_file(url: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url.clone())?;
    
    let file_path= match get_filename(url.clone()){
        Ok(filename)=> filename,
        Err(e)=> return Err(e.into())
    };
    if let Some(length)= response.content_length(){
        println!("Content Length is: {:.2} MB", (length as f64)/(1024.0*1024.0));
    }
    else{
        print!("Content-Length header is missing !!");
    }
    let mut dest = File::create(file_path)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}

fn get_filename(url: String)->Result<String, String>{
    let filename= url.split('/').last();
    match filename{
        Some(val)=>Ok(val.to_string()),
        _=>Err("Failed to extract filename".to_string())
    }
}

fn main() {
    let cli= Cli::parse();

    match download_file(cli.url){
        Ok(_)=> println!("File downloaded successfully..."),
        Err(e)=> println!("{}",e)
    };

    
}
