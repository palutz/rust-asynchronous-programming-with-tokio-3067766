use std::thread::JoinHandle;
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
    let thn = newvalue - 100;
    println!("{} ***********************", thn);
    println!("thread {}", thn);
    println!("{} ***********************", thn);
    let mut remote_val = iarc.lock().await;

    *remote_val = newvalue;
    println!("new value is {}", remote_val);

    let _ = sleep(Duration::from_secs(3));
}

fn playing_with_mutex() -> Vec<tokio::task::JoinHandle<()>> {
    let anum = 13i32;

    let mutexvec = Mutex::new(anum);
    let mutexarc = Arc::new(mutexvec);

    let mut handles = vec!();

    for i in 100..110 {
        handles.push(tokio::spawn(
            mutex_hello(mutexarc.clone(), i)
        ));
    }

    handles
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
    for t in playing_with_mutex() {
        let _ = t.await;
    }
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
