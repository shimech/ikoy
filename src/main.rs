use crate::args::Args;
use crate::event_loop::EventLoop;
use crate::event_loop::executable::ExecuteState;
use crate::helper::{Register, Registerable};
use clap::Parser;
use std::cell::RefCell;
use std::rc::Rc;

mod args;
mod console;
mod date;
mod event_loop;
mod fs;
mod fs_promises;
mod helper;
mod http;
mod ikoy;
mod microtask;
mod timer;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let script = args.script();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let result = Rc::new(RefCell::new(None::<String>));
    let result_clone = Rc::clone(&result);

    helper::with_scope(isolate, |context, scope| {
        let js_global = context.global(scope);
        let registers: Vec<Register> = vec![
            /*
             * Original
             */
            ikoy::Ikoy::REGISTER,
            /*
             * Create Date
             */
            date::Date::REGISTER,
            /*
             * Create microtask
             */
            microtask::Microtask::REGISTER,
            /*
             * Create timers
             * @see https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers
             */
            timer::Timer::REGISTER,
            /*
             * Create console
             * @see https://console.spec.whatwg.org
             */
            console::Console::REGISTER,
            /*
             * Create fs
             * @see https://nodejs.org/api/fs.html
             */
            fs::Fs::REGISTER,
            /*
             * Create fsPromises
             * @see https://nodejs.org/api/fs.html#promises-api
             */
            fs_promises::FsPromises::REGISTER,
            /*
             * Create http
             * @see https://nodejs.org/docs/v24.12.0/api/http.html
             */
            http::Http::REGISTER,
        ];

        for register in registers {
            register(scope, js_global).unwrap();
        }

        let code = v8::String::new(scope, &script).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();
        let script = v8::Global::new(scope, script);

        let event_loop = EventLoop::get_mut();
        event_loop.enqueue_task(Box::new(move |scope| {
            let script = script.open(scope);
            let ret = script
                .run(scope)
                .and_then(|v| v.to_string(scope))
                .map(|v| v.to_rust_string_lossy(scope));
            *result_clone.borrow_mut() = ret;
            ExecuteState::FULFILLED
        }));
    });

    let event_loop = EventLoop::get_mut();
    event_loop.run(isolate);

    if args.print.is_some() {
        if let Some(ref result) = *result.borrow() {
            println!("[RESULT] {}", result);
        }
    }
}
