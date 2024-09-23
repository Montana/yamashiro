use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    duration: u64,
}

struct Yamashiro {
    worker_id: u32,
}

impl Yamashiro {
    fn new(worker_id: u32) -> Self {
        Yamashiro { worker_id }
    }

    fn process_task(&self, task: Task) {
        println!("Worker {} started processing task {}", self.worker_id, task.id);
        thread::sleep(Duration::from_secs(task.duration));
        println!("Worker {} completed task {}", self.worker_id, task.id);
    }

    fn run(&self, tasks: Vec<Task>) {
        for task in tasks {
            self.process_task(task);
        }
    }
}

fn main() {
    let worker = Yamashiro::new(1);
    let tasks = vec![
        Task { id: 1, duration: 2 },
        Task { id: 2, duration: 1 },
        Task { id: 3, duration: 3 },
    ];

    println!("Yamashiro worker starting...");
    worker.run(tasks);
    println!("Yamashiro worker finished all tasks.");
}
