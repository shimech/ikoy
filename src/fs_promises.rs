use crate::helper::{self, Register, Registerable};

mod read_file;

pub struct FsPromises;

impl<'s> Registerable<'s> for FsPromises {
    const REGISTER: Register<'s> = |scope, global| {
        let fs_promises = v8::Object::new(scope);

        let read_file = v8::Function::new(scope, read_file::v8_read_file)?;
        helper::register(scope, fs_promises, "readFile", read_file.into())?;

        helper::register(scope, global, "fsPromises", fs_promises.into())
    };
}
