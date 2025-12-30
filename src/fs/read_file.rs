use std::fs;

/// Implementation of [fs.readFile](https://nodejs.org/api/fs.html#fsreadfilepath-options-callback)
pub fn v8_read_file<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    args: v8::FunctionCallbackArguments<'s>,
    mut _ret: v8::ReturnValue<'s>,
) {
    let path = args.get(0).to_string(scope).unwrap();
    let path = path.to_rust_string_lossy(scope);
    let callback: v8::Local<v8::Function> = args.get(1).try_into().unwrap();

    let (err, data) = match fs::read_to_string(path) {
        Ok(data) => {
            let err: v8::Local<v8::Value> = v8::undefined(scope).into();
            let data: v8::Local<v8::Value> = v8::String::new(scope, &data).unwrap().into();
            (err, data)
        }
        Err(e) => {
            let err = format!("{:?}", e);
            let err = v8::String::new(scope, &err).unwrap();
            let err: v8::Local<v8::Value> = v8::Exception::error(scope, err).into();
            let data: v8::Local<v8::Value> = v8::undefined(scope).into();
            (err, data)
        }
    };

    callback.call(scope, v8::undefined(scope).into(), &[err, data]);
}
