use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct StopToken {
    stop_requested: Arc<AtomicBool>,
}

impl StopToken {
    pub fn new() -> Self {
        Self { stop_requested: Arc::new(AtomicBool::new(false)), }
    }

    pub fn stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::Acquire)
    }

    pub fn request_stop(&self) {
        self.stop_requested.store(true, Ordering::Release);
    }
}
