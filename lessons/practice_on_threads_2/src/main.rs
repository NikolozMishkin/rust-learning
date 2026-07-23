use std::thread;

fn main() {
    let handle_1 = thread::spawn(|| {
        let mut sum = 0;
        let range = 0..=1_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_2 = thread::spawn(|| {
        let mut sum = 0;
        let range = 1_001..=2_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_3 = thread::spawn(|| {
        let mut sum = 0;
        let range = 2_001..=3_000;
        for num in range {
            sum += num;
        }
        sum
    });

    let mut sum = 0;
    sum += handle_1.join().unwrap();
    sum += handle_2.join().unwrap();
    sum += handle_3.join().unwrap();

    println!("Final Summation Result {sum}");
}
