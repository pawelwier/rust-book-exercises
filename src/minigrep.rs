pub mod minigrep {
    use std::{ env::{self, args}, fs, process };

    pub fn get_bin_name() -> () {
        let args: Vec<String> = env::args().collect();
        let path: Vec<&str> = args[0].split("/").collect();
        println!("{}", path[path.len() - 1].to_string());
    }
    
    
    pub struct ParseArgs {
        query: String,
        file_name: String
    }

    impl ParseArgs {
        pub fn new(args: &[String]) -> Result<ParseArgs, &'static str> {
            if !validate_args(args) {
                exit();
            }
            if let [_, query, file_name] = &args[..] {
                Ok(ParseArgs { 
                    query: query.to_string(),
                    file_name: file_name.to_string()
                })
            } else {
                panic!("Invalid args");
            }
        }
    }

    fn exit() {
        process::exit(1);
    }

    fn is_text_file(file_name: String) -> bool {
        let file_name_parts: Vec<&str> = file_name.split(".").collect();
        let extension = file_name_parts.last().unwrap();
        extension.to_string() == "txt"
    }

    fn validate_args(args: &[String]) -> bool {
        if args.len() != 3 {
            println!("Invalid argument length. Need 3, got {}", args.len());
            return false
        }
        if !is_text_file(args[2].to_string()) {
            println!("{}", "Second argument needs to be a .txt file");
            return false
        }
        true
    }

    fn get_args() -> ParseArgs {
        let args: Vec<String> = env::args().collect();
        ParseArgs::new(&args).unwrap()
    }

    fn get_file_text(file_name: &str) -> String {
        fs::read_to_string(format!("src/{}", file_name))
            .expect("File reading error")
    }

    fn text_contains_slice(text: &str, slice: &str) -> bool {
        text.to_lowercase().contains(&slice.to_lowercase())
    }

    fn get_text_file_lines(text: &str) -> Vec<&str> {
        let split_text: Vec<&str> = text.split("\n").collect();
        split_text
    }

    fn find_lines_with_query(lines: Vec<&str>, query: &str) -> Vec<String> {
        lines.iter().map(|line| line.to_string()).filter(|line| text_contains_slice(line, query)).collect()
    }

    fn print_message(query: &str, lines: Vec<String>) -> () {
        println!("{} lines contain(s) \"{}\":", lines.len(), query);

        for line in lines {
            println!("{}", line)
        }
    }
    
    pub fn get_lines_with_query() {
        let args = get_args();
        let text = get_file_text(&args.file_name);
        let lines = get_text_file_lines(&text);
        let lines_with_query = find_lines_with_query(lines, &args.query);
        print_message(&args.query, lines_with_query);
    }

    #[cfg(test)]
    mod minigrep_tests {
        use super::{is_text_file, validate_args, text_contains_slice};

        #[test]
        fn text_file_incorrect() {
            let file_name = "image.jpeg";
            assert_eq!(is_text_file(file_name.to_string()), false);
        }
    
        #[test]
        fn text_file_correct() {
            let file_name = "some-text.txt";
            assert!(is_text_file(file_name.to_string()));
        }

        #[test]
        fn create_parse_args() {
            let args: [String; 3] = ["skip".to_string(), "only_one".to_string(), "sdf.txt".to_string()];
            assert!(validate_args(&args));
        }

        #[test]
        fn contains_substring() {
            let text = "A sample text.";
            let slice = "sample";
            assert!(text_contains_slice(text, slice));
        }

        #[test]
        fn does_not_contain_substring() {
            let text = "A sample text again.";
            let slice = "missing";
            assert!(!text_contains_slice(text, slice));
        }
    }
}