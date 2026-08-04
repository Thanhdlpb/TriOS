use crate::{
    doctor::{DoctorCheck, DoctorResult},
    registry::PluginRegistry,
};

pub struct PluginCheck;

impl DoctorCheck for PluginCheck {
    fn name(&self) -> &'static str {
        "plugin"
    }

    fn run(&self) -> DoctorResult {
        let registry = PluginRegistry::new("tripatch/plugins");

        if registry.is_empty() {
            return DoctorResult::warn("Plugin", "Không tìm thấy plugin nào");
        }

        let count = registry.count();

        DoctorResult::pass("Plugin", &format!("Đã nạp {} plugin", count))
    }
}
