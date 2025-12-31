pub type TaskResult = Option<()>;

pub fn fulfilled() -> TaskResult {
    Some(())
}

pub fn pending() -> TaskResult {
    None
}
