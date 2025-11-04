use std::env;
use std::fs;

fn main() {
    let args: Vec<String>= env::args().collect();


    let query: &String= &args[1];
    let filename:&String= &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", filename);
    
    let content= fs::read_to_string(filename);
    
    
    match content{
        Err(value)=> println!("Something went wrong !!"),
        Ok(value)=> {
            if value.contains(query){
                println!("Match found !!");
            }        
            else{
                println!{"No match found !!"};
            }
        }
    }

    
}
