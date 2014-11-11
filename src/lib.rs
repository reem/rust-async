#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! A collection of important traits for asynchronous programming.

/// Anything that can be used as a callback.
pub trait Callback<T, O> {
    /// Consume the callback.
    fn invoke(self, T) -> O;
}

