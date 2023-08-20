use std::sync::Arc;

use tokio::sync::{Barrier, BarrierWaitResult, Notify};
use tokio::time::{sleep, Duration};

async fn production_barrier(barrier : Arc<Barrier>, notifyme: Arc<Notify>) -> BarrierWaitResult {
    println!("Waiting for the green light");

    let bar_res = barrier.wait().await;
    println!("One passed...");

    if bar_res.is_leader() {
        notifyme.notify_one(); // let's go!!!
    }

    bar_res
}

#[tokio::main]
async fn main() {
    let barrier_num = 6;
    let barr_arc = Arc::new(Barrier::new(barrier_num));
    let noti_arc = Arc::new(Notify::new());

    let mut handles = vec!();
    for i in 0..30 {
        if i > 0 && i % barrier_num == 0 { // wait to complete the box
            noti_arc.notified().await;
            sleep(Duration::from_secs(1)).await;
        }
        handles.push(tokio::spawn(
            production_barrier(barr_arc.clone(), noti_arc.clone())
        ));
    }

    let mut counter = 0i32;
    for h in handles {
        let r = h.await.unwrap();
        if r.is_leader() {
            counter += 1;
        }
    }

    println!("Created {} 6 packs", counter);
}
