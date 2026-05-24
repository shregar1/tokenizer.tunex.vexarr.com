//! Backpressure contracts for tokenizer streaming.

/// Bounded queue capacity information.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueCapacity {
    /// Maximum queue slots.
    pub capacity: usize,
    /// Currently used queue slots.
    pub len: usize,
}

impl QueueCapacity {
    /// Returns queue pressure in the range 0.0..=1.0.
    #[must_use]
    pub fn pressure(self) -> f64 {
        if self.capacity == 0 {
            return 1.0;
        }
        (self.len as f64 / self.capacity as f64).clamp(0.0, 1.0)
    }

    /// Returns true when queue is full.
    #[must_use]
    pub const fn is_full(self) -> bool {
        self.len >= self.capacity
    }
}

/// Backpressure state observed by the adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackpressureState {
    /// Queue can accept more work.
    Ready,
    /// Queue is nearing capacity.
    Pressured,
    /// Queue is full.
    Full,
}

/// Action requested by a backpressure hook.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackpressureAction {
    /// Continue processing.
    Continue,
    /// Yield to the caller/runtime.
    Yield,
    /// Stop accepting more input.
    Stop,
}

/// Synchronous backpressure hook.
pub trait BackpressureHook: Send + Sync {
    /// Called before processing an input batch.
    fn on_capacity(&self, capacity: QueueCapacity) -> BackpressureAction;
}

impl<F> BackpressureHook for F
where
    F: Fn(QueueCapacity) -> BackpressureAction + Send + Sync,
{
    fn on_capacity(&self, capacity: QueueCapacity) -> BackpressureAction {
        self(capacity)
    }
}
