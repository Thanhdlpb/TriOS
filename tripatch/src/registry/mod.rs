use crate::plugins::{scan_plugins, Plugin};

pub struct PluginRegistry {
    plugins: Vec<Plugin>,
}

impl PluginRegistry {
    pub fn new(root: &str) -> Self {
        Self {
            plugins: scan_plugins(root),
        }
    }

    pub fn list(&self) -> &[Plugin] {
        &self.plugins
    }

    pub fn find(&self, name: &str) -> Option<&Plugin> {
        self.plugins.iter().find(|p| p.manifest.name == name)
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}
