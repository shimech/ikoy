use crate::helper;

mod read_file;

pub fn register<'s>(
    scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    global: v8::Local<'s, v8::Object>,
) -> Option<bool> {
    let fs = v8::Object::new(scope);

    let read_file = v8::Function::new(scope, read_file::v8_read_file)?;
    helper::register(scope, fs, "readFile", read_file.into())?;

    helper::register(scope, global, "fs", fs.into())
}
