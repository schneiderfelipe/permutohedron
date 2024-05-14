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
    fn continuing() -> Self;
    #[inline]
    fn should_break(&self) -> bool {
        false
    }
}

impl ControlFlow for () {
    fn continuing() {}
}

impl<B> ControlFlow for std::ops::ControlFlow<B, ()> {
    fn continuing() -> Self {
        Self::Continue(())
    }
    fn should_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }
}

impl<E> ControlFlow for Result<(), E> {
    fn continuing() -> Self {
        Ok(())
    }
    fn should_break(&self) -> bool {
        self.is_err()
    }
}
