use std::sync::Arc;

use guano_common::regex::Regex;
use guano_common::sync::Map;

#[::static_init::dynamic]
pub(crate) static REGEX_REGISTRY: RegexRegistry = RegexRegistry::default();

#[derive(Debug, Clone, Default)]
pub(crate) struct RegexRegistry {
    registry: Arc<Map<&'static str, Arc<Regex>>>,
}

impl RegexRegistry {
    pub fn get(&self, re: &'static str) -> Option<Arc<Regex>> {
        if let Some(regex) = self.registry.get(re) {
            Some(regex.clone())
        } else {
            let regex = Regex::new(re).ok()?;
            let regex = Arc::new(regex);
            self.registry.insert(re, regex.clone());

            Some(regex)
        }
    }
}
