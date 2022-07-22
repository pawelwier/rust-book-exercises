pub mod minigrep {
    use std::{ env::{self, VarError}, fs, process };

    pub fn get_bin_name() -> () {
        let args: Vec<String> = env::args().collect();
        let path: Vec<&str> = args[0].split("/").collect();
        println!("{}", path[path.len() - 1].to_string());
    }
    
    
    pub struct ParseArgs {
        query: String,
        file_name: String,
        ignore_case: bool
    }

    impl ParseArgs {
        pub fn new(args: &[String]) -> Result<ParseArgs, &'static str> {
            if !validate_args(args) {
                exit();
            }
            if let [_, query, file_name] = &args[..] {
                Ok(ParseArgs { 
                    query: query.to_string(),
                    file_name: file_name.to_string(),
                    ignore_case: is_case_sensitive()
                })
            } else {
                panic!("Invalid args");
            }
        }
    }

    fn is_case_sensitive() -> bool {
        let ignore_case_env: Result<String, VarError> = env::var("IGNORE_CASE");
        match ignore_case_env {
            Ok(res) => res == "1".to_string(),
            Err(_) => false
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

    fn text_contains_slice(text: &str, slice: &str, to_lower: Option<bool>) -> bool {
        let lowercase = to_lower.unwrap_or(false);
        let text_case_check = if lowercase { text.to_lowercase() } else { text.to_string() };
        let slice_case_check = if lowercase { slice.to_lowercase() } else { slice.to_string() };
        text_case_check.contains(&slice_case_check)
    }

    fn get_text_file_lines(text: &str) -> Vec<&str> {
        let split_text: Vec<&str> = text.split("\n").collect();
        split_text
    }
    
    fn find_lines_with_query(lines: Vec<&str>, query: &str, ignore_case: bool) -> Vec<String> {
        lines.iter().map(|line| line.to_string()).filter(|line| text_contains_slice(line, query, Some(ignore_case))).collect()
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
        let lines_with_query = find_lines_with_query(lines, &args.query, args.ignore_case);
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
            assert!(text_contains_slice(text, slice, Some(false)));
        }

        #[test]
        fn does_not_contain_substring() {
            let text = "A sample text again.";
            let slice = "missing";
            assert!(!text_contains_slice(text, slice, Some(true)));
        }
    }
}