mod m1 {
    struct A {
        d: m2::D,
    }
    pub mod m2 {
        pub enum D {
            B,
            C,
        }
    }
}

mod m3 {
    struct C {
        e: crate::m1::m2::D,
    }
}

fn main() {}
