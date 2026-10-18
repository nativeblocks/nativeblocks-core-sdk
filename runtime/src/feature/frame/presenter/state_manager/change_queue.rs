use std::collections::VecDeque;
use std::sync::Mutex;

use crate::feature::frame::presenter::state_manager::model::FrameChangeType;

#[derive(Default)]
pub(super) struct ChangeQueue {
    queue: Mutex<Queue>,
}

#[derive(Default)]
struct Queue {
    pending: VecDeque<FrameChangeType>,
    is_delivering: bool,
}

impl ChangeQueue {
    pub(super) fn enqueue(&self, change: FrameChangeType) {
        self.queue.lock().unwrap().pending.push_back(change);
    }

    pub(super) fn clear(&self) {
        self.queue.lock().unwrap().pending.clear();
    }

    pub(super) fn deliver_pending(&self, deliver: impl Fn(FrameChangeType)) {
        let mut queue = self.queue.lock().unwrap();
        if queue.is_delivering {
            return;
        }
        queue.is_delivering = true;
        while let Some(change) = queue.pending.pop_front() {
            drop(queue);
            deliver(change);
            queue = self.queue.lock().unwrap();
        }
        queue.is_delivering = false;
    }
}
