use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::structures::{config::{core::ID, core_functions::amend_id, Config}, config_enum::ConfigEnum};

impl<C: Default + Serialize + for<'de> Deserialize<'de>> Config<C> {
    pub fn get(&self, id: &dyn AsRef<str>) -> Option<&ConfigEnum<C>> {

        let id = amend_id(&id);
        let captures = ID.captures(id.as_ref())?;

        let key = captures.get(1)?.as_str();
        let _pattern = captures.get(2)?.as_str();

        self.inner.get(key)
    }
    pub fn set(&mut self, id: &dyn AsRef<str>, value: C) -> anyhow::Result<()> {
        let id = amend_id(&id);
        let captures = ID.captures(id.as_ref()).context("can't retrieve captures")?;

        let key = captures.get(1).context("can't get first pattern match")?.as_str();
        let pattern = captures.get(2).context("can't get second pattern match")?.as_str();

        if pattern == "_" {
            self.inner.insert(key.to_owned(), ConfigEnum::Singleton(value));
        }

        else if pattern == "#" {
            unreachable!("can't query length");
        }

        else if let Ok(len) = pattern.parse::<usize>() {
            if let Some(ConfigEnum::Vec(vec)) = self.inner.get_mut(&id) {
                if len == vec.len() {
                    vec.push(value);
                }
            }
        }

        else if let Some(ConfigEnum::HashMap(map)) = self.inner.get_mut(&id) {
            map.insert(pattern.to_owned(), value);
        }

        else {
            unreachable!("can't set value")
        }
        self.save()?;

        Ok(())
    }
}
