//! Control flow for callbacks.

macro_rules! try_control {
    ($e:expr) => {
        let x = $e;
        if x.is_break() {
            return x;
        }
    };
}

/// Control flow for callbacks.
///
/// The empty return value `()` is equivalent to continue.
#[allow(clippy::module_name_repetitions)]
pub trait ControlFlow {
    #[inline]
    fn is_break(&self) -> bool {
        false
    }
}

impl ControlFlow for () {}

impl<B> ControlFlow for std::ops::ControlFlow<B, ()> {
    fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }
}

impl<E> ControlFlow for Result<(), E> {
    fn is_break(&self) -> bool {
        self.is_err()
    }
}
