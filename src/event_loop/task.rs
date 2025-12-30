use crate::event_loop::callback::Callback;

pub struct Task {
    callback: Callback,
}

impl Task {
    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }
}

pub struct Microtask {
    callback: Callback,
}

impl Microtask {
    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }
}
