pub(crate) trait Executable {
    type NextExecutable: Executable;

    fn execute<'s>(
        self,
        scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    ) -> Option<Self::NextExecutable>;
}

pub enum ExecuteState {
    PENDING,
    FULFILLED,
    // REJECTED,
}

pub(crate) type CallbackOnce =
    Box<dyn for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>) -> ExecuteState>;

pub(crate) type Callback =
    Box<dyn for<'s> FnMut(&mut v8::PinnedRef<'s, v8::HandleScope>) -> ExecuteState>;
