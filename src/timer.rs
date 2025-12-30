mod set_timeout;

use crate::helper;

pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    global: v8::Local<'s, v8::Object>,
) -> Option<bool> {
    let set_timeout = v8::Function::new(scope, set_timeout::v8_set_timeout)?;
    helper::register(scope, global, "setTimeout", set_timeout.into())
}
