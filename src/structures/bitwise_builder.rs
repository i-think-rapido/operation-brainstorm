use getset::Getters;

pub type BitwiseBuilderU8  = BitwiseBuilder<u8>;
pub type BitwiseBuilderU16 = BitwiseBuilder<u16>;
pub type BitwiseBuilderU32 = BitwiseBuilder<u32>;
pub type BitwiseBuilderU64 = BitwiseBuilder<u64>;

#[derive(Debug, Default, Clone, Getters)]
pub struct BitwiseBuilder<T> {
    mask: T,
    #[getset(get)]
    pub start_bit: u8,
    #[getset(get)]
    pub bits: u8,
}

/// core modules
crate::core_modules!();

/// trait modules
crate::trait_modules!(handler);

