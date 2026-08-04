#[derive(Debug, Clone)]
pub struct TransactionContext {
    pub plugin: String,
}

impl TransactionContext {
    pub fn new(plugin: impl Into<String>) -> Self {
        Self {
            plugin: plugin.into(),
        }
    }
}
