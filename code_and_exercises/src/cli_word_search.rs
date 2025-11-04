use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about="A simple query engine from txt file", long_about=None)]

struct Args{
    #[arg(short, long)]
    query:String,
    
    #[arg(short, long)]
    filename:String,
}

fn main(){
    let arg= Args::parse();

    let content= fs::read_to_string(arg.filename);

    match content{
        Ok(value) => {
            if value.contains(&arg.query){
            println!("Match found !!");
            }
            else{
            println!("No match found !!");
        }       
        },

        Err(_)=> println!("Something went down !!")
    }
}
