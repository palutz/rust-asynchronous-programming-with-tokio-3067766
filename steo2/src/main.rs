use tokio::sync::oneshot;
use std::sync::Arc;

async fn spy_game(mut rx1: oneshot::Receiver<String>, mut rx2: oneshot::Receiver<String>) {
    let msg = tokio::select! {
        msg1 = &mut rx1=> msg1.unwrap(),
        msg2 = &mut rx2=> msg2.unwrap()
    };
    println!("Spy has received msg: {msg}"); // receiving only one msg and destroying the other channel
}

#[tokio::main]
async fn main() {

}