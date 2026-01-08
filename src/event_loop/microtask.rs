use crate::event_loop::executable::{CallbackOnce, Executable};

pub(super) struct Microtask {
    callback: CallbackOnce,
}

impl Microtask {
    pub(super) fn new(callback: CallbackOnce) -> Self {
        Self { callback }
    }
}

impl Executable for Microtask {
    type NextExecutable = Self;

    fn execute<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) -> Option<Self> {
        (self.callback)(scope);
        // Microtasks are executed once and do not need to be rescheduled.
        None
    }
}
