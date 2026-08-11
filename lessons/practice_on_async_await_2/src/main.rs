use tokio::time::{Duration, sleep};

async fn fn1() {
    println!("Task 1 started!");
    sleep(Duration::from_secs(3)).await;
    println!("Task 1 completed!");
}

async fn fn2() {
    println!("Task 2 started!");
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 completed!");
}

#[tokio::main]
async fn main() {
    let mut r = vec![];
    let m = tokio::spawn(async move {
        fn1().await;
    });
    r.push(m);

    let f = tokio::spawn(async move {
        fn2().await;
    });
    r.push(f);

    for b in r {
        b.await.unwrap();
    }
}
