#[derive(Debug, Clone)]
pub enum TransactionStep {
    Doctor,
    Backup,
    Apply,
    Verify,
    Rollback,
    Commit,
}
