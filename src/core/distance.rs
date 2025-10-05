use crate::{core::Service, input::DistanceData};

pub struct TotalDistanceService;
pub struct SimilarityScoreService;

impl Service for TotalDistanceService {
    type Input = DistanceData;
    type Output = i32;

    fn calc(&self, data: Self::Input) -> anyhow::Result<Self::Output> {
        let total_distance: i32 = data
            .list_a
            .iter()
            .zip(data.list_b.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        Ok(total_distance)
    }
}

impl Service for SimilarityScoreService {
    type Input = DistanceData;
    type Output = i32;

    fn calc(&self, data: Self::Input) -> anyhow::Result<Self::Output> {
        let mut iter_b = data.list_b.iter().peekable();
        let mut similarity: i32 = 0;

        let mut traverse_b = |index: usize, data_a: &i32| -> i32 {
            let prev_a = if index > 0 {
                Some(data.list_a[index - 1]).unwrap()
            } else {
                0
            };

            if prev_a == *data_a {
                return data_a * similarity;
            } else {
                similarity = 0
            }

            while let Some(data_b) = iter_b.peek() {
                if data_a == *data_b {
                    similarity += 1;
                }

                if *data_b <= data_a {
                    iter_b.next();
                } else {
                    break;
                }
            }

            data_a * similarity
        };

        let similarity_score = data
            .list_a
            .iter()
            .enumerate()
            .map(|(index, val)| traverse_b(index, val))
            .sum();

        Ok(similarity_score)
    }
}
