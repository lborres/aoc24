use std::fs::File;
use std::path::Path;

pub trait InputReader {
    fn read_csv<P: AsRef<Path>>(&self, file_path: P) -> anyhow::Result<Vec<csv::StringRecord>>;
}

pub struct CsvInputReader;

impl InputReader for CsvInputReader {
    fn read_csv<P: AsRef<Path>>(&self, file_path: P) -> anyhow::Result<Vec<csv::StringRecord>> {
        let file = File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true) // Allow varying number of fields
            .from_reader(file);
        let records: anyhow::Result<Vec<csv::StringRecord>, csv::Error> =
            reader.records().collect();
        records.map_err(|e| e.into())
    }
}
