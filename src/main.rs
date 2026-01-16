mod args;
mod error;
mod format;
mod report;
mod png;
mod jpeg;
mod dng;

use args::{Args, Command};
use clap::Parser;
use std::fs;

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Inspect { path } | Command::Metadata { path } => {
            let data = match fs::read(&path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(1);
                }
            };

            match format::inspect(&data) {
                Ok(report) => println!("{}", report),
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
