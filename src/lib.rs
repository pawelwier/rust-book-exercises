#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    struct Rectangle {
        width: i32,
        height: i32
    }
    impl Rectangle {
        pub fn can_hold(&self, rect2: &Rectangle) -> bool {
            self.width >= rect2.width && self.height >= rect2.height 
        }
    }

    #[test]
    fn larger_holds_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 4
        };

        let smaller = Rectangle {
            width: 5,
            height: 2
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    fn greeting(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Kret");
        assert!(
            result.contains("Kret"),
            "Failed, did not contain provided name, value was: {}.",
            result
        );
    }

    pub struct Guess {
        value: i32
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Value out of range")
            }
            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected = "Value out of range")]
    fn guess_out_of_range() {
        Guess::new(107);
    }

    pub fn multiply_by_two(num: i32) -> i32 {
        num * 2
    }
    
    #[test]
    fn check_multiply_by_two() -> Result<(), String> {
        if multiply_by_two(10) == 20 {
            Ok(())
        } else {
            Err(String::from("Invalid result"))
        }
    }

    #[test]
    #[ignore]
    fn ignore_this_test() {
        assert_eq!(4, 2 + 2);
    }
}