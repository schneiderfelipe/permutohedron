//! Control flow for callbacks.

macro_rules! try_control {
    ($e:expr) => {
        match $e {
            x => {
                if x.should_break() {
                    return x;
                }
            }
        }
    };
}

/// Control flow for callbacks.
///
/// The empty return value `()` is equivalent to continue.
#[allow(clippy::module_name_repetitions)]
pub trait ControlFlow {
    #[inline]
    fn should_break(&self) -> bool {
        false
    }
}

impl ControlFlow for () {}

impl<B> ControlFlow for std::ops::ControlFlow<B, ()> {
    fn should_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }
}

impl<E> ControlFlow for Result<(), E> {
    fn should_break(&self) -> bool {
        self.is_err()
    }
}
