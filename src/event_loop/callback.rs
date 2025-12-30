pub type Callback = Box<dyn for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>)>;
