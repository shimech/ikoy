use crate::event_loop::{EventLoop, executable::ExecuteState};

pub(super) fn v8_queue_microtask<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    _ret: v8::ReturnValue<'s>,
) {
    let callback: v8::Local<v8::Function> = args.get(0).try_into().unwrap();
    let callback = v8::Global::new(scope, callback);

    let event_loop = EventLoop::get_mut();
    event_loop.enqueue_microtask(Box::new(move |scope| {
        callback
            .open(scope)
            .call(scope, v8::undefined(scope).into(), &[]);
        ExecuteState::FULFILLED
    }));
}
