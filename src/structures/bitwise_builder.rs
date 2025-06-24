use getset::Getters;



#[derive(Debug, Clone, Getters)]
pub struct BitwiseBuilder<T> {
    #[getset(get)]
    pub start_bit: u8,
    #[getset(get)]
    pub bits: u8,
    mask: T,
}

/// core modules
crate::core_modules!();
