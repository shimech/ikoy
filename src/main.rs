mod args;
mod console;
mod helper;

use crate::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = std::pin::pin!(v8::HandleScope::new(isolate));
    let scope = &mut scope.init();
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);

    let js_global = context.global(scope);

    /*
     * Create console
     * @see https://developer.mozilla.org/ja/docs/Web/API/console
     */
    console::register(scope, js_global).unwrap();

    let code = v8::String::new(scope, &args.print).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));
}
