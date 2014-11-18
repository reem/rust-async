#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! A collection of important traits for asynchronous programming.

/// Anything that can be used as a callback.
pub trait Callback<T, O> {
    /// Consume the callback.
    fn invoke(self, T) -> O;
}

// All unboxed closures are callbacks.
impl<I, O, F: FnOnce(I) -> O> Callback<I, O> for F {
    fn invoke(self, val: I) -> O {
        self.call_once((val,))
    }
}

/// An asynchronous operation, executable with a callback.
pub trait Operation<T> {
    /// Execute this operation, feeding its result to the passed-in callback.
    fn execute<Co, C: Callback<T, Co>>(self, C);
}

// Actually implementing the above trait for any kind of
// fn or closure requires higher-ranked types, so no
// blanket implementations for now.

