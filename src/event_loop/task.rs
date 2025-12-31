use crate::event_loop::{callback::Callback, task_result::TaskResult};

pub struct Task {
    callback: Callback,
}

impl Task {
    pub(crate) fn new(callback: Callback) -> Self {
        Self { callback }
    }

    pub(crate) fn run<'s>(&mut self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) -> TaskResult {
        (self.callback)(scope)
    }
}
