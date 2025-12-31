use crate::event_loop::callback::{Callback, CallbackOnce};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
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
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub struct Timer {
    pub id: TimerId,
    when: Timestamp,
    kind: TimerKind,
}

enum TimerKind {
    Once { callback: CallbackOnce },

    Repeating { callback: Callback, delay: u64 },
}

impl Timer {
    pub fn new_once(callback: CallbackOnce, delay: u64) -> Self {
        Self {
            id: TimerId::generate(),
            when: Timestamp::now().delta(delay as i64),
            kind: TimerKind::Once { callback },
        }
    }

    pub fn new_repeating(callback: Callback, delay: u64) -> Self {
        Self {
            id: TimerId::generate(),
            when: Timestamp::now().delta(delay as i64),
            kind: TimerKind::Repeating { callback, delay },
        }
    }

    pub(crate) fn run<'s>(
        mut self,
        scope: &mut v8::PinnedRef<'s, v8::HandleScope>,
    ) -> Option<Self> {
        match self.kind {
            TimerKind::Once { callback, .. } => {
                (callback)(scope);
                None
            }
            TimerKind::Repeating {
                ref mut callback, ..
            } => {
                (callback)(scope);
                Some(self.copy())
            }
        }
    }

    pub(crate) fn should_run(&self) -> bool {
        self.when <= Timestamp::now()
    }

    fn copy(self) -> Self {
        match self.kind {
            TimerKind::Repeating { callback, delay } => Timer {
                id: self.id.clone(),
                when: self.when.delta(delay as i64),
                kind: TimerKind::Repeating { callback, delay },
            },
            _ => panic!("copy is allowed only for TimerKind::Repeating"),
        }
    }
}

impl PartialEq for Timer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Timer {}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.when.cmp(&other.when)
    }
}

#[cfg(test)]
mod tests {
    mod timer_id {
        mod new {
            use super::super::super::*;

            #[test]
            fn returns_timer_id() {
                // given
                let expected = TimerId("test".to_string());

                // when
                let actual = TimerId::new("test".to_string());

                // then
                assert_eq!(actual, expected);
            }
        }

        mod into_raw {
            use super::super::super::*;

            #[test]
            fn returns_timer_id_as_string() {
                // given
                let expected = "test".to_string();
                let timer_id = TimerId::new(expected.clone());

                // when
                let actual = timer_id.into_raw();

                // then
                assert_eq!(actual, expected);
            }
        }
    }

    mod timestamp {
        mod new {
            use super::super::super::*;

            #[test]
            fn returns_timestamp() {
                // given
                let expected = Timestamp(1000);

                // when
                let actual = Timestamp::new(1000);

                // then
                assert_eq!(actual, expected);
            }
        }

        mod delta {
            use super::super::super::*;

            #[test]
            fn returns_future_timestamp_when_delta_is_positive() {
                // given
                let expected = Timestamp(1500);
                let timestamp = Timestamp::new(1000);

                // when
                let actual = timestamp.delta(500);

                // then
                assert_eq!(actual, expected);
            }

            #[test]
            fn returns_past_timestamp_when_delta_is_negative() {
                // given
                let expected = Timestamp(500);
                let timestamp = Timestamp::new(1000);

                // when
                let actual = timestamp.delta(-500);

                // then
                assert_eq!(actual, expected);
            }
        }
    }
}
