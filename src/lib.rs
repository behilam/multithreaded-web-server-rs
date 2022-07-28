use std::{fmt, thread};

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            todo!();
        }

        Ok(ThreadPool { threads })
    }

    pub fn hi() {
        println!("Holi");
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("Execute");
    }
}
