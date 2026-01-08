use crate::{
    event_loop::{EventLoop, executable::ExecuteState},
    helper::{self, Register, Registerable},
};
use std::{thread, time};

pub struct Ikoy;

impl<'s> Registerable<'s> for Ikoy {
    const REGISTER: Register<'s> = |scope, global| {
        let ikoy = v8::Object::new(scope);

        let super_heavy_process = v8::Function::new(scope, v8_super_heavy_process)?;
        helper::register(scope, ikoy, "superHeavyProcess", super_heavy_process.into())?;

        helper::register(scope, global, "ikoy", ikoy.into())?;

        Some(true)
    };
}

fn v8_super_heavy_process<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    _ret: v8::ReturnValue<'s>,
) {
    let callback: v8::Local<v8::Function> = args.get(0).try_into().unwrap();
    let delay = args.get(1).to_uint32(scope).map(|v| v.value()).unwrap() as u64;

    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        thread::sleep(time::Duration::from_millis(delay));
        tx.send(())
    });

    let callback = v8::Global::new(scope, callback);
    let event_loop = EventLoop::get_mut();
    event_loop.enqueue_task(Box::new(move |scope| {
        rx.try_recv()
            .map(|_| {
                callback
                    .open(scope)
                    .call(scope, v8::undefined(scope).into(), &[]);
                ExecuteState::FULFILLED
            })
            .unwrap_or(ExecuteState::PENDING)
    }));
}
