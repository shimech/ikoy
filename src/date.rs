use crate::helper::{self, Register, Registerable};

pub struct Date;

impl<'s> Registerable<'s> for Date {
    const REGISTER: Register<'s> = |scope, global| {
        let date = v8::Object::new(scope);

        let now = v8::Function::new(scope, v8_now)?;
        helper::register(scope, date, "now", now.into())?;

        helper::register(scope, global, "Date", date.into())?;

        Some(true)
    };
}

fn v8_now<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    _args: v8::FunctionCallbackArguments<'s>,
    mut ret: v8::ReturnValue<'s>,
) {
    let now = chrono::Local::now().timestamp_millis();
    ret.set(v8::Number::new(scope, now as f64).into())
}
