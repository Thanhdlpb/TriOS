use std::process::Command;

pub struct ShellAgent;

impl ShellAgent {
    pub fn new() -> Self { Self }
    
    pub fn execute(&self, cmd: &str) -> Result<String, String> {
        let output = if cfg!(windows) {
            Command::new("cmd").args(["/C", cmd]).output()
        } else {
            Command::new("sh").args(["-c", cmd]).output()
        };
        output.map(|o| {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            if !out.is_empty() { out.to_string() } else { err.to_string() }
        }).map_err(|e| format!("Shell: {}", e))
    }
}
