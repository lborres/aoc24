use csv::StringRecord;


pub mod distance;
pub mod safety;

pub trait CsvParser<T> {
    fn parse_record(records: Vec<StringRecord>) -> anyhow::Result<T>;
}
