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

    fn remove_user(users: &mut Vec<Employee>, user_name: String) -> () {
        let index = users.iter().position(|user| user.name == user_name).unwrap();
        users.remove(index);
    }

    fn add_user(users: &mut Vec<Employee>, user_name: String, department_name: String) -> () {
        users.push(Employee::new(user_name.to_string().to_lowercase(), department_name.to_string().to_lowercase()));
    }

    fn display_all_users(users: &Vec<Employee>) -> () {
        println!("all users:");
        for user in users {
            user.display_user();
        }
    }

    fn display_users_by_dept_name(users: &mut Vec<Employee>, inserted_department: String) -> () {
        let department_users: Vec<&Employee> = users.iter().filter(|user| user.department == inserted_department).collect();
        println!("users from {}:", inserted_department);
        for user in department_users {
            println!("- {}", user.name);
        }
    }

    pub fn handle_users(mut users: Vec<Employee>) {
        let text = get_input_text();
        if text == "exit".to_string() {
            println!("Bye bye!");
            return
        }
        if text[..4].to_string() == "show" {
            if text == "show all".to_string() {
                display_all_users(&users);
            } else if let [_, inserted_department] = text.split(" ").collect::<Vec<&str>>()[..] {
                display_users_by_dept_name(&mut users, inserted_department.to_string());
            }
        } else {
            if let [first, second, third, fourth] = text.split(" ").collect::<Vec<&str>>()[..] {
                if first.to_lowercase() == "add" && third.to_lowercase() == "to" {      
                    add_user(&mut users, second.to_string(), fourth.to_string());
                    println!("user {} added to department {}", second, fourth);
                } else if first.to_lowercase() == "move" && third.to_lowercase() == "to" {
                    remove_user(&mut users, second.to_string());
                    add_user(&mut users, second.to_string(), fourth.to_string());
                    println!("user {} moved to {}", second, fourth)
                } else if first.to_lowercase() == "remove" && second.to_lowercase() == "user" && third.to_lowercase() == "name"  {
                    remove_user(&mut users, fourth.to_string());
                }
            } else {
                println!("Wrong command");
            }
        }
        handle_users(users);
    }
}