use crate::{
    event_loop::{
        executable::{Callback, CallbackOnce, Executable},
        micro_task::Microtask,
        task::Task,
        timer::{Timer, TimerId},
    },
    helper,
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    sync::OnceLock,
};

pub mod executable;
pub mod micro_task;
pub mod task;
pub mod timer;

static EVENT_LOOP: OnceLock<EventLoop> = OnceLock::new();

/// An event loop implementation similar to Node.js.
/// See also: <https://nodejs.org/en/learn/asynchronous-work/event-loop-timers-and-nexttick>
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
            // timers phase
            // @see https://nodejs.org/en/learn/asynchronous-work/event-loop-timers-and-nexttick#timers
            while let Some(Reverse(timer)) = self.timer_queue.peek() {
                if timer.should_run() {
                    let Reverse(timer) = self.timer_queue.pop().unwrap();

                    if self.is_timer_cleared(&timer.id) {
                        self.cleared_timers.remove(&timer.id);
                        continue;
                    }

                    if let Some(new_timer) =
                        self.execute_with_microtask(isolate, move |scope| timer.execute(scope))
                    {
                        self.timer_queue.push(Reverse(new_timer));
                    }
                } else {
                    break;
                }
            }

            // poll phase
            while let Some(task) = self.task_queue.pop_front() {
                if let Some(new_task) =
                    self.execute_with_microtask(isolate, move |scope| task.execute(scope))
                {
                    self.task_queue.push_back(new_task);
                }

                // Check if the next scheduled timer should run; if so, move to the next phase.
                if let Some(next_timer) = self.next_timer() {
                    if next_timer.should_run() {
                        break;
                    }
                }
            }
        }
    }

    pub fn enqueue_task(&mut self, callback: Callback) {
        let task = Task::new(callback);
        self.task_queue.push_back(task);
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

    fn execute_with_microtask<F, NE: Executable>(
        &mut self,
        isolate: &mut v8::Isolate,
        f: F,
    ) -> Option<NE>
    where
        F: for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>) -> Option<NE>,
    {
        let ret = helper::with_scope(isolate, |_, scope| f(scope));

        // Run V8's internal microtask queue.
        isolate.perform_microtask_checkpoint();

        // Run ikoy's microtask queue.
        while let Some(micro_task) = self.microtask_queue.pop_front() {
            helper::with_scope(isolate, move |_, scope| {
                micro_task.execute(scope);
            });
        }

        ret
    }

    fn is_running(&self) -> bool {
        !self.timer_queue.is_empty()
            || !self.task_queue.is_empty()
            || !self.microtask_queue.is_empty()
    }

    fn is_timer_cleared(&self, id: &TimerId) -> bool {
        self.cleared_timers.contains(id)
    }

    fn next_timer(&self) -> Option<&Timer> {
        for Reverse(timer) in (&self.timer_queue).into_iter() {
            if !self.is_timer_cleared(&timer.id) {
                return Some(&timer);
            }
        }
        None
    }
}
