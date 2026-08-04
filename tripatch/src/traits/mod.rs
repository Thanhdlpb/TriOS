pub trait PatchPlugin {
    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn description(&self) -> &str;

    fn apply(&self) -> Result<(), String>;

    fn verify(&self) -> Result<(), String>;

    fn doctor(&self) -> Result<(), String>;

    fn rollback(&self) -> Result<(), String>;
}
