use tokio::net::TcpListener;
use tokio::{io::AsyncWriteExt, sync::mpsc::Sender};

use crate::event_loop::{EventLoop, executable::ExecuteState};

pub(super) fn v8_create_server<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    _ret: v8::ReturnValue<'s>,
) {
    let callback: v8::Local<v8::Function> = args.get(0).try_into().unwrap();

    let (request_tx, mut request_rx) = tokio::sync::mpsc::channel::<Sender<String>>(100);

    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();

            let this_request_tx = request_tx.clone();
            let (response_tx, mut response_rx) = tokio::sync::mpsc::channel::<String>(1);
            tokio::spawn(async move {
                this_request_tx.send(response_tx).await.unwrap();

                if let Some(response) = response_rx.recv().await {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}",
                        response
                    );
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
            });
        }
    });

    let callback = v8::Global::new(scope, callback);
    let event_loop = EventLoop::get_mut();
    event_loop.enqueue_task(Box::new(move |scope| {
        if let Ok(tx) = request_rx.try_recv() {
            let tx = Box::new(tx);
            let tx_ptr = Box::into_raw(tx);
            let external = v8::External::new(scope, tx_ptr as *mut std::ffi::c_void);
            let binding = v8::FunctionTemplate::builder(
                |scope: &mut v8::PinnedRef<v8::HandleScope>,
                 args: v8::FunctionCallbackArguments,
                 _ret: v8::ReturnValue| {
                    let body = args.get(0).to_string(scope).unwrap();
                    let body = body.to_rust_string_lossy(scope);

                    let data = args.data();
                    let external = v8::Local::<v8::External>::try_from(data).unwrap();
                    let tx_ptr = external.value() as *mut Sender<String>;
                    unsafe {
                        let tx = &*tx_ptr;
                        tx.try_send(body).unwrap();
                    }
                },
            )
            .data(external.into())
            .build(scope);
            let res = binding.get_function(scope).unwrap();

            callback
                .open(scope)
                .call(scope, v8::undefined(scope).into(), &[res.into()]);
        };
        ExecuteState::PENDING
    }));
}
