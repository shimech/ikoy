use crate::event_loop::{EventLoop, executable::ExecuteState};
use std::io;

/// Implementation of [fsPromises.readFile](https://nodejs.org/api/fs.html#fspromisesreadfilepath-options)
pub(super) fn v8_read_file<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut ret: v8::ReturnValue<'s>,
) {
    let path = args.get(0).to_string(scope).unwrap();
    let path = path.to_rust_string_lossy(scope);

    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    ret.set(promise.into());

    let (tx, mut rx) = tokio::sync::oneshot::channel::<Result<String, io::Error>>();

    tokio::spawn(async move {
        let result = tokio::fs::read_to_string(path).await;
        tx.send(result)
    });

    let resolver = v8::Global::new(scope, resolver);
    let event_loop = EventLoop::get_mut();
    event_loop.enqueue_task(Box::new(move |scope| {
        let resolver = resolver.open(scope);

        rx.try_recv()
            .map(|result| {
                match result {
                    Ok(data) => {
                        let data = v8::String::new(scope, &data).unwrap();
                        resolver.resolve(scope, data.into());
                    }
                    Err(err) => {
                        let err = format!("{:?}", err);
                        let err = v8::String::new(scope, &err).unwrap();
                        let err = v8::Exception::error(scope, err);
                        resolver.reject(scope, err.into());
                    }
                };
                ExecuteState::FULFILLED
            })
            .unwrap_or(ExecuteState::PENDING)
    }));
}
