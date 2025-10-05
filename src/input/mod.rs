pub mod parsers; // Parses the Input into usable data structures
pub mod reader; // Reads the Input itself

#[derive(Debug, Clone)]
pub struct DistanceData {
    pub list_a: Vec<i32>,
    pub list_b: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct SafetyReportData {
    pub reports: Vec<Vec<i32>>,
}
