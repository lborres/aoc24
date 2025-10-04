use clap::{Parser, Subcommand};
use std::process::ExitCode;
use tracing::trace;

#[derive(Parser, Debug)]
#[command(
    version,
    author = "Louis Borres",
    about = "Provides Advent of Code 2024 Answers given a CSV file input"
)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Dist {
        #[arg(short, long, default_value = "./input/01.csv")]
        input: String,
    },
}

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    aoc24::tracer::init();

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
    trace!("{:?}", parse_args_result);

    let _args = match parse_args_result.cmd {
        Some(Commands::Dist { input }) => {
            trace!("{:?}", input);
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
            return Ok(ExitCode::from(1));
        }
    };
    Ok(ExitCode::from(0))
}
