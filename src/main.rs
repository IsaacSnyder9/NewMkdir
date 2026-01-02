#!/usr/bin/env bash
use clap::Parser;
use std::fs::{self};
use std::path::{PathBuf};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Com {
    /// takes a specific path, if none is provided then it will be in current directory
    #[arg(short, long, value_parser = clap::value_parser!(PathBuf))]
    path: Option<PathBuf>,

    /// All paths you would like to add as a child of the current or provided path
    #[arg(num_args = 1..)]
    new_paths: Vec<String>,

    /// Removes all files and directories within the current or provided path before setting any new ones
    #[arg(short, long)]
    rmall: bool,

    /// How many of each file will be made, will add an _ and an int at the end
    #[arg(short, long, default_value_t = 1)]
    count: u8,
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
            // if path doesn't exist, ask to create and create it if yes
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => eprintln!("Please enter 'y' or 'n'.")
        }
    }
}

fn main() {
    let args: Com = Com::parse();
    let working_path = args.path.unwrap_or_else(|| {
            std::env::current_dir().expect("Could not get current directory")
    });

    if !working_path.exists(){
    println!("Path {:?} does not exist, would you like to create it?", &working_path);
        if confirmation() {
            match fs::create_dir_all(&working_path) {  
                Ok(_) => println!("Directory created {:?}", &working_path),
                Err(e) => eprintln!("Failed to create directory ({})", e)
            }
        }
        return;
    }

    if args.rmall {
        let current_path = std::env::current_dir().expect("Could not get current directory");
        if working_path == current_path {
            println!("WARNING CURRENT DIRECTORY WILL BE DELETED, IS THIS OKAY?")
        } else {
            println!("Everything in the directory will be removed. Is this okay?");
        }
        
        if confirmation(){
            match fs::remove_dir_all(&working_path){
                Ok(_) => {
                    println!("Directory has been removed");
                    if working_path == current_path {
                        println!("Terminal may need to be refreshed, type: cd {:?}", &current_path)
                    }
                }
                Err(e) => {
                    eprintln!("Directory ({}) failed to remove! Might not be a directory!", e);
                    return;
                }
            }
            match fs::create_dir_all(&working_path){
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
        for i in 1..=args.count { 
            let mut new_dir = PathBuf::from(&working_path);
            if i > 1 {
                new_dir.push(format!("{}_{}",dir, &i))
            } else {
                new_dir.push(dir.to_string())
            };
            match fs::create_dir(&new_dir){
                Ok(_) => println!("New directory: {}", &new_dir.display()),
                Err(e) => eprintln!("Could not create new directory with path '{}' ({})", &new_dir.display(), e)
            }
        }
    }
}
