use crate::helper::{self, Register, Registerable};

mod error;
mod log;
mod sleep;

pub struct Console;

impl<'s> Registerable<'s> for Console {
    const REGISTER: Register<'s> = |scope, global| {
        let console = v8::Object::new(scope);

        let log = v8::Function::new(scope, log::v8_log)?;
        helper::register(scope, console, "log", log.into())?;

        let error = v8::Function::new(scope, error::v8_error)?;
        helper::register(scope, console, "error", error.into())?;

        helper::register(scope, global, "console", console.into())
    };
}
