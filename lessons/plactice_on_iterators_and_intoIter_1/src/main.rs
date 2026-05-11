struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let result = Some(self.current);
            self.current += 1;
            result
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(3);
    assert!(matches!(counter.next(), Some(0)));
    assert!(matches!(counter.next(), Some(1)));
    assert!(matches!(counter.next(), Some(2)));
    assert!(matches!(counter.next(), None));
}
