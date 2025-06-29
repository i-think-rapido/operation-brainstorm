use crate::structures::bitwise_builder::BitwiseBuilder;



pub struct Bitwise<'t, 'b, T>(&'t T, &'b BitwiseBuilder<T>);



/// core modules
crate::core_modules!();

/// trait modules
crate::trait_modules!(ops);