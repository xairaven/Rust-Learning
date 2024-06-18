/*
Using a hash map and vectors, create a text interface to allow a user
to add employee names to a department in a company. For example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or
all people in the company by department, sorted alphabetically
 */

use std::collections::HashMap;
use std::{io, process};
use itertools::Itertools;

struct Company {
    departments: HashMap<String, Vec<String>>
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new()
        }
    }

    fn add_employee(&mut self, employee: &str, department: &str) {
        let entry = self.departments.entry(department.to_string())
            .or_insert(vec![]);
        entry.push(employee.to_string());
        entry.sort();
    }

    fn get_employees_of_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }

    fn list_employees_of_department(&self, department: Option<&Vec<String>>) -> String {
        match department {
            None => String::from("None"),
            Some(vector) => {
                let mut result = String::new();

                if vector.len() == 0 {
                    return String::from("None");
                }

                for name in vector {
                    result.push_str(format!("{name}\n").as_str());
                }

                result
            }
        }
    }

    fn list_all_employees(&self) -> String {
        let mut result = String::new();

        for department in self.departments.keys().sorted()  {
            let department_vector = self.get_employees_of_department(department);
            let department_string = self.list_employees_of_department(department_vector);

            result.push_str(format!("{department}:\n").as_str());
            for line in department_string.lines() {
                result.push_str(format!("\t{line}\n").as_str());
            }
        }

        result
    }
}

struct CommandExecutor {
    company: Company
}

impl CommandExecutor {
    fn new() -> CommandExecutor {
        CommandExecutor {
            company: Company::new()
        }
    }

    fn start(&mut self) {
        loop {
            print!("Enter a command:\n> ");

            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            let mut words = command.split_whitespace();
            if let Some(first_word) = words.next() {
                let args: Vec<&str> = words.collect();
                match first_word {
                    "Add" => self.add(args),
                    "ListBy" => self.list_by(args),
                    "ListAll" => self.list_all(),
                    "Help" => self.help(),
                    "Exit" => self.exit(),
                    _ => self.display_wrong()
                }
            }
        }
    }

    fn add(&mut self, args: Vec<&str>) {
        if args.len() != 3 {
            self.display_wrong_args();
            return;
        }

        let name;
        let department;

        if let Some(&n) = args.get(0) {
            name = n;
        } else {
            self.display_wrong_args();
            return;
        }

        if let Some(&d) = args.get(2) {
            department = d;
        } else {
            self.display_wrong_args();
            return;
        }

        self.company.add_employee(name, department);
        println!("Success!");
    }

    fn list_by(&self, args: Vec<&str>) {
        if args.len() != 1 {
            self.display_wrong_args();
            return;
        }

        if let Some(&department) = args.get(0) {
            let department = self.company
                .get_employees_of_department(department);
            let result = self.company.list_employees_of_department(department);
            println!("Result:\n{result}");
        } else {
            self.display_wrong_args();
            return;
        }
    }

    fn list_all(&self) {
        let result = self.company.list_all_employees();
        println!("Result:\n{result}");
    }

    fn help(&self) {
        let help_text = r#"
Available commands:
    Add <Name> to <Department>     - Adds an employee to the specified department.
                                   Example: Add Sally to Engineering
    ListBy <Department>            - Lists all employees in the specified department, sorted alphabetically.
                                   Example: List by Engineering
    ListAll                        - Lists all employees in the company, sorted alphabetically by department.
                                   Example: List All
    Help                           - Displays this help section.
    Exit                           - Exits the program.
        "#;
        println!("{}", help_text);
    }

    fn display_wrong(&self) {
        println!("Unknown command. Please use 'Help' to see available commands.");
    }

    fn display_wrong_args(&self) {
        println!("There are problems with args. Please use 'Help' to see available commands.");
    }

    fn exit(&self) {
        println!("Bye!");
        process::exit(0);
    }
}

fn main() {
    let mut executor = CommandExecutor::new();
    executor.start();
}

/*
Add Alex to Engineering
Add Bob to Engineering
Add Charlie to Sales
Add Ethan to HR
Add Fiona to Support
 */