mod callback;
pub mod task;
pub mod timer;

use crate::{
    event_loop::{
        callback::Callback,
        task::{Microtask, Task},
        timer::{Timer, TimerId},
    },
    helper,
};
use std::{
    collections::{HashSet, VecDeque},
    sync::OnceLock,
};

static EVENT_LOOP: OnceLock<EventLoop> = OnceLock::new();

pub struct EventLoop {
    timer_queue: VecDeque<Timer>,
    cleared_timers: HashSet<TimerId>,
    task_queue: VecDeque<Task>,
    microtask_queue: VecDeque<Microtask>,
}

// This runtime works on single thread. So, mutex is not needed.
unsafe impl Send for EventLoop {}
unsafe impl Sync for EventLoop {}

impl EventLoop {
    pub fn get() -> &'static Self {
        EVENT_LOOP.get_or_init(|| Self::new())
    }

    #[allow(invalid_reference_casting)]
    pub fn get_mut() -> &'static mut Self {
        unsafe { &mut *(Self::get() as *const Self as *mut Self) }
    }

    pub fn new() -> Self {
        Self {
            timer_queue: VecDeque::new(),
            cleared_timers: HashSet::new(),
            task_queue: VecDeque::new(),
            microtask_queue: VecDeque::new(),
        }
    }

    pub fn run(&mut self, isolate: &mut v8::Isolate) {
        while self.is_running() {
            while let Some(micro_task) = self.microtask_queue.pop_front() {
                helper::with_scope(isolate, |_, scope| {
                    micro_task.run(scope);
                });
            }

            while let Some(timer) = self.timer_queue.front() {
                if timer.should_run() {
                    let timer = self.timer_queue.pop_front().unwrap();

                    if self.is_timer_cleared(&timer.id) {
                        self.cleared_timers.remove(&timer.id);
                        continue;
                    }

                    helper::with_scope(isolate, |_, scope| {
                        timer.run(scope);
                    });
                } else {
                    break;
                }
            }

            if let Some(task) = self.task_queue.pop_front() {
                helper::with_scope(isolate, |_, scope| {
                    task.run(scope);
                });
                continue;
            }
        }
    }

    pub fn enqueue_timer(&mut self, callback: Callback, delay: u64) -> TimerId {
        let timer = Timer::new(callback, delay);
        let id = timer.id.clone();
        self.timer_queue.push_back(timer);
        id
    }

    pub fn clear_timer(&mut self, id: &TimerId) {
        // Cleared timer ids are stored in a set to execute this method with O(1) time complexity.
        self.cleared_timers.insert(id.clone());
    }

    fn is_running(&self) -> bool {
        !self.timer_queue.is_empty()
            || !self.task_queue.is_empty()
            || !self.microtask_queue.is_empty()
    }

    fn is_timer_cleared(&self, id: &TimerId) -> bool {
        self.cleared_timers.contains(id)
    }
}
