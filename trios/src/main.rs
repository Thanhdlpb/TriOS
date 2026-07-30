mod agent_system;
use agent_system::AgentRuntime;
use std::io::{self, Write};

fn main() {
    println!("🚀 TriOS Multi-Agent Runtime v2.0");
    let mut runtime = AgentRuntime::new();
    runtime.init();

    loop {
        print!("trios> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() { break; }
        let cmd = input.trim();

        match cmd {
            "exit" | "quit" => break,
            "help" => println!("Lệnh: shell <cmd> | web <url> | ai <câu> | agents | exit"),
            c if c.starts_with("shell ") => println!("{}", runtime.run_agent("shell", &c[6..])),
            c if c.starts_with("web ") => println!("{}", runtime.run_agent("web", &c[4..])),
            c if c.starts_with("ai ") => println!("{}", runtime.run_agent("ai", &c[3..])),
            "agents" => println!("{:?}", runtime.list_agents()),
            "" => continue,
            _ => println!("❌ Gõ 'help'"),
        }
    }
}
