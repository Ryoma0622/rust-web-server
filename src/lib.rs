use std::{
  sync::{ mpsc, Arc, Mutex },
  thread };

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let ( sender, receiver ) = mpsc::channel();

    let receiver = Arc::new(receiver);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender: Some(sender)
    }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    println!("Before Sending Job to Worker");
    self.sender.as_ref().unwrap().send(job).unwrap();
    println!("After Sending Job to Worker");
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());
    println!("After Dropping Sender");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      let Some(thread) = worker.thread.take() else {
        println!("The worker has already had its thread cleaned up.");
        return;
      };

      thread.join().unwrap();
      println!("Shutting down worker {} is Completed", worker.id);
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      println!("Worker {id} - Before Receiver");
      let message = receiver.lock().unwrap().recv();
      println!("Worker {id} - After Receiver");

      match message {
        Ok(job) => {
          println!("Worker {id} got a job; executing.");
          job();
        }
        Err(_) => {
          println!("Worker {id} disconnected; shutting down.");
          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
