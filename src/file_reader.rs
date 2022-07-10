pub mod file_reader {
    use std::{fs::File, io::{ErrorKind, Read}};
    
    fn get_text_file(file_name: String) -> Result<File, std::io::Error> {
        let name = format!("{}.txt", file_name);
        let f = File::open(&name).unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                println!("File not found. Created {}", name);
                File::create(&name).unwrap()
            } else {
                panic!("Error reading file");
            }
        });
        Ok(f)
    }
    
    pub fn get_file_contents(file_name: String) -> String {
        let mut file = get_text_file(file_name).expect("Error getting text file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|err| {
            if err.raw_os_error().unwrap() == 9 {
                println!("File is empty");
                0
            } else {
                println!("Error reading file");
                0
            }
        });
        contents
    }
}