use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

trait FnBox {
    fn call_closure_in_box(self: Box<Self>);
  }

// For all types that are "Functions which are only called once", add the
// function "call_closure_in_box", which allows calling the function from
// inside a Box (if it is stored within one).
impl<F: FnOnce()> FnBox for F {
    fn call_closure_in_box(self: Box<F>) {
        (*self)()
    }
  }

type Job = Box<FnBox + Send + 'static>;

// The Message types which may be received
// on the threadpool.
//
// Either the message is a job, accompanied by a `Job` object, which includes a
// closure that can be invoked with `call_closure_in_box`, or it is a request
// to terminate with no accompanying data.
enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Sends messages to all threads, causing them to terminate,
    /// and blocks until all threads have been joined.
    ///
    /// Called automatically when ThreadPool goes out of
    /// scope (thanks to the fact that `Drop` is the destructor
    /// trait for Rust).
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// This is just for `id` for debugging.
#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        job.call_closure_in_box();
                    },
                    Message::Terminate => {
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn threadpool_empty() {
        ThreadPool::new(0);
    }

    #[test]
    fn threadpool_lifetime() {
        // Although we send no work, ensure the threadpool
        // can initialize and tear down.
        ThreadPool::new(4);
    }

    #[test]
    fn threadpool_work() {
        // Fill a uint32 with '1's, using a new job for each bit.
        let test_value = Arc::new(Mutex::new(0));
        let golden_value = 0xFFFFFFFFu32;
        {
            let pool = ThreadPool::new(4);
            for i in 0..32 {
                let worker_value = test_value.clone();
                pool.execute(move || {
                    let new_bit = 1 << i;
                    let mut value = worker_value.lock().unwrap();
                    assert_eq!(*value & new_bit, 0);
                    assert_ne!(*value, golden_value);
                    *value |= new_bit;
                });
            }
        }
        assert_eq!(*test_value.lock().unwrap(), golden_value);
    }
}
