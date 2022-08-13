pub mod smart_pointers {
    use std::ops::Deref;
    use crate::List::{ Cons, Nil };

    pub fn make_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    pub fn say_hello(name: &str) -> () {
        println!("Hello, {name}!", );
    }

}