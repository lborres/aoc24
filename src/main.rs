use clap::{Parser, Subcommand};
use std::process::ExitCode;
use tracing::trace;

use advent::{
    handlers::{distance::DistanceHandler, safety::SafetyHandler, Handler},
    tracer,
};

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
    /// Calculate distance between right and left lists (2024 Day 1)
    ///
    /// Primary Function Calculates the *Total Distance* between right and left unsorted lists.
    /// Where total distance is sum of differences between the sorted list values
    ///
    /// Secondary Function Calculates the *Similarity Score* between right and left unsorted lists.
    /// Where Similarity Score is the Sum between Similarities
    /// Where Similarities are the counts of the number of times an element in the left list
    /// occurs in the right list. Including duplicates in the left list.
    Dist {
        /// input csv file location
        #[arg(short, long, default_value = "./input/01.csv")]
        input: String,
    },
    /// Calculate the Safety Score given Safety Levels Report
    Safe {
        /// input csv file location
        #[arg(short, long, default_value = "./input/02.csv")]
        input: String,
    },
}

fn main() -> ExitCode {
    dotenvy::dotenv().ok();
    tracer::init();

    match run(Args::parse()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{:#}", err);
            ExitCode::from(2)
        }
    }
}

/// ref: [@BurntSushi/ripgrep/crates/core/main.rs](https://github.com/BurntSushi/ripgrep/blob/9802945e6342ec284633924cb7d8d3ce67204995/crates/core/main.rs#L77)
fn run(parsed_args: Args) -> anyhow::Result<ExitCode> {
    trace!("{:?}", parsed_args);

    match parsed_args.cmd {
        Some(Commands::Dist { input }) => {
            DistanceHandler::process(input)?;
        }
        Some(Commands::Safe { input }) => {
            SafetyHandler::process(input)?;
        }
        None => {
            eprintln!("No command provided. Use --help for usage.");
            return Ok(ExitCode::from(1));
        }
    };
    Ok(ExitCode::from(0))
}
