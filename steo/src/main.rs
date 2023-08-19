use std::{thread, time};
use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::sync::Arc;

#[allow(dead_code)]
fn blocking_call() -> String {
    println!("\nThis is a long task...");
    for _ in 0..15 {
        print!(":");
    }
    thread::sleep(time::Duration::from_secs(10));

    String::from("Finally done...")
}

async fn async_hello(id : i32) {
    for _ in 1..10 {
        print!("{}", id);
    }
    println!("\nHello from thread #{}", id);
}

async fn mutex_hello(iarc: Arc<Mutex<i32>>, newvalue: i32) {
    let mut newthread = iarc.lock().await;
    *newthread = newvalue;
    println!("Hello from the new thread {}", newvalue);

    thread::sleep(time::Duration::from_secs(1));
}

async fn playing_with_mutex() {
    let valmutex = 032;
    let mymutex = Mutex::new(valmutex);
    let shared_mutex = Arc::new(mymutex);
    let mut handles  = vec!();

    for i in 0..10 {
        handles.push(
            tokio::spawn(mutex_hello(shared_mutex.clone(), i))
        );
        println!("Old thread i: {:?} ^^^^^^^^^^^^ ", i);
    }

    for h in handles {
        h.await.unwrap();
    }
}

async fn chapter2 () {
    let blockhandle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = vec!();
    for id in 0..10 {
        async_handles.push(tokio::spawn(async_hello(id)));
    }

    for h in async_handles {
        h.await.unwrap();
    }

    let res = blockhandle.await.unwrap();
    println!("\n\t{}\n", res);
}

async fn chapter3() {
    let _ = playing_with_mutex().await;
}

#[tokio::main]
async fn main() {
    //chapter2().await;

    //let _ = sleep(Duration::from_secs(3));

    chapter3().await;
}

#[cfg(test)]
#[tokio::test(flavor="multi_thread", worker_threads=1)]
async fn test_tokio1 () {
}
