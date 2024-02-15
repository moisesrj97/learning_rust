use std::collections::HashMap;

struct Employee {
    name: String,
    department: String,
}

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        return Company {
            departments: HashMap::new(),
        };
    }
    fn create_department(&mut self, department_name: String) {
        if self.departments.get(&department_name).is_some() {
            return;
        };

        self.departments.insert(department_name, Vec::new());
    }

    fn add_employee_to_department(&mut self, employee: Employee) {
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
    println!("Hello, world!");
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
