// async fn printing() {
//     println!("I am async function");
// }

// #[tokio::main]
// async fn main() {
//     let x = printing();

//     println!("The future has not been polled yet");
//     drop(x);
//     //x.await;
// }

// fn main() {
//     use tokio::runtime::Builder;
//     let my_str = "Hello, World!".to_string();
//     let my_clogure = async || {
//         let x = printing();

//         println!("The future has not been polled yet");
//         x.await;
//         let my_str1 = my_str;
//         println!("{}", my_str1);
//     };
//     let body = async {
//         let x = printing().await;
//         let my_clogure_feature = my_clogure();
//         println!(
//             "my_clogure_feature size_of_val: {}",
//             size_of_val(&my_clogure_feature)
//         );
//         println!("The future has not been polled yet");
//         // drop(x);
//         // x.await;
//     };

//     Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .expect("Failed building the Runtime")
//         .block_on(body);
// }

use std::time::Duration;
use tokio::time::sleep;

// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("11 Task {i} printing, first time");
            printing(i).await;
            println!("13 Task {i}, printing, second time");
            printing(i).await;
            println!("15 Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        println!("20 Handle await");
        handle.await.unwrap();
    }
}

async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("27 Task {i}")
}

async fn my_fn() -> i32 {
    5
}
