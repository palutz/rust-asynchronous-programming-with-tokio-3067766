use std::fmt::write;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

async fn read_doc(shareddoc: Arc<RwLock<String>>, id : i32) {
    let readit = shareddoc.read().await;

    println!("{} Reading...{}", id, readit);
}

async fn write_doc(shareddoc: Arc<RwLock<String>>, new_str : &str) {
    let mut writer = shareddoc.write().await;

    writer.push_str(new_str);
}

#[tokio::main]
async fn main() {
    let a_doc = Arc::new(RwLock::new("".to_string()));

    let mut handles = vec!();
    for s in "a b c d e f g h i j k l m n o p q r s t u v w x y z".split_whitespace() {
        handles.push(tokio::spawn(read_doc(a_doc.clone(), 1)));

        handles.push(tokio::spawn(write_doc(a_doc.clone(), s)));

        handles.push(tokio::spawn(read_doc(a_doc.clone(), 2)));
        handles.push(tokio::spawn(read_doc(a_doc.clone(), 3)));
        handles.push(tokio::spawn(write_doc(a_doc.clone(), " ")));
    }

    for h in handles {
        h.await.unwrap();
    }
}
