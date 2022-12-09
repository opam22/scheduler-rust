// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::Arc;

#[derive(Debug)]
enum Unit {
    Seconds,
    Minutes,
    Hours
}

struct Scheduler {
    logger: String,
    jobs: Vec<Arc<Job>>
}

struct Job {
    id: i32,
    name: String,
    every: u64,
    unit: Unit,
    task: Box<dyn Fn() -> () + Send + Sync>, // Box is used when we want to use heap reference instead of stack 
    registered_at: SystemTime
}

impl Scheduler {
    fn new(logger: String) -> Self {
        Scheduler {
            logger: logger,
            jobs: Vec::<Arc<Job>>::new(),
        }
    }

    fn add_job(&mut self, job: Job) {
        self.jobs.push(Arc::new(Job {
            id: job.id,
            name: job.name,
            every: job.every,
            unit: job.unit,
            task: Box::new(job.task),
            registered_at: job.registered_at
        }));
    }

    fn get_jobs(&self) {
    }

    fn start(&self) {
        loop {
            for job in &self.jobs {
                let j = job.clone();
                // Create a new thread and run the background function in it.
                thread::spawn(move || {
                    // This closure will be run on a separate thread
                    // run the closure here
                    Self::should_run(j)
                });
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    fn should_run(job: Arc<Job>) {
        match job.registered_at.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs()%job.every == 0 {
                        println!("{}", elapsed.as_secs());
                        (job.task)();
                }
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
            }
        };
    }
}

fn main() { 
    let mut sch = Scheduler::new(String::from("logger"));
    
    sch.add_job(Job { 
        id: 1,
        name: String::from("downloading report"),
        every: 1,
        unit: Unit::Seconds,
        task: Box::new(|| {
            // This is the first task. It will be executed every 1 seconds.
            println!("downloading...");
        }),
        registered_at: SystemTime::now()
    });

    sch.add_job(Job { 
        id: 2,
        name: String::from("uploading report"),
        every: 10,
        unit: Unit::Seconds,
        task: Box::new(|| {
            // This is the second task. It will be executed every 10 seconds and took 5 secs to finish.
            println!("uploading...");
            thread::sleep(Duration::from_secs(5));
            println!("uploading... done");
        }),
        registered_at: SystemTime::now()
    });

    sch.start();

}
