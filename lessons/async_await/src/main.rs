async fn printing() {
    println!("I am async function");
}

// #[tokio::main]
// async fn main() {
//     let x = printing();

//     println!("The future has not been polled yet");
//     drop(x);
//     //x.await;
// }

fn main() {
    use tokio::runtime::Builder;
    let my_str = "Hello, World!".to_string();
    let my_clogure = async || {
        let x = printing();

        println!("The future has not been polled yet");
        x.await;
        let my_str1 = my_str;
        println!("{}", my_str1);
    };
    let body = async {
        let x = printing().await;
        let my_clogure_feature = my_clogure();
        println!(
            "my_clogure_feature size_of_val: {}",
            size_of_val(&my_clogure_feature)
        );
        println!("The future has not been polled yet");
        // drop(x);
        // x.await;
    };

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body);
}
