pub type CallbackOnce = Box<dyn for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>)>;

pub type Callback = Box<dyn for<'s> Fn(&mut v8::PinnedRef<'s, v8::HandleScope>)>;
