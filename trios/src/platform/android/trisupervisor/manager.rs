use super::service::Service;
use std::collections::HashMap;

pub struct Supervisor{
    pub services:HashMap<String,Service>,
}

impl Supervisor{

    pub fn new()->Self{
        Self{
            services:HashMap::new(),
        }
    }

    pub fn register(&mut self,svc:Service){
        self.services.insert(svc.name.clone(),svc);
    }

    pub fn list(&self){
        println!("Registered services:");
        for s in self.services.keys(){
            println!(" - {}",s);
        }
    }

}
