fn main() {
    println!(
        "available_parallelism = {:?}",
        std::thread::available_parallelism()
    );
}
