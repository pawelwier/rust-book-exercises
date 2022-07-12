pub mod text_forms {
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
}