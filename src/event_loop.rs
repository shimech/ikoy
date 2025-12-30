mod task;

use crate::event_loop::task::{Microtask, Task};
use std::{
    collections::VecDeque,
    sync::{Mutex, OnceLock},
};

static EVENT_LOOP: OnceLock<Mutex<EventLoop>> = OnceLock::new();

pub struct EventLoop {
    task_queue: VecDeque<Task>,
    microtask_queue: VecDeque<Microtask>,
}

impl EventLoop {
    pub fn get() -> &'static Mutex<EventLoop> {
        EVENT_LOOP.get_or_init(|| Mutex::new(Self::new()))
    }

    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
            microtask_queue: VecDeque::new(),
        }
    }

    pub fn run(&mut self) {
        while self.is_running() {
            while let Some(micro_task) = self.microtask_queue.pop_front() {
                micro_task.run();
            }

            if let Some(task) = self.task_queue.pop_front() {
                task.run();
                continue;
            }
        }
    }

    pub fn enqueue_task(&mut self, callback: Box<dyn FnOnce() + Send>) {
        let task = Task::new(callback);
        self.task_queue.push_back(task);
    }

    fn is_running(&self) -> bool {
        !self.task_queue.is_empty() || !self.microtask_queue.is_empty()
    }
}
