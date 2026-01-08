use crate::event_loop::{EventLoop, executable::ExecuteState};
use std::io;

/// Implementation of [fs.readFile](https://nodejs.org/api/fs.html#fsreadfilepath-options-callback)
pub(super) fn v8_read_file<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut ret: v8::ReturnValue<'s>,
) {
    let path = args.get(0).to_string(scope).unwrap();
    let path = path.to_rust_string_lossy(scope);
    let callback: v8::Local<v8::Function> = args.get(1).try_into().unwrap();

    ret.set(v8::undefined(scope).into());

    let (tx, mut rx) = tokio::sync::oneshot::channel::<Result<String, io::Error>>();

    tokio::spawn(async move {
        let result = tokio::fs::read_to_string(path).await;
        tx.send(result)
    });

    let callback = v8::Global::new(scope, callback);
    let event_loop = EventLoop::get_mut();
    event_loop.enqueue_task(Box::new(move |scope| {
        rx.try_recv()
            .map(|result| {
                let (err, data) = match result {
                    Ok(data) => {
                        let err: v8::Local<v8::Value> = v8::undefined(scope).into();
                        let data: v8::Local<v8::Value> =
                            v8::String::new(scope, &data).unwrap().into();
                        (err, data)
                    }
                    Err(err) => {
                        let err = format!("{:?}", err);
                        let err = v8::String::new(scope, &err).unwrap();
                        let err: v8::Local<v8::Value> = v8::Exception::error(scope, err);
                        let data: v8::Local<v8::Value> = v8::undefined(scope).into();
                        (err, data)
                    }
                };
                callback
                    .open(scope)
                    .call(scope, v8::undefined(scope).into(), &[err, data]);
                ExecuteState::FULFILLED
            })
            .unwrap_or(ExecuteState::PENDING)
    }));
}
