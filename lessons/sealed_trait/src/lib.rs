pub mod my_module {
    use private_mod::Sealed;

    pub trait SomeTrait: Sealed {
        fn some_fn(&self);
    }

    pub trait NonSealedTrait {
        fn open_for_implementation();
    }
    mod private_mod {
        pub trait Sealed {}
    }

    pub struct MyType1;
    impl SomeTrait for MyType1 {
        fn some_fn(&self) {
            println!("You can access me");
        }
    }

    impl Sealed for MyType1 {}
}
