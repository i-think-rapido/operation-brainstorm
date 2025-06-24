use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::structures::config::Config;


impl<C: Serialize + for<'de> Deserialize<'de>> Config<C> {
    pub fn new(filename: &dyn AsRef<str>) -> anyhow::Result<Self> {

        if let Ok(content) = std::fs::read_to_string(filename.as_ref()) {
            let content = content.clone();
            let mut out: Self = serde_json::from_str(content.as_str())?;
            out.filename = filename.as_ref().to_owned();
            return Ok(out)
        }

        let out = Self {
            inner: HashMap::default(),
            filename: filename.as_ref().to_owned(),
        };
        out.save()?;

        Ok(out)
    }
}

