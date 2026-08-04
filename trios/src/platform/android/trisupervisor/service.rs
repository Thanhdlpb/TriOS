#[derive(Clone)]
pub struct Service {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

impl Service {
    pub fn new(name:&str,command:&str,args:Vec<String>)->Self{
        Self{
            name:name.into(),
            command:command.into(),
            args,
        }
    }
}
