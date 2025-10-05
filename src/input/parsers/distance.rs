use anyhow::Context;
use csv::StringRecord;

use crate::input::{parsers::CsvParser, DistanceData};

pub struct DistanceParser;

impl CsvParser<DistanceData> for DistanceParser {
    fn parse_record(records: Vec<StringRecord>) -> anyhow::Result<DistanceData> {
        let mut list_a: Vec<i32> = Vec::new();
        let mut list_b: Vec<i32> = Vec::new();

        for record in records {
            let a = record
                .get(0)
                .context("Missing value in first column")?
                .parse::<i32>()
                .context("Failed to parse first column as i32")?;

            let b = record
                .get(1)
                .context("Missing value in second column")?
                .parse::<i32>()
                .context("Failed to parse second column as i32")?;

            list_a.push(a);
            list_b.push(b);
        }

        list_a.sort();
        list_b.sort();

        Ok(DistanceData { list_a, list_b })
    }
}
