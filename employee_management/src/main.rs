use std::collections::HashMap;

struct Employee {
    name: String,
    department: String,
}

fn main() {
    println!("Hello, world!");
}

fn create_department(departments: &mut HashMap<String, Vec<String>>, department_name: String) {
    if departments.get(&department_name).is_some() {
        return;
    };

    departments.insert(department_name, Vec::new());
}

fn add_employee_to_department(employee: Employee, departments: &mut HashMap<String, Vec<String>>) {
    let department = departments
        .get_mut(&employee.department)
        .expect("Department does not exist");

    department.push(employee.name);
}

fn get_department_employees(
    departments: HashMap<String, Vec<String>>,
    department_name: String,
) -> Vec<String> {
    let department = departments
        .get(&department_name)
        .expect("Department does not exist");
    let mut department_clone = department.clone();

    department_clone.sort();

    department_clone
}

fn get_company_employees(departments: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut result = Vec::new();

    for department in departments.iter() {
        for employee in department.1 {
            result.push(employee.clone());
        }
    }

    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn create_department_should_create_department() {
        let mut departments: HashMap<String, Vec<String>> = HashMap::new();
        let department_name = String::from("test");

        create_department(&mut departments, department_name);

        let expected_departments = HashMap::from([(String::from("test"), Vec::new())]);
        assert_eq!(departments, expected_departments)
    }

    #[test]
    fn create_department_should_not_overwrite_existing_department() {
        let mut departments: HashMap<String, Vec<String>> = HashMap::from([(
            String::from("test"),
            Vec::from([String::from("test_employee")]),
        )]);
        let expected_departments = departments.clone();
        let department_name = String::from("test");

        create_department(&mut departments, department_name);

        assert_eq!(departments, expected_departments)
    }

    #[test]
    fn add_employee_to_department_should_add_an_employee_to_a_department() {
        let mut departments: HashMap<String, Vec<String>> =
            HashMap::from([(String::from("department_name"), Vec::new())]);

        let employee: Employee = Employee {
            name: String::from("test_name"),
            department: String::from("department_name"),
        };

        add_employee_to_department(employee, &mut departments);

        let employees = departments.get(&String::from("department_name")).unwrap();
        let expected_employees = Vec::from([String::from("test_name")]);
        assert_eq!(*employees, expected_employees);
    }

    #[test]
    fn get_department_employees_should_return_department_employees_ordered_alphabetically() {
        let departments: HashMap<String, Vec<String>> = HashMap::from([(
            String::from("test"),
            Vec::from([String::from("c"), String::from("b"), String::from("a")]),
        )]);

        let department_name = String::from("test");

        let employees = get_department_employees(departments, department_name);

        let expected_employees =
            Vec::from([String::from("a"), String::from("b"), String::from("c")]);
        assert_eq!(employees, expected_employees)
    }

    #[test]
    fn get_company_employees_should_return_all_employees_ordered_alphabetically() {
        let departments: HashMap<String, Vec<String>> = HashMap::from([
            (String::from("test"), Vec::from([String::from("c")])),
            (
                String::from("test2"),
                Vec::from([String::from("b"), String::from("a")]),
            ),
        ]);

        let employees = get_company_employees(departments);

        let expected_employees =
            Vec::from([String::from("a"), String::from("b"), String::from("c")]);
        assert_eq!(employees, expected_employees)
    }
}
