use std::{thread, time};

pub(crate) fn sleep() {
    // Sleep for 5ms to make it easier to check the print order.
    thread::sleep(time::Duration::from_millis(5));
}
