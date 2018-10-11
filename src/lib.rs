use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

trait FnBox {
    fn call_box(self: Box<dyn Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<dyn F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;


impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// the size is the number in the pool
    ///
    /// # Panics
    ///
    /// the `new` function will panic if the size is zero

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
        self.sender.send(job).unwrap();
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().expect("Thread lock error!").recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}


