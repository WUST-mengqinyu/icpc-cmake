use std::sync::{Arc, Mutex};
mod bind_core;

struct ThreadManagerInner {
    free_cores: Vec<usize>,
    waiters: Vec<std::sync::mpsc::Sender<usize>>,
}

pub struct ThreadManager {
    inner: Arc<Mutex<ThreadManagerInner>>,
}

impl Clone for ThreadManager {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct Permit {
    core_id: usize,
    manager_ref: ThreadManager,
}

impl Permit {
    fn start_process_with_thread(
        &mut self,
        mut cmd: std::process::Command,
    ) -> std::io::Result<std::process::Child> {
        let core_id = self.core_id;
        std::thread::spawn(move || {
            if !bind_core::set_for_current(bind_core::CoreId { id: core_id }) {
                log::error!("bind_core not supported, will run in any core");
            }
            cmd.spawn()
        })
        .join()
        .unwrap()
    }
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.manager_ref.notify_waiter(self.core_id);
    }
}

impl ThreadManager {
    // TODO: make it with file lock for cross thread share
    pub fn new(cap: Option<usize>) -> Self {
        let cap = cap.unwrap_or_else(|| bind_core::get_core_ids().map_or(0, |ids| ids.len()));
        Self {
            inner: Arc::new(Mutex::new(ThreadManagerInner {
                free_cores: (0..cap).collect(),
                waiters: Vec::with_capacity(cap),
                // mu: Mutex::new(()),
            })),
        }
    }

    pub fn start(&self, cmd: std::process::Command) -> std::io::Result<std::process::Child> {
        let mut p = self.acquire();
        p.start_process_with_thread(cmd)
    }

    // TODO: set it as inner's method?
    fn acquire(&self) -> Permit {
        let mut mu = self.inner.lock().unwrap();
        if !mu.free_cores.is_empty() && mu.waiters.is_empty() {
            let p = mu.free_cores.pop().unwrap();
            // drop(mu);
            return Permit {
                core_id: p,
                manager_ref: self.clone(),
            };
        }
        let (tx, rx) = std::sync::mpsc::channel();
        mu.waiters.push(tx);
        // drop(mu);
        Permit {
            core_id: rx.recv().unwrap(),
            manager_ref: self.clone(),
        }
    }

    fn notify_waiter(&self, core_id: usize) {
        let mut mu = self.inner.lock().unwrap();
        if !mu.waiters.is_empty() {
            let _ = mu.waiters.pop().unwrap().send(core_id);
        } else {
            mu.free_cores.push(core_id);
        }
        // drop(mu);
    }
}
