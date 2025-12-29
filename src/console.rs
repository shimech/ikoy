mod log;

use crate::helper;

pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    global: v8::Local<'s, v8::Object>,
) -> Option<bool> {
    let console = v8::Object::new(scope);

    let log = v8::Function::new(scope, log::v8_log)?;
    helper::register(scope, console, "log", log.into())?;

    helper::register(scope, global, "console", console.into())
}
