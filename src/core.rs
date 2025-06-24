
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

pub(crate) use core_modules;