mod callback;
pub mod task;
pub mod timer;

use crate::{
    event_loop::{
        callback::{Callback, CallbackOnce},
        task::{Microtask, Task},
        timer::{Timer, TimerId},
    },
    helper,
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    sync::OnceLock,
};

static EVENT_LOOP: OnceLock<EventLoop> = OnceLock::new();

pub struct EventLoop {
    timer_queue: BinaryHeap<Reverse<Timer>>,
    cleared_timers: HashSet<TimerId>,
    task_queue: VecDeque<Task>,
    microtask_queue: VecDeque<Microtask>,
}

// This runtime works on single thread. So, mutex is not needed.
unsafe impl Send for EventLoop {}
unsafe impl Sync for EventLoop {}

impl EventLoop {
    pub fn get() -> &'static Self {
        EVENT_LOOP.get_or_init(Self::new)
    }

    #[allow(invalid_reference_casting)]
    pub fn get_mut() -> &'static mut Self {
        unsafe { &mut *(Self::get() as *const Self as *mut Self) }
    }

    pub fn new() -> Self {
        Self {
            timer_queue: BinaryHeap::new(),
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

            while let Some(Reverse(timer)) = self.timer_queue.peek() {
                if timer.should_run() {
                    let Reverse(timer) = self.timer_queue.pop().unwrap();

                    if self.is_timer_cleared(&timer.id) {
                        self.cleared_timers.remove(&timer.id);
                        continue;
                    }

                    helper::with_scope(isolate, |_, scope| {
                        if let Some(new_timer) = timer.run(scope) {
                            self.timer_queue.push(Reverse(new_timer));
                        }
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

    pub fn enqueue_once_timer(&mut self, callback: CallbackOnce, delay: u64) -> TimerId {
        let timer = Timer::new_once(callback, delay);
        let id = timer.id.clone();
        self.timer_queue.push(Reverse(timer));
        id
    }

    pub fn enqueue_repeating_timer(&mut self, callback: Callback, delay: u64) -> TimerId {
        let timer = Timer::new_repeating(callback, delay);
        let id = timer.id.clone();
        self.timer_queue.push(Reverse(timer));
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
