use tracing::trace;

use crate::{
    core::{
        distance::{SimilarityScoreService, TotalDistanceService},
        Service,
    },
    handlers::Handler,
    input::{
        parsers::{distance::DistanceParser, CsvParser},
        reader::{CsvInputReader, InputReader},
    },
};

pub struct DistanceHandler;

impl Handler for DistanceHandler {
    fn process(input: String) -> anyhow::Result<()> {
        let input_reader = CsvInputReader;
        let records = input_reader.read_csv(input);
        trace!("Records Read: {:?}", records);

        let data = DistanceParser::parse_record(records?)?;
        trace!("Data Parsed: {:?}", data);

        let service = TotalDistanceService;
        let total_distance: i32 = service.calc(data.clone())?;
        println!("Total Distance: {}", total_distance);

        let service = SimilarityScoreService;
        let similarity_score: i32 = service.calc(data)?;
        println!("Similarity Score: {}", similarity_score);

        Ok(())
    }
}
