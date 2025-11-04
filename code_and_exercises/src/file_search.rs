use clap::{Parser, Subcommand};
use std::fs;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(version, about= "A simple CLI file search utility", long_about=None)]

struct Cli{
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand, Debug)]
enum Commands{
    Search{
        #[arg(short, long)]
        query: String,
        #[arg(short, long)]
        filename: String,
        #[arg(short, long)]
        ignore_case: bool,
    },
}

fn main() -> Result<()> {
    let cli= Cli::parse();

    match &cli.command{
        Commands::Search{query, filename, ignore_case} => {
            search_file(query, filename, *ignore_case)
        }
    }

}

fn search_file(query: &str, filename: &str, ignore_case: bool) -> Result<()> {
    let content = fs::read_to_string(filename)
        .with_context(|| format!("Could not read file: {}", filename))?;

    let search_query= if ignore_case {
        query.to_lowercase()
    }
    else{
        query.to_string()
    };

    let mut found_matches= false;

    println!("Searching for '{}' in '{}'", query, filename);
    println!("-------------------------------\n");

    for (line_number, line) in content.lines().enumerate() {

        if line.contains(&search_query){
            found_matches= true;

            let highlighted_line= highlight_match(line, &query);
            println!("{}: {}", line_number+1, highlighted_line);
        }
    }

    println!("\n------------------------------");
    if found_matches{
        println!("Search completed. Match(es) found.");
    } else {
        println!("Search completed. No matches found.");
    }

    Ok(())


}

fn highlight_match(line: &str, query: &str) ->String{
    let mut highlighted= String::new();
    let mut last_index= 0;

    for (index, _) in line.match_indices(&query){
        highlighted.push_str(&line[last_index..index]);


        highlighted.push_str(&format!("\x1b[32;1m{}\x1b[0m", &line[index..index+ query.len()]));

        last_index= index + query.len();
    }

    highlighted.push_str(&line[last_index..]);
    return highlighted;
}