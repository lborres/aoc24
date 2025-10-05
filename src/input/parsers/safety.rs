use anyhow::Context;

use crate::input::{parsers::CsvParser, SafetyReportData};

pub struct SafetyParser;

impl CsvParser<SafetyReportData> for SafetyParser {
    fn parse_record(records: Vec<csv::StringRecord>) -> anyhow::Result<SafetyReportData> {
        let mut reports: Vec<Vec<i32>> = Vec::new();

        for record in records {
            let parsed_report: Vec<i32> = record
                .iter()
                .map(|val| val.parse::<i32>().context("Failed to parse integer"))
                .collect::<Result<Vec<i32>, _>>()?;

            reports.push(parsed_report)
        }

        Ok(SafetyReportData { reports })
    }
}
