
#[macro_export]
macro_rules! core_modules {
    () => {
        pub(crate) mod core_macros;
/// core modules
        pub(crate) mod core;
        pub(crate) mod core_functions;

/// impls
        pub(crate) mod core_constructors;
        pub(crate) mod core_impls;

/// tests
        #[cfg(test)]
        pub(crate) mod core_tests;

    }
}

#[macro_export]
macro_rules! trait_modules {
    ($($r#mod:tt)*) => {
        $(
            pub mod $r#mod;
            pub use $r#mod::*;
        )?
    };
}

// macro_rules! deref_0 {
//     (impl<($list: literal)> ($struct: ty) target ($target: ty)) => {
// /// deref
//         impl<$list> Deref for $struct {
//         type Target = $target;
//         fn deref(&self) -> &Self::Target {
//                 &self.0
//         }
//         }
//         impl DerefMut for $struct {
//         fn deref_mut(&mut self) -> &mut Self::Target {
//                 &mut self.0
//         }
//         }
//     };
//     (($struct: ty) target ($target: ty)) => {
// /// deref
//         impl Deref for $struct {
//         type Target = $target;
//         fn deref(&self) -> &Self::Target {
//                 &self.0
//         }
//         }
//         impl DerefMut for $struct {
//         fn deref_mut(&mut self) -> &mut Self::Target {
//                 &mut self.0
//         }
//         }
//     };
// }
// macro_rules! deref_inner {
//         (
//             impl$($(type_parameters:tt)+)? $type:ty where target is $target_type:ty
//         ) => {
// /// deref
//                 impl $type_parameters ::core::deref::Deref for $type {
//                         type Target = $target_type;
//                         fn deref(&self) -> &Self::Target {
//                                 &self.inner
//                         }
//                 }
//                 impl $type_parameters ::core::deref::DerefMut for $type {
//                         fn deref_mut(&mut self) -> &mut Self::Target {
//                                 &mut self.inner
//                         }
//                 }
//         };
// }


pub(crate) use core_modules;
pub(crate) use trait_modules;
// pub(crate) use deref_0;
// pub(crate) use deref_inner;
