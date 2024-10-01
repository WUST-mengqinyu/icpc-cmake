use std::sync::{Arc, Mutex};

struct ThreadManagerInner {
    free_cores: crossbeam::deque::Worker<usize>,
    waiters: crossbeam::deque::Worker<tokio::sync::mpsc::Sender<usize>>,
    mu: Mutex<()>,
}

pub struct ThreadManager {
    inner: Arc<ThreadManagerInner>,
}

pub struct Permit {
    core_id: usize,
    manager_ref: Arc<ThreadManagerInner>,
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.manager_ref.
    }
}

impl ThreadManager {
    pub async fn acquire(&self) -> Permit {
        let mu = self.inner.mu.lock().unwrap();
        if !self.inner.free_cores.is_empty() && self.inner.waiters.is_empty() {
            let p = self.inner.free_cores.pop().unwrap();
            drop(mu);
            return Permit {
                core_id: p,
                manager_ref: self.inner.clone(),
            };
        }
        let (tx, mut rx) = tokio::sync::mpsc::channel(0);
        self.inner.waiters.push(tx);
        drop(mu);
        Permit {
            core_id: rx.recv().await.unwrap(),
            manager_ref: self.inner.clone(),
        }
    }

    fn notify_waiter(&self, core_id: usize) {
        let mu = self.inner.mu.lock().unwrap();
        if !self.inner.waiters.is_empty() {
            self.inner.
        }
        drop(mu);
    }
}
