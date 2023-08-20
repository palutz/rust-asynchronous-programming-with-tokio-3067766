use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};


async fn tasks(semap : Arc<Semaphore>, idx : i32) {
    let tname = format!("task{}",idx);
    println!("{} in the queue", tname);

    workers(semap, tname).await;
}

async fn workers(semap : Arc<Semaphore>, name: String) {
    let permit = semap.acquire().await.unwrap();

    sleep(Duration::from_secs(2)).await;
    println!("Serving customer {}", name);
    sleep(Duration::from_secs(1)).await;
    println!("Job done for {}", name);

    drop(permit); // job done... forcing the drop of the permit
}

#[tokio::main]
async fn main () {
    let s : Semaphore = Semaphore::new(3);
    let arc_s : Arc<Semaphore> = Arc::new(s);
    let mut handles  = vec!();

    for i in 0..10 {
        handles.push(tokio::spawn(
            tasks(arc_s.clone(), i)
        ));
    }

    for h in handles {
        h.await.unwrap();
    }
}