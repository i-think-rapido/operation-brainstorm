use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::config_enum::ConfigEnum;



#[derive(Serialize, Deserialize, Clone)]
pub struct Config<C> {
    inner: HashMap<String, ConfigEnum<C>>,
    #[serde(skip)]
    filename: String,
}


/// core modules
crate::core_modules!();

pub(crate) mod serde_impls;
