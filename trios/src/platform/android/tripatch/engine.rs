use super::rules::*;

pub fn apply() {

    println!();
    println!("========== TriPatch ==========");

    for rule in rules() {

        println!(
            "[PATCH] {:<12} {}",
            rule.name,
            rule.description
        );

    }

    println!("==============================");
}
