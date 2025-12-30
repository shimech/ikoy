mod args;
mod console;
mod event_loop;
mod helper;
mod timer;

use crate::{args::Args, event_loop::EventLoop};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let script = args.script();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    helper::with_scope(isolate, |context, scope| {
        let js_global = context.global(scope);

        /*
         * Create timers
         * @see https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#timers
         */
        timer::register(scope, js_global).unwrap();

        /*
         * Create console
         * @see https://console.spec.whatwg.org
         */
        console::register(scope, js_global).unwrap();

        let code = v8::String::new(scope, &script).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();

        EventLoop::get_mut().run(scope);

        if args.print.is_some() {
            let result = result.to_string(scope).unwrap();
            println!("{}", result.to_rust_string_lossy(scope));
        }
    });
}
