use std::process::Command;

pub fn start(){

    println!("Starting Home Assistant...");

    let _=Command::new("bash")
        .arg("-lc")
        .arg("source ~/homeassistant/bin/activate && hass")
        .status();

}
