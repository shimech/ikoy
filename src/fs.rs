use crate::helper::{self, Register, Registerable};

mod read_file;

pub struct Fs;

impl<'s> Registerable<'s> for Fs {
    const REGISTER: Register<'s> = |scope, global| {
        let fs = v8::Object::new(scope);

        let read_file = v8::Function::new(scope, read_file::v8_read_file)?;
        helper::register(scope, fs, "readFile", read_file.into())?;

        helper::register(scope, global, "fs", fs.into())
    };
}
