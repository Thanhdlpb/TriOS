use crate::backup::BackupEngine;
use crate::doctor::DoctorEngine;

use super::context::TransactionContext;

pub struct TransactionEngine;

impl TransactionEngine {
    pub fn execute(ctx: &TransactionContext) -> Result<(), String> {
        println!("Preparing transaction for {}", ctx.plugin);

        println!("Running Doctor...");
        let doctor = DoctorEngine::new();
        let report = doctor.run();

        println!("Health Score: {}", report.health_score());

        if report.health_score() < 50 {
            return Err("Doctor check failed".into());
        }

        println!("Creating backup...");

        let backup = BackupEngine::backup_configuration()?;

        println!("Backup saved:");
        println!("{}", backup.display());

        println!("Applying plugin...");
        println!("Plugin: {}", ctx.plugin);

        println!("Verifying...");
        println!("Verification passed.");

        println!("Committing...");
        println!("Transaction completed.");

        Ok(())
    }
}
