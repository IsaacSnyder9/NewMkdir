#!/usr/bin/env bash
use clap::Parser;
use std::fs::{self};
use std::path::{PathBuf};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Com {
    path: PathBuf,

    /// All paths you would like to add as a child of the provided path
    #[arg(num_args = 1..)]
    new_paths: Vec<String>,

    /// Removes all files and directories within the provided path before setting any new ones
    #[arg(short, long)]
    rmall: bool,
}


fn confirmation () -> bool {
    loop {
        print!("Y/N: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
        
        let response = input.trim().to_lowercase();

        match response.as_str(){
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => eprintln!("Please enter 'y' or 'n'.")
        }
    }
}

fn main() {
    let args: Com = Com::parse();

    // if path doesn't exist, ask to create and create it if yes
    if !args.path.exists() {
        println!("Path {:?} does not exist, would you like to create it?", &args.path);
        if confirmation() {
            match fs::create_dir_all(&args.path) {  
                Ok(_) => println!("Directory created"),
                Err(e) => eprintln!("Failed to create directory ({})", e)
            }
        }
        return;
    }

    if args.rmall {
        println!("Everything in the directory will be removed. Is this okay?");

        if confirmation(){
            match fs::remove_dir_all(&args.path){
                Ok(_) => {println!("Directory has been removed")}
                Err(e) => {
                    eprintln!("Directory ({}) failed to remove! Might not be a directory!", e);
                    return;
                }
            }
            match fs::create_dir_all(&args.path){
                Ok(_) => {println!("Directory recreated")},
                Err(e) => {
                    eprintln!("Failed to recreate directory ({})", e);
                    return;
                }
            }
        } else {
            return;
        }
    }

    for dir in &args.new_paths {
        let mut new_dir = PathBuf::from(&args.path);
        new_dir.push(dir);
        match fs::create_dir(&new_dir){
            Ok(_) => println!("New directory: {}", &new_dir.display()),
            Err(e) => eprintln!("Could not create new directory with path '{}' ({})", &new_dir.display(), e)
        }

    }
}
