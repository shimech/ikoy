mod args;
mod console;

use crate::args::Args;
use clap::Parser;
use v8::PinScope;

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

    let js_console = v8::Object::new(scope);
    let js_console_log = v8::Function::new(scope, console::log::v8_log).unwrap();
    register(scope, js_console, "log", js_console_log.into());

    register(scope, js_global, "console", js_console.into());

    let code = v8::String::new(scope, &args.print).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();
    println!("{}", result.to_rust_string_lossy(scope));
}

fn register(
    scope: &PinScope,
    object: v8::Local<v8::Object>,
    key: &str,
    value: v8::Local<v8::Value>,
) -> Option<bool> {
    let js_key = v8::String::new(scope, key)?;
    object.set(scope, js_key.into(), value)
}
