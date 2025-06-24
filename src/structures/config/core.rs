use regex::Regex;
use super::core_macros::*;

/// types

/// constants

/// lazy statics
lazy_static::lazy_static!(
    pub static ref ID: Regex = config_regex!(r"^(.*)\[(.*)\]$");
);
