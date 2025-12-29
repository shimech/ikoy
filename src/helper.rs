pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    object: v8::Local<'s, v8::Object>,
    key: &str,
    value: v8::Local<'s, v8::Value>,
) -> Option<bool> {
    let js_key = v8::String::new(scope, key)?;
    object.set(scope, js_key.into(), value)
}
