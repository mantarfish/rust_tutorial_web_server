use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::new();
        for id in 0..size {
            let worker = Worker::new(id, receiver.clone());
            workers.push(worker);
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute(&self, job: Job) {
        self.sender.send(job).unwrap();
    }
}
struct Worker {
    id: usize,
    thread: JoinHandle<Job>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {

        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job.call_box();
            }
        });

        Worker {
            id,
            thread
        }
    }
}



type Job = Box<FnBox + 'static + Send>;


pub trait FnBox {
    fn call_box(self: Box<Self>);
}


impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}