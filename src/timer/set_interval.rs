use crate::{event_loop, timer::delay::DEFAULT_DELAY};

/// Implementation of [setInterval](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)
pub fn v8_set_interval<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut ret: v8::ReturnValue<'s>,
) {
    let function_ref: v8::Local<v8::Function> = args.get(0).try_into().unwrap();
    let delay = if args.length() > 1 {
        args.get(1)
            .to_uint32(scope)
            .map(|v| v.value())
            .unwrap_or(DEFAULT_DELAY as u32) as u64
    } else {
        DEFAULT_DELAY
    };
    let params: Vec<v8::Local<v8::Value>> = (2..args.length()).map(|i| args.get(i)).collect();

    let function_ref = v8::Global::new(scope, function_ref);
    let params: Vec<v8::Global<v8::Value>> = params
        .into_iter()
        .map(|param| v8::Global::new(scope, param))
        .collect();

    let timer_id = event_loop::EventLoop::get_mut().enqueue_repeating_timer(
        Box::new(move |scope| {
            let function_ref = function_ref.open(scope);
            let params: Vec<v8::Local<v8::Value>> = params
                .clone()
                .into_iter()
                .map(|param| v8::Local::new(scope, param))
                .collect();
            function_ref.call(scope, v8::undefined(scope).into(), &params);
            event_loop::task_result::fulfilled()
        }),
        delay,
    );

    ret.set(v8::String::new(scope, &timer_id.into_raw()).unwrap().into());
}
