use super::step::TransactionStep;

pub struct TransactionLogger;

impl TransactionLogger {
    pub fn log(step: TransactionStep, message: &str) {
        println!("[{:?}] {}", step, message);
    }
}
