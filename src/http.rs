use crate::helper::{self, Register, Registerable};

mod create_server;

pub struct Http;

impl<'s> Registerable<'s> for Http {
    const REGISTER: Register<'s> = |scope, global| {
        let http = v8::Object::new(scope);

        let create_server = v8::Function::new(scope, create_server::v8_create_server)?;
        helper::register(scope, http, "createServer", create_server.into())?;

        helper::register(scope, global, "http", http.into())?;

        Some(true)
    };
}
