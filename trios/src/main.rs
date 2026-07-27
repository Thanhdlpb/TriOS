mod agents;
use agents::{web_agent::WebAgent, shell_agent::ShellAgent, iot_agent::IoTAgent, ai_agent::AIAgent};
use std::io::{self, Write};

fn main() {
    println!("🚀 TriOS Agent Runtime v2.0");
    let web = WebAgent::new();
    let shell = ShellAgent::new();
    let mut iot = IoTAgent::new();
    let mut ai = AIAgent::new();
    let mut connected = false;
    
    loop {
        print!("trios> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let cmd = input.trim();
        
        match cmd {
            "exit" | "quit" => break,
            "help" => println!("web <url> | tim <q> | shell <cmd> | iot <broker> | gui <t> <m> | ai train <m> <f> | ai pred <m> <v>"),
            c if c.starts_with("web ") => println!("{}", web.fetch_json(&c[4..]).unwrap_or_else(|e| e)),
            c if c.starts_with("tim ") => println!("{}", web.search(&c[4..]).unwrap_or_else(|e| e)),
            c if c.starts_with("shell ") => println!("{}", shell.execute(&c[6..]).unwrap_or_else(|e| e)),
            c if c.starts_with("iot ") => {
                if iot.connect(&c[4..], 1883, "trios").is_ok() { connected = true; println!("✅ Kết nối MQTT"); }
            }
            c if c.starts_with("gui ") => {
                let parts: Vec<&str> = c[4..].splitn(2, ' ').collect();
                if parts.len() == 2 && connected { iot.publish(parts[0], parts[1]).ok(); }
            }
            c if c.starts_with("ai train ") => {
                let parts: Vec<&str> = c[9..].splitn(2, ' ').collect();
                if parts.len() == 2 { println!("{}", ai.train(parts[0], parts[1]).map(|_| "✅".to_string()).unwrap_or_else(|e| e)); }
            }
            c if c.starts_with("ai pred ") => {
                let parts: Vec<&str> = c[9..].splitn(2, ' ').collect();
                if parts.len() == 2 { println!("{}", ai.predict(parts[0], parts[1]).unwrap_or_else(|e| e)); }
            }
            "" => continue,
            _ => println!("❌ Gõ 'help'"),
        }
    }
}
