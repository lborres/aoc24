use clap::{Parser, Subcommand};

use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    author = "Louis Borres",
    version,
    about = "Provides Advent of Code 2024 Answers given a CSV file input"
)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Dist,
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{:#}", err);
            ExitCode::from(2)
        }
    }
}

// ref: https://github.com/BurntSushi/ripgrep/blob/9802945e6342ec284633924cb7d8d3ce67204995/crates/core/main.rs#L77
fn run(parse_args_result: Args) -> anyhow::Result<ExitCode> {
    println!("{:?}", parse_args_result);
    let _args = match parse_args_result.cmd {
        Some(Commands::Dist) => {
            println!("{:?}", Commands::Dist)
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
            return Ok(ExitCode::from(1));
        }
    };
    Ok(ExitCode::from(0))
}
