use crate::event_loop::{callback::CallbackOnce, task_result::TaskResult};

pub struct Microtask {
    callback: CallbackOnce,
}

impl Microtask {
    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) -> TaskResult {
        (self.callback)(scope)
    }
}
