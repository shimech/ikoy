pub struct Task {
    callback: Box<dyn FnOnce()>,
}

impl Task {
    pub fn new(callback: Box<dyn FnOnce()>) -> Self {
        Self { callback }
    }

    pub fn run(self) {
        (self.callback)();
    }
}

pub struct Microtask {
    callback: Box<dyn FnOnce()>,
}

impl Microtask {
    pub fn new(callback: Box<dyn FnOnce()>) -> Self {
        Self { callback }
    }

    pub fn run(self) {
        (self.callback)();
    }
}
