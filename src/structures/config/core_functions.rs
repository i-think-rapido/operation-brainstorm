

use regex::Replacer;
use serde::{Deserialize, Serialize};

use crate::structures::config::Config;

// crate
impl<C: Serialize + for<'de> Deserialize<'de>> Config<C> {
    pub fn save(&self) -> anyhow::Result<()> {

        let content = serde_json::to_string(self)?;

        std::fs::write(&self.filename, content)?;

        Ok(())
    }
    pub(crate) fn delete(&self) -> anyhow::Result<()> {
        std::fs::remove_file(&self.filename)?;
        Ok(())
    }
}

pub fn amend_id(string: &dyn AsRef<str>) -> String {
    let mut string = string.as_ref().to_owned();
    string.push_str("[_]");
    while string.ends_with("][_]") {
        string = string.replace("][_]", "]");
    }
    string
}

