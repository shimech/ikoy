pub struct Task {
    callback: Box<dyn FnOnce() + Send>,
}

impl Task {
    pub fn new(callback: Box<dyn FnOnce() + Send>) -> Self {
        Self { callback }
    }

    pub fn run(self) {
        (self.callback)();
    }
}

pub struct Microtask {
    callback: Box<dyn FnOnce() + Send>,
}

impl Microtask {
    pub fn new(callback: Box<dyn FnOnce() + Send>) -> Self {
        Self { callback }
    }

    pub fn run(self) {
        (self.callback)();
    }
}
