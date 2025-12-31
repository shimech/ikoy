use crate::event_loop::task_result::TaskResult;

pub type CallbackOnce =
    Box<dyn for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>) -> TaskResult>;

pub type Callback = Box<dyn for<'s> FnMut(&mut v8::PinnedRef<'s, v8::HandleScope>) -> TaskResult>;
