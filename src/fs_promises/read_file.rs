use crate::event_loop;
use std::io;

/// Implementation of [fsPromises.readFile](https://nodejs.org/api/fs.html#fspromisesreadfilepath-options)
pub fn v8_read_file<'s>(
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
    event_loop::EventLoop::get_mut().enqueue_task(Box::new(move |scope| {
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
                event_loop::task_result::fulfilled()
            })
            .unwrap_or(event_loop::task_result::pending())
    }));
}
