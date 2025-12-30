use crate::event_loop::callback::CallbackOnce;

pub struct Task {
    callback: CallbackOnce,
}

impl Task {
    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }
}

pub struct Microtask {
    callback: CallbackOnce,
}

impl Microtask {
    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }
}
