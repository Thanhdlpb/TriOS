use triai::runtime::{AIRuntime, SimpleLinearModel};
use std::fs;

pub struct AIAgent {
    runtime: AIRuntime,
}

impl AIAgent {
    pub fn new() -> Self {
        let mut rt = AIRuntime::new();
        rt.register_model(Box::new(SimpleLinearModel::new("mặc_định")));
        Self { runtime: rt }
    }
    
    pub fn train(&mut self, model: &str, data: &str) -> Result<(), String> {
        let content = fs::read_to_string(data).map_err(|e| e.to_string())?;
        self.runtime.train(model, &content)
    }
    
    pub fn predict(&self, model: &str, input: &str) -> Result<String, String> {
        self.runtime.predict(model, input)
    }
}
