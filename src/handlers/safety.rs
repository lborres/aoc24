use tracing::trace;

use crate::{
    core::{safety::{DampenedSafetyReportService, RawSafetyReportService}, Service},
    handlers::Handler,
    input::{
        parsers::{safety::SafetyParser, CsvParser},
        reader::{CsvInputReader, InputReader},
    },
};

pub struct SafetyHandler;

impl Handler for SafetyHandler {
    fn process(input: String) -> anyhow::Result<()> {
        let input_reader = CsvInputReader;
        let records = input_reader.read_csv(input);
        trace!("Records Read: {:?}", records);

        let data = SafetyParser::parse_record(records?)?;
        trace!("Data Parsed: {:?}", data);

        let service = RawSafetyReportService;
        let raw_safety_report = service.calc(data.clone())?;
        println!("Raw Safety Report: {}", raw_safety_report);

        let service = DampenedSafetyReportService;
        let raw_safety_report = service.calc(data)?;
        println!("Dampened Safety Report: {}", raw_safety_report);

        Ok(())
    }
}
