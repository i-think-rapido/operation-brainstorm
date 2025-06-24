use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::structures::{config::Config, config_enum::ConfigEnum};

#[test]
fn test_create_delete() -> anyhow::Result<()> {

    let filename = "./.dump/bla.json";

    let id = "bla";

    #[derive(Default, Serialize, Deserialize, Clone)]
    struct Settings {
        pub a: u8,
    }

    let mut config = Config::<Settings>::new(&filename)?;
    if config.get(&id).is_none() {
        let mut settings = Settings::default();
        settings.a = 222;
        config.set(&"bla", settings)?;
    }
    if let ConfigEnum::Singleton(settings) = config.get(&id).context("context")? {
        assert_eq!(settings.a, 222);
    }
    drop(config);

    let config = Config::<Settings>::new(&filename)?;
    if let ConfigEnum::Singleton(settings) = config.get(&id).context("context")? {
        assert_eq!(settings.a, 222);
    }
    config.delete()?;

    Ok(())
}
