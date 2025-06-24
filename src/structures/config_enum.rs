use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigEnum<C> {
    None,
    Singleton(C),
    Vec(Vec<C>),
    Len(usize),
    HashMap(HashMap<String, C>),
}

