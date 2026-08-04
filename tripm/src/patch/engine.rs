use crate::patch::common::*;
use crate::patch::registry::registry;

pub fn check_all() {
    for p in registry() {
        println!();
        println!("========== {} ==========", p.name());

        match p.check() {
            Ok(_) => ok("PASS"),

            Err(e) => fail(&e),
        }
    }
}

pub fn apply_all() {
    for p in registry() {
        println!();
        println!("========== {} ==========", p.name());

        match p.apply() {
            Ok(_) => ok("Installed"),

            Err(e) => fail(&e),
        }
    }
}

pub fn rollback_all() {
    for p in registry() {
        println!();
        println!("========== {} ==========", p.name());

        match p.rollback() {
            Ok(_) => ok("Rollback OK"),

            Err(e) => fail(&e),
        }
    }
}

pub fn status() {
    println!();

    println!("========== TriPatch ==========");

    for p in registry() {
        println!("{:<15} {}", p.name(), p.status());
    }
}
