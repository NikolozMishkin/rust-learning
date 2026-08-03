async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}
use std::time::Duration;
use tokio::time::sleep;
#[tokio::main]
//#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing, first time");
            printing(i).await;
            println!("Task {i}, printing, second time");
            printing(i).await;
            println!("Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("All Tasks are now completed");
}
