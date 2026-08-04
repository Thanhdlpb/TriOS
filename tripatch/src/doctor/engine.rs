use crate::doctor::{
    checks::{
        ConfigurationCheck, FilesystemCheck, HassDiscoveryCheck, PermissionCheck, PluginCheck,
    },
    DoctorCheck, DoctorReport,
};

pub struct DoctorEngine {
    checks: Vec<Box<dyn DoctorCheck>>,
}

impl DoctorEngine {
    pub fn new() -> Self {
        let mut engine = Self { checks: Vec::new() };

        engine.register(FilesystemCheck);
        engine.register(HassDiscoveryCheck);
        engine.register(ConfigurationCheck);
        engine.register(PermissionCheck);
        engine.register(PluginCheck);

        engine
    }

    pub fn register<C>(&mut self, check: C)
    where
        C: DoctorCheck + 'static,
    {
        self.checks.push(Box::new(check));
    }

    pub fn run(&self) -> DoctorReport {
        let mut report = DoctorReport::new();

        for check in &self.checks {
            report.push(check.run());
        }

        report
    }
}
