use std::{thread, time};
use tokio::time::{sleep, Duration};


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

#[tokio::main]
async fn main() {
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

#[cfg(test)]
#[tokio::test(flavor="multi_thread", worker_threads=1)]
async fn test_tokio1 () {
}
