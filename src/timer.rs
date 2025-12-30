use crate::helper;

mod clear_interval;
mod clear_timeout;
mod delay;
mod set_interval;
mod set_timeout;

pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    global: v8::Local<'s, v8::Object>,
) -> Option<bool> {
    let set_timeout = v8::Function::new(scope, set_timeout::v8_set_timeout)?;
    helper::register(scope, global, "setTimeout", set_timeout.into())?;

    let clear_timeout = v8::Function::new(scope, clear_timeout::v8_clear_timeout)?;
    helper::register(scope, global, "clearTimeout", clear_timeout.into());

    let set_interval = v8::Function::new(scope, set_interval::v8_set_interval)?;
    helper::register(scope, global, "setInterval", set_interval.into())?;

    let clear_interval = v8::Function::new(scope, clear_interval::v8_clear_interval)?;
    helper::register(scope, global, "clearInterval", clear_interval.into())?;

    Some(true)
}
