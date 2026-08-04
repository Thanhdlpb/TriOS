use crate::traits::PatchPlugin;

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&self, plugin: &dyn PatchPlugin) -> Result<(), String> {
        plugin.apply()
    }

    pub fn verify(&self, plugin: &dyn PatchPlugin) -> Result<(), String> {
        plugin.verify()
    }

    pub fn doctor(&self, plugin: &dyn PatchPlugin) -> Result<(), String> {
        plugin.doctor()
    }

    pub fn rollback(&self, plugin: &dyn PatchPlugin) -> Result<(), String> {
        plugin.rollback()
    }

    pub fn run_all_checks(&self, plugin: &dyn PatchPlugin) -> Result<(), String> {
        plugin.verify()?;
        plugin.doctor()?;
        Ok(())
    }
}
