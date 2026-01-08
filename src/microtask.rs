use crate::helper::{self, Registerable};

mod queue_microtask;

pub struct Microtask;

impl<'s> Registerable<'s> for Microtask {
    const REGISTER: helper::Register<'s> = |scope, global| {
        let queue_microtask = v8::Function::new(scope, queue_microtask::v8_queue_microtask)?;
        helper::register(scope, global, "queueMicrotask", queue_microtask.into())
    };
}
