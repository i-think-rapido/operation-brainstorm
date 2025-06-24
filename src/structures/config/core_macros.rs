
#[macro_export]
macro_rules! config_regex {
    ($expr: expr) => {
        Regex::new($expr).expect("can't create regex")
    };
}

pub use config_regex;
