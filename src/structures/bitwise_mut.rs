use crate::structures::bitwise_builder::BitwiseBuilder;



pub struct BitwiseMut<'t, 'b, T>(&'t mut T, &'b BitwiseBuilder<T>);




/// core modules
crate::core_modules!();
