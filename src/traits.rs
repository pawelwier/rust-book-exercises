pub mod traits {
    pub trait Summary {
        fn get_summary(&self) -> String {
            String::from("Default summary")
        }
    }

    pub struct Article {
        pub author: String,
        pub page: i32,
        pub topic: String,
        pub content: String
    }
    
    pub struct Tweet {
        pub author: String,
        pub content: String
    }

    pub struct Book {
        pub title: String,
        pub author: String,
        pub pages: i32
    }

    impl Summary for Article {
        fn get_summary(&self) -> String {
            format!("about {} on page {}, by {}", self.topic, self.page, self.author)
        }
    }

    impl Summary for Tweet {
        fn get_summary(&self) -> String {
            format!("{}: {}", self.author, self.content)
        }
    }

    impl Summary for Book {}

    // also:
    // pub fn summarize(item: &impl Summary) -> String {
    // pub fn summarize<T: Summary>(item: &T) -> String {
    pub fn summarize<T>(item: &T) -> String 
        where T: Summary
    {
        format!("Here's your summary: {}", item.get_summary())
    }

    // also:
    // pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    pub fn largest<T>(items: &[T]) -> T 
        where T: PartialOrd + Copy
    {
        let mut largest = items[0];
        for &item in items {
            if item > largest {
                largest = item
            }
        }
        largest
    }
}