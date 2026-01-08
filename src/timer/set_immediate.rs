use crate::event_loop::{EventLoop, executable::ExecuteState};

/// Implementation of [setImmediate](https://nodejs.org/docs/v24.12.0/api/timers.html#setimmediatecallback-args)
pub(super) fn v8_set_immediate<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut ret: v8::ReturnValue<'s>,
) {
    let function_ref: v8::Local<v8::Function> = args.get(0).try_into().unwrap();
    let params: Vec<v8::Local<v8::Value>> = (2..args.length()).map(|i| args.get(i)).collect();

    let function_ref = v8::Global::new(scope, function_ref);
    let params: Vec<v8::Global<v8::Value>> = params
        .into_iter()
        .map(|param| v8::Global::new(scope, param))
        .collect();

    let event_loop = EventLoop::get_mut();
    let timer_id = event_loop.enqueue_immediate_timer(Box::new(move |scope| {
        let function_ref = function_ref.open(scope);
        let params: Vec<v8::Local<v8::Value>> = params
            .into_iter()
            .map(|param| v8::Local::new(scope, param))
            .collect();
        function_ref.call(scope, v8::undefined(scope).into(), &params);
        ExecuteState::FULFILLED
    }));

    ret.set(v8::String::new(scope, &timer_id.into_raw()).unwrap().into());
}
