use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // This will be our shared queue of jobs.
    // We initialize it with some jobs (just integers for this example)
    let jobs = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]));

    // This will hold our worker threads
    let mut workers = vec![];

    for id in 0..2 { // create 2 workers
        // Each worker gets a clone of the Arc (this is how they share ownership of the jobs)
        let jobs = Arc::clone(&jobs);

        // Spawn the worker thread
        let worker = thread::spawn(move || {
            loop {
                // Lock the mutex to get access to the jobs. This blocks if another worker
                // has the mutex until they unlock it.
                let mut jobs = jobs.lock().unwrap();

                // If there are any jobs left, pop one off and process it
                if let Some(job) = jobs.pop() {
                    println!("Worker {} processing job {}", id, job);
                } else {
                    // If there are no jobs left, break out of the loop
                    break;
                }
            }
        });

        workers.push(worker);
    }

    // Wait for all the workers to finish
    for worker in workers {
        worker.join().unwrap();
    }
}
