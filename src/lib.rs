use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct PoolCreationError {
    pub message: String,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POOL_CREATION_ERROR: {}", self.message)
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {} }} {}",
            file!(),
            line!(),
            self.message
        )
    }
}

impl ThreadPool {
    /**Create a new ThreadPool
     *
     * The size is the number of threads in the pool.
     *
     * # Panics
     *
     * The `new` function will panic if the size is zero.
     */
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError {
                message: "Pool size cannot be zero!".to_owned(),
            });
        }

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Ok(ThreadPool { workers, tx })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.tx.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = rx.lock().expect("Mutex is poisoned").recv().unwrap();
            println!("Worker {id} got a job; executing...");

            job();
        });

        Worker { id, thread }
    }
}
