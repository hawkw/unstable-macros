//! Macros for conditionally including unstable features
//!
//! This crate contains a number of macros for reducing the boilerplate needed
//! to use various nightly Rust features if compiling on nightly.
//!
//! Note that all these macros require Cargo to be passed the feature flag
//! "unstable" if compiling on nightly.
//  TODO: allow support for different unstable feature flags?

/// Macro for making something a `const fn` if on unstable
///
/// If compiling on stable Rust, the function will be `#[inline]` instead.
#[macro_export]
macro_rules! unstable_const_fn {
    (   $(#[$attr:meta])*
        pub const fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ty:ty {
        $($body:expr)+
    }) => {
        #[cfg(features = "unstable")]
        $(#[$attr])*
        pub const fn $name($($arg: $arg_ty),*) -> $ty {
            $($body)+
        }
        #[cfg(not(features = "unstable"))]
        $(#[$attr])*
        #[inline]
        pub fn $name($($arg: $arg_ty),*) -> $ty {
            $($body)+
        }
    };
    (   $(#[$attr:meta])*
        const fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ty:ty {
        $($body:expr)+
    }) => {
        #[cfg(features = "unstable")]
        $(#[$attr])*
        const fn $name($($arg: $arg_ty),*) -> $ty {
            $($body)+
        }
        #[cfg(not(features = "unstable"))]
        $(#[$attr])*
        #[inline]
        fn $name($($arg: $arg_ty),*) -> $ty {
            $($body)+
        }
    };
}
