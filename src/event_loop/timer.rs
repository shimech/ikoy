use crate::event_loop::callback::Callback;

#[derive(Clone)]
pub struct TimerId(String);

impl TimerId {
    fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
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
    pub fn new(callback: Callback, delay: u64) -> Self {
        Self {
            id: TimerId::new(),
            callback,
            when: Timestamp::now().delta(delay as i64),
        }
    }

    pub fn run<'s>(self, scope: &mut v8::PinnedRef<'s, v8::HandleScope>) {
        (self.callback)(scope);
    }

    pub fn should_run(&self) -> bool {
        self.when <= Timestamp::now()
    }
}
