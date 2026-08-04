use super::result::{DoctorResult, Status};

pub struct DoctorReport {
    pub results: Vec<DoctorResult>,
}

impl DoctorReport {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn push(&mut self, r: DoctorResult) {
        self.results.push(r);
    }

    pub fn health_score(&self) -> u32 {
        if self.results.is_empty() {
            return 100;
        }

        let mut score: u32 = 100;

        for r in &self.results {
            match r.status {
                Status::Pass => {}

                Status::Warn => {
                    score = score.saturating_sub(10);
                }

                Status::Fail => {
                    score = score.saturating_sub(25);
                }
            }
        }

        score
    }
}
