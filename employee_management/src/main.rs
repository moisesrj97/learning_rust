use std::{collections::HashMap, io};

struct Employee {
    name: String,
    department: String,
}

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee_to_department(&mut self, employee: Employee) {
        if self.departments.get(&employee.department).is_none() {
            self.departments
                .insert(employee.department.clone(), Vec::new());
        };

        let department = self
            .departments
            .get_mut(&employee.department)
            .expect("Department does not exist");

        department.push(employee.name);
    }

    fn get_department_employees(&mut self, department_name: String) -> Vec<String> {
        let department = self
            .departments
            .get(&department_name)
            .expect("Department does not exist");
        let mut department_clone = department.clone();

        department_clone.sort();

        department_clone
    }

    fn get_all_employees(&mut self) -> Vec<String> {
        let mut result = Vec::new();

        for department in self.departments.iter() {
            for employee in department.1 {
                result.push(employee.clone());
            }
        }

        result.sort();

        result
    }
}

fn main() {
    let mut company = Company::new();
    let mut close = false;

    println!("Hello, welcome to the employee management software!");
    while !close {
        println!("Please, select the action you want to perform");
        println!("1. Add employee to department");
        println!("2. Retrieve employees from a specific department");
        println!("3. Retrieve all employees");
        println!("4. Close");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line!");

        let numeric_option: i32 = option.trim().parse().unwrap_or(0);

        match numeric_option {
            1 => {
                let mut name = String::new();
                let mut department = String::new();
                println!("Please introduce name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line!");
                println!("Please introduce department:");
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line!");

                let new_employee = Employee { department, name };

                company.add_employee_to_department(new_employee);

                println!("Successfully added employee")
            }
            2 => {
                let mut department = String::new();
                println!("Please introduce department:");
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line!");
                let employees = company.get_department_employees(department);
                println!("List of employees:");
                println!("{:?}", employees);
            }
            3 => {
                let employees = company.get_all_employees();
                println!("List of employees:");
                println!("{:?}", employees);
            }
            4 => {
                close = true;
            }
            _ => println!("That option does not exist, please try again"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn create_department_should_create_department() {
        let mut company = Company::new();
        let department_name = String::from("test");

        company.create_department(department_name);

        let expected_departments = HashMap::from([(String::from("test"), Vec::new())]);
        assert_eq!(company.departments, expected_departments)
    }

    #[test]
    fn create_department_should_not_overwrite_existing_department() {
        let mut company = Company::new();
        company.departments.insert(
            String::from("test"),
            Vec::from([String::from("test_employee")]),
        );

        let expected_departments = company.departments.clone();
        let department_name = String::from("test");

        company.create_department(department_name);

        assert_eq!(company.departments, expected_departments)
    }

    #[test]
    fn add_employee_to_department_should_add_an_employee_to_a_department() {
        let mut company = Company::new();
        company
            .departments
            .insert(String::from("department_name"), Vec::new());

        let employee: Employee = Employee {
            name: String::from("test_name"),
            department: String::from("department_name"),
        };

        company.add_employee_to_department(employee);

        let employees = company
            .departments
            .get(&String::from("department_name"))
            .unwrap();
        let expected_employees = Vec::from([String::from("test_name")]);
        assert_eq!(*employees, expected_employees);
    }

    #[test]
    fn get_department_employees_should_return_department_employees_ordered_alphabetically() {
        let mut company = Company::new();
        company.departments.insert(
            String::from("test"),
            Vec::from([String::from("c"), String::from("b"), String::from("a")]),
        );

        let department_name = String::from("test");

        let employees = company.get_department_employees(department_name);

        let expected_employees =
            Vec::from([String::from("a"), String::from("b"), String::from("c")]);
        assert_eq!(employees, expected_employees)
    }

    #[test]
    fn get_company_employees_should_return_all_employees_ordered_alphabetically() {
        let mut company = Company::new();

        company
            .departments
            .insert(String::from("test"), Vec::from([String::from("c")]));

        company.departments.insert(
            String::from("test2"),
            Vec::from([String::from("b"), String::from("a")]),
        );

        let employees = company.get_all_employees();

        let expected_employees =
            Vec::from([String::from("a"), String::from("b"), String::from("c")]);
        assert_eq!(employees, expected_employees)
    }
}
