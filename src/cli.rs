pub mod users {
    use std::io::{self, BufRead};

    #[derive(Debug)]
    pub struct Employee {
        name: String,
        department: String
    }

    impl Employee {
        fn new(name: String, department: String) -> Employee {
            Employee {
                name,
                department
            }
        }

        fn display_user(&self) -> () {
            println!("user {} from {}", self.name, self.department);
        }
    }

    fn get_input_text() -> String {
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        let text = iterator.next().unwrap().unwrap();
        text
    }

    pub fn handle_users(mut users: Vec<Employee>) {
        let text = get_input_text();
        if text == "exit".to_string() {
            println!("Bye bye!");
            return
        }
        if text[..4].to_string() == "show" {
            if text == "show all".to_string() {
                println!("all users:");
                for user in &users {
                    user.display_user();
                }
            } else {
                if let [_, inserted_department] = text.split(" ").collect::<Vec<&str>>()[..] {
                    let department_users: Vec<&Employee> = users.iter().filter(|user| user.department == inserted_department).collect();
                    println!("users from {}:", inserted_department);
                    for user in department_users {
                        println!("- {}", user.name);
                    }
                }
            }
        } else {
            if let [first, second, third, fourth] = text.split(" ").collect::<Vec<&str>>()[..] {
                if first.to_lowercase() == "add" && third.to_lowercase() == "to" {                
                    users.push(Employee::new(second.to_string().to_lowercase(), fourth.to_string().to_lowercase()));
                    println!("user {} added to department {}", second, fourth);
                } else if first.to_lowercase() == "move" && third.to_lowercase() == "to" {
                    let index = users.iter().position(|user| user.name == second).unwrap();
                    users.remove(index);
                    users.push(Employee::new(second.to_string(), fourth.to_string()));
                    for user in &users {
                        user.display_user();
                    }
                } else if first.to_lowercase() == "remove" && second.to_lowercase() == "user" && third.to_lowercase() == "name"  {
                    let index = users.iter().position(|user| user.name == fourth).unwrap();
                    users.remove(index);
                }
            } else {
                println!("Wrong text formatting");
            }
        }
        handle_users(users);
    }
}