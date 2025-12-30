use crate::event_loop::callback::Callback;

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

#[derive(Clone, PartialEq, Debug)]
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
