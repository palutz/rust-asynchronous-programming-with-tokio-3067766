use std::sync::Arc;

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

async fn order_something(delivery : Arc<Notify>) {
    println!("Start oder");
    sleep(Duration::from_secs(2)).await;
    println!("Order done");
    sleep(Duration::from_secs(2)).await;
    delivery.notify_one();
}

async fn grab_delivery(delivery : Arc<Notify>) {
    delivery.notified().await;
    println!("Going to grab my order");

    sleep(Duration::from_secs(2)).await;
    println!("Received...");
}

#[tokio::main]
async fn main() {
    let delivery : Notify = Notify::new();
    let del_arc = Arc::new(delivery);

    let handle_order = tokio::spawn(order_something(del_arc.clone()));

    let handle_grab = tokio::spawn(grab_delivery(del_arc.clone()));

    handle_order.await.unwrap();
    handle_grab.await.unwrap();
}
