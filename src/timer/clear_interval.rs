use crate::event_loop::{EventLoop, timer::TimerId};

/// Implementation of [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)
pub fn v8_clear_interval<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut _ret: v8::ReturnValue<'s>,
) {
    let timer_id = args
        .get(0)
        .to_string(scope)
        .map(|s| s.to_rust_string_lossy(scope))
        .map(TimerId::new)
        .unwrap();

    EventLoop::get_mut().clear_timer(&timer_id);
}
