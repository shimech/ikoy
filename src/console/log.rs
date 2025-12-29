pub fn v8_log<'s>(
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

    println!("{}", inputs.join(" "));
}
