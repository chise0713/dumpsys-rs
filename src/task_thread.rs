use std::{
    sync::{
        atomic::Ordering,
        mpsc::{self, SendError, Sender},
    },
    thread,
};

use crate::Task;

pub struct TaskThread {
    tx: Sender<Task>,
}

impl TaskThread {
    pub fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                let (args, writer, service, status) = match task {
                    Task::Dump(a, w, s, e) => (a, w, s, e),
                    Task::Shutdown => break,
                };

                // continue drops writer, reader will get an EOF
                let Some(proxy) = service.as_proxy() else {
                    continue;
                };

                // if failed then return the StatusCode back to calling thread
                let _ = proxy
                    .dump(writer, &args)
                    .inspect_err(|e| status.store(i32::from(*e), Ordering::Relaxed));
            }
        });

        Self { tx }
    }

    #[inline(always)]
    pub fn send(&self, t: Task) -> Result<(), SendError<Task>> {
        self.tx.send(t)
    }
}

impl Drop for TaskThread {
    fn drop(&mut self) {
        let _ = self.tx.send(Task::Shutdown);
    }
}
