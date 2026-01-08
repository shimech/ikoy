use crate::event_loop::executable::{Callback, Executable, ExecuteState};

pub struct Task {
    callback: Callback,
}

impl Task {
    pub(crate) fn new(callback: Callback) -> Self {
        Self { callback }
    }
}

impl Executable for Task {
    type NextExecutable = Self;

    fn execute<'s>(mut self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) -> Option<Self> {
        match (self.callback)(scope) {
            ExecuteState::PENDING => Some(self),
            _ => None,
        }
    }
}
