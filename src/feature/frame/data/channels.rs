use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::feature::frame::domain::repository::FrameResult;

pub(crate) struct FrameChannels {
    senders: Mutex<HashMap<String, watch::Sender<FrameResult>>>,
}

impl FrameChannels {
    pub(crate) fn new() -> Self {
        return Self {
            senders: Mutex::new(HashMap::new()),
        };
    }

    pub(crate) fn subscribe(
        &self,
        route: &str,
        initial: impl FnOnce() -> FrameResult,
    ) -> watch::Receiver<FrameResult> {
        let mut senders = self.senders.lock().unwrap();
        senders.retain(|_, sender| !sender.is_closed());
        return senders
            .entry(route.to_string())
            .or_insert_with(move || watch::channel(initial()).0)
            .subscribe();
    }

    pub(crate) fn publish(&self, route: &str, value: FrameResult) {
        let mut senders = self.senders.lock().unwrap();
        let Some(sender) = senders.get(route) else {
            return;
        };
        let unchanged = {
            let current = sender.borrow();
            is_same_frame(&value, &current)
        };
        if unchanged {
            return;
        }
        if sender.send(value).is_err() {
            senders.remove(route);
        }
    }
}

fn is_same_frame(next: &FrameResult, current: &FrameResult) -> bool {
    return match (next, current) {
        (Ok(next), Ok(current)) => Arc::ptr_eq(next, current),
        _ => false,
    };
}
