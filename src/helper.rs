pub fn with_scope<F, R>(isolate: &mut v8::Isolate, f: F) -> R
where
    F: for<'s> FnOnce(v8::Local<'s, v8::Context>, &mut v8::PinnedRef<'s, v8::HandleScope>) -> R,
{
    let scope = std::pin::pin!(v8::HandleScope::new(isolate));
    let scope = &mut scope.init();
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);
    f(context, scope)
}

pub type Register<'s> =
    fn(&mut v8::PinnedRef<'s, v8::HandleScope>, v8::Local<'s, v8::Object>) -> Option<bool>;

pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    object: v8::Local<'s, v8::Object>,
    key: &str,
    value: v8::Local<'s, v8::Value>,
) -> Option<bool> {
    let js_key = v8::String::new(scope, key)?;
    object.set(scope, js_key.into(), value)
}

pub trait Registerable<'s> {
    const REGISTER: Register<'s>;
}
