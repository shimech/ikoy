use crate::console;

/// Implementation of [console.error](https://developer.mozilla.org/en-US/docs/Web/API/console/error_static)
pub fn v8_error<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    _ret: v8::ReturnValue<'s>,
) {
    let inputs: Vec<String> = (0..args.length())
        .map(|i| {
            let arg = args.get(i);
            match arg.to_string(scope) {
                Some(string) => string.to_rust_string_lossy(scope),
                None => "[object]".to_string(),
            }
        })
        .collect();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    eprintln!("[{}] {}", now, inputs.join(" "));

    console::sleep::sleep();
}
