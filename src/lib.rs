//! Macros for conditionally including unstable features
//!
//! This crate contains a number of macros for reducing the boilerplate needed
//! to use various nightly Rust features if compiling on nightly.
//!
//! Note that all these macros require Cargo to be passed the feature flag
//! "unstable" if compiling on nightly.
//  TODO: allow support for different unstable feature flags?

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
