use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;

#[derive(Clone)]
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

    async fn process_task(&self, task: Task) -> Result<(), Box<dyn std::error::Error>> {
        println!("Worker {} started processing task {}", self.worker_id, task.id);
        sleep(Duration::from_secs(task.duration)).await;
        println!("Worker {} completed task {}", self.worker_id, task.id);
        Ok(())
    }

    async fn run(&self, tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles = vec![];
        for task in tasks {
            let worker = self.clone();
            let handle = task::spawn(async move {
                worker.process_task(task).await
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await??;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = Arc::new(Yamashiro::new(1));
    let tasks = vec![
        Task { id: 1, duration: 2 },
        Task { id: 2, duration: 1 },
        Task { id: 3, duration: 3 },
    ];

    println!("Yamashiro worker starting...");
    worker.run(tasks).await?;
    println!("Yamashiro worker finished all tasks.");

    Ok(())
}
