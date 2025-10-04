use chrono::Local;
use std::fmt::Result;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

pub struct CustomTimer;

impl FormatTime for CustomTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> Result {
        let now = Local::now();
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}
