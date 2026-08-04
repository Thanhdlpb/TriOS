use std::fs;
use std::path::Path;

const CONFIG:&str="/root/.homeassistant/configuration.yaml";

pub fn load()->String{
    fs::read_to_string(CONFIG).unwrap_or_default()
}

pub fn save(text:&str){
    fs::write(CONFIG,text).unwrap();
}

pub fn exists()->bool{
    Path::new(CONFIG).exists()
}

pub fn contains(key:&str)->bool{
    load().contains(key)
}

pub fn append(line:&str){

    let mut cfg=load();

    if !cfg.contains(line){

        cfg.push('\n');
        cfg.push_str(line);
        cfg.push('\n');

        save(&cfg);

    }

}

pub fn remove(line:&str){

    let cfg=load();

    let new_cfg=cfg
        .lines()
        .filter(|l|l.trim()!=line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    save(&new_cfg);

}
