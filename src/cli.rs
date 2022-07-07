pub mod users {
  use std::io::{self, BufRead};

  pub fn get_input_text() -> String {
      let stdin = io::stdin();
      let mut iterator = stdin.lock().lines();
      let text = iterator.next().unwrap().unwrap();
      text
  }

  pub fn handle_users(mut users: Vec<String>) {
      let text = get_input_text();
      if text == "exit".to_string() {
          println!("Bye bye!");
          return
      }
      if text == "show".to_string() {
          println!("{:?}", users);
      } else {
          let text_divided = text.split(" ").collect::<Vec<&str>>();
          if text_divided[0].to_lowercase() != "add" || text_divided[2].to_lowercase() != "to" {
              println!("Wrong text formatting");
          } else {
              users.push(text_divided[1].to_string().to_lowercase());
              println!("Ok!");
          }
      }
      handle_users(users);
  }
}