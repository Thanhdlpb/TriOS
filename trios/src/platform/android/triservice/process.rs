use std::process::{Command, Child};

pub fn spawn(cmd:&str,args:&[&str])->Option<Child>{
    Command::new(cmd)
        .args(args)
        .spawn()
        .ok()
}
