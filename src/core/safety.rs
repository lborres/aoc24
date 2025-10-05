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

            if is_safe(&report) {
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
        let mut safety_report: i32 = 0;

        for report in data.reports {
            trace!("Safety Record: {:?}", report);

            if is_safe(&report) {
                safety_report += 1;
                trace!("Report is Safe");
            } else if is_dampened_safe(&report) {
                safety_report += 1;
                trace!("Report is Safe");
            }
        }
        Ok(safety_report)
    }
}

fn is_safe(report: &[i32]) -> bool {
    let is_increasing: bool;

    if report[0] > report[1] {
        is_increasing = false;
    } else if report[0] < report[1] {
        is_increasing = true;
    } else {
        return false;
    }

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

fn is_dampened_safe(report: &[i32]) -> bool {
    if report.len() <= 2 {
        return false;
    }

    // try removing each index once
    for remove_idx in 0..report.len() {
        let mut reduced = Vec::with_capacity(report.len() - 1);
        reduced.extend_from_slice(&report[..remove_idx]);
        reduced.extend_from_slice(&report[remove_idx + 1..]);

        trace!("Trying removal index {} -> {:?}", remove_idx, reduced);
        if is_safe(&reduced) {
            trace!("Dampened Report Safe by removing index {}", remove_idx);
            return true;
        }
    }


    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // ref: https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust
    macro_rules! verify_is_safe {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let actual = is_safe(&input);
                    assert_eq!(expected, actual);
                }
            )*
        };
    }
    verify_is_safe! {
        is_safe_should_evaluate_correctly_00: ([7,6,4,2,1], true),
        is_safe_should_evaluate_correctly_01: ([1,2,7,8,9], false),
        is_safe_should_evaluate_correctly_02: ([9,7,6,2,1], false),
        is_safe_should_evaluate_correctly_03: ([1,3,2,4,5], false),
        is_safe_should_evaluate_correctly_04: ([8,6,4,4,1], false),
        is_safe_should_evaluate_correctly_05: ([1,3,6,7,9], true),
        is_safe_should_evaluate_correctly_06: ([1,5,3], false),
        is_safe_should_evaluate_correctly_07: ([1,2,3], true),
        is_safe_should_evaluate_correctly_08: ([1,1,2,3,4], false),
        is_safe_should_evaluate_correctly_09: ([90,91,93,96,93], false),
    }

    macro_rules! verify_is_dampened_safe {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let actual = is_dampened_safe(&input);
                    assert_eq!(expected, actual);
                }
            )*
        };
    }
    verify_is_dampened_safe! {
        is_dampened_safe_should_evaluate_correctly_00: ([7,6,4,2,1], true),
        is_dampened_safe_should_evaluate_correctly_01: ([1,2,7,8,9], false),
        is_dampened_safe_should_evaluate_correctly_02: ([9,7,6,2,1], false),
        is_dampened_safe_should_evaluate_correctly_03: ([1,3,2,4,5], true),
        is_dampened_safe_should_evaluate_correctly_04: ([8,6,4,4,1], true),
        is_dampened_safe_should_evaluate_correctly_05: ([1,3,6,7,9], true),
        is_dampened_safe_should_evaluate_correctly_06: ([1,5,3], true),
        is_dampened_safe_should_evaluate_correctly_07: ([1,2,3], true),
        is_dampened_safe_should_evaluate_correctly_08: ([1,1,2,3,4], true),
        is_dampened_safe_should_evaluate_correctly_09: ([90,91,93,96,93], true),
    }
}
