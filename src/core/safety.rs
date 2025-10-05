use tracing::trace;

use crate::{core::Service, input::SafetyReportData};

pub struct RawSafetyReportService;
pub struct DampenedSafetyReportService;

impl Service for RawSafetyReportService {
    type Input = SafetyReportData;
    type Output = i32;

    fn calc(&self, data: Self::Input) -> anyhow::Result<Self::Output> {
        let mut safety_report: i32 = 0;

        for report in data.reports {
            trace!("Safety Record: {:?}", report);
            let is_safe: bool;

            if report[0] > report[1] {
                let is_increasing = false;
                is_safe = check_safety(&report, is_increasing);
            } else if report[0] < report[1] {
                let is_increasing = true;
                is_safe = check_safety(&report, is_increasing);
            } else {
                continue;
            }

            if is_safe {
                safety_report += 1;
                trace!("Report is Safe");
            }
        }
        Ok(safety_report)
    }
}

impl Service for DampenedSafetyReportService {
    type Input = SafetyReportData;
    type Output = i32;

    fn calc(&self, data: Self::Input) -> anyhow::Result<Self::Output> {
        Ok(0)
    }
}

fn check_safety(report: &[i32], is_increasing: bool) -> bool {
    for level in report.windows(2) {
        trace!("Current Level Pair: {:?}", level);
        let diff = if is_increasing {
            level[1] - level[0]
        } else {
            level[0] - level[1]
        };

        trace!("Diff: {}", diff);
        if diff > 3 || diff <= 0 {
            trace!("Unsafe Level found");
            return false;
        }
    }
    true
}
