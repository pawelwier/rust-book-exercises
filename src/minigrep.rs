pub mod minigrep {
    use std::{ env, fs };

    pub fn get_bin_name() -> () {
        let args: Vec<String> = env::args().collect();
        let path: Vec<&str> = args[0].split("/").collect();
        println!("{}", path[path.len() - 1].to_string());
    }

    fn get_args() -> Vec<String> {
        env::args().collect()
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
        if let [_, query, file_name] = &args[..] {
            let text = get_file_text(&file_name);
            let lines = get_text_file_lines(&text);
            let lines_with_query = find_lines_with_query(lines, query);
            print_message(query, lines_with_query);
        } else {
            panic!("Invalid arguments")
        } 
    }
    
}