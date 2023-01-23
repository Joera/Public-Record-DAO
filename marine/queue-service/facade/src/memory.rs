use std::cell::RefCell;
use std::thread::LocalKey;

use crate::{Job};

thread_local!(
    static QUEUE: RefCell<Vec<Job>> = RefCell::new(vec!());
);

pub fn add(job: Job)  {

    QUEUE.with(|cell| {
      cell.borrow_mut().push(job.clone());
    });
} 


pub fn get() -> Vec<Job> {

    let mut jobs: Vec<Job> = vec!();

    QUEUE.with(|cell| {    
        
        jobs = cell.borrow_mut().clone().to_vec();
        
    });

    let jobs2 = &jobs;

    jobs2.clone()

}

pub fn clear() {

    QUEUE.with(|cell| {
        cell.replace(vec!());
      });
}

