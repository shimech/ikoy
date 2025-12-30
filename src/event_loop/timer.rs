use crate::event_loop::callback::Callback;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TimerId(String);

impl TimerId {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn into_raw(self) -> String {
        self.0
    }

    fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Clone, PartialEq)]
struct Timestamp(i64);

impl Timestamp {
    fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }

    fn now() -> Self {
        Self::new(chrono::Local::now().timestamp_millis())
    }

    fn delta(&self, delta: i64) -> Self {
        Self::new(self.0 + delta)
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

pub struct Timer {
    pub id: TimerId,
    callback: Callback,
    when: Timestamp,
}

impl Timer {
    pub(crate) fn new(callback: Callback, delay: u64) -> Self {
        Self {
            id: TimerId::generate(),
            callback,
            when: Timestamp::now().delta(delay as i64),
        }
    }

    pub(crate) fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }

    pub(crate) fn should_run(&self) -> bool {
        self.when <= Timestamp::now()
    }
}
