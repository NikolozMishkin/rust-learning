struct A {
    b: B,
    c: C,
}

struct B {
    f2: u32,
}

struct C {
    f1: u32,
    f3: u32,
}

fn fn1(a: &mut B) -> &u32 {
    &a.f2
}

fn fn2(a: &mut C) -> u32 {
    a.f1 + a.f3
}

fn fn3(a: &mut A) {
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);
    println!("{x}");
}

fn main() {}
