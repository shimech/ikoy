use crate::{
    event_loop::{
        executable::{Callback, CallbackOnce, Executable},
        microtask::Microtask,
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
pub mod microtask;
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
    immediate_timer_queue: VecDeque<Timer>,
}

// This runtime works on single thread. So, mutex is not needed.
unsafe impl Send for EventLoop {}
unsafe impl Sync for EventLoop {}

impl EventLoop {
    #[allow(invalid_reference_casting)]
    pub fn get_mut() -> &'static mut Self {
        unsafe { &mut *(Self::get() as *const Self as *mut Self) }
    }

    fn get() -> &'static Self {
        EVENT_LOOP.get_or_init(Self::new)
    }

    fn new() -> Self {
        Self {
            timer_queue: BinaryHeap::new(),
            cleared_timers: HashSet::new(),
            task_queue: VecDeque::new(),
            microtask_queue: VecDeque::new(),
            immediate_timer_queue: VecDeque::new(),
        }
    }

    /// [Reference](https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick)
    pub fn run(&mut self, isolate: &mut v8::Isolate) {
        while self.is_running() {
            self.timers_phase(isolate);
            self.pending_callbacks_phase(isolate);
            self.poll_phase(isolate);
            self.check_phase(isolate);
            self.close_callbacks_phase(isolate);
        }
    }

    pub fn enqueue_task(&mut self, callback: Callback) {
        let task = Task::new(callback);
        self.task_queue.push_back(task);
    }

    pub fn enqueue_microtask(&mut self, callback: CallbackOnce) {
        let microtask = Microtask::new(callback);
        self.microtask_queue.push_back(microtask);
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

    pub fn enqueue_immediate_timer(&mut self, callback: CallbackOnce) -> TimerId {
        let timer = Timer::new_immediate(callback);
        let id = timer.id.clone();
        self.immediate_timer_queue.push_back(timer);
        id
    }

    pub fn clear_timer(&mut self, id: &TimerId) {
        // Cleared timer ids are stored in a set to execute this method with O(1) time complexity.
        self.cleared_timers.insert(id.clone());
    }

    /// [Reference](https://nodejs.org/en/learn/asynchronous-work/event-loop-timers-and-nexttick#timers)
    fn timers_phase(&mut self, isolate: &mut v8::Isolate) {
        while let Some(Reverse(timer)) = self.timer_queue.peek() {
            if timer.should_run() {
                let Reverse(timer) = self.timer_queue.pop().unwrap();

                if self.is_timer_cleared(&timer.id) {
                    self.cleared_timers.remove(&timer.id);
                    continue;
                }

                if let Some(new_timer) =
                    self.execute_then_microtask(isolate, move |scope| timer.execute(scope))
                {
                    self.timer_queue.push(Reverse(new_timer));
                }
            } else {
                break;
            }
        }
    }

    /// [Reference](https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick#pending-callbacks)
    fn pending_callbacks_phase(&mut self, _isolate: &mut v8::Isolate) {
        // noop
    }

    /// [Reference](https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick#poll)
    fn poll_phase(&mut self, isolate: &mut v8::Isolate) {
        while let Some(task) = self.task_queue.pop_front() {
            if let Some(new_task) =
                self.execute_then_microtask(isolate, move |scope| task.execute(scope))
            {
                self.task_queue.push_back(new_task);
            }

            // Check if immediate timers are in the queue; if so, proceed to the next phase.
            if self.has_immediate_timer() {
                break;
            }

            // Check if the next scheduled timer should run; if so, move to the next phase.
            if let Some(next_timer) = self.next_timer() {
                if next_timer.should_run() {
                    break;
                }
            }
        }
    }

    /// [Reference](https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick#check)
    fn check_phase(&mut self, isolate: &mut v8::Isolate) {
        while let Some(immediate_timer) = self.immediate_timer_queue.pop_front() {
            if self.is_timer_cleared(&immediate_timer.id) {
                self.cleared_timers.remove(&immediate_timer.id);
                continue;
            }

            helper::with_scope(isolate, move |_, scope| {
                immediate_timer.execute(scope);
            });
        }
    }

    /// [Reference](https://nodejs.org/ja/learn/asynchronous-work/event-loop-timers-and-nexttick#close-callbacks)
    fn close_callbacks_phase(&mut self, _isolate: &mut v8::Isolate) {
        // noop
    }

    fn execute_then_microtask<F, NE>(&mut self, isolate: &mut v8::Isolate, f: F) -> Option<NE>
    where
        F: for<'s> FnOnce(&mut v8::PinnedRef<'s, v8::HandleScope>) -> Option<NE>,
        NE: Executable,
    {
        let ret = helper::with_scope(isolate, |_, scope| f(scope));

        // Microtasks are always executed after other tasks.
        self.execute_microtask(isolate);

        ret
    }

    fn execute_microtask(&mut self, isolate: &mut v8::Isolate) {
        // Run V8's internal microtask queue.
        isolate.perform_microtask_checkpoint();

        // Run ikoy's microtask queue.
        while let Some(micro_task) = self.microtask_queue.pop_front() {
            helper::with_scope(isolate, move |_, scope| {
                micro_task.execute(scope);
            });
        }
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

    fn has_immediate_timer(&self) -> bool {
        !self.immediate_timer_queue.is_empty()
    }
}
