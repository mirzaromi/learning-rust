use std::collections::HashMap;

fn main() {

    let mut vec = vec!["Add Sally To Engineering", "Add Amir To Sales", "Add Sally To Sales"];

    let employee_depertment = add_and_get_employee_department(&mut vec);
    println!("{:?}", employee_depertment);
}

fn add_and_get_employee_department(vec: &mut Vec<&str>) -> Vec<(String, String)> {
    let mut company = HashMap::new();

    for value in  vec {
        let temp_string = value.to_string();
        let new_value: Vec<&str> = temp_string.trim().split_whitespace().collect();
        let mut name = "";
        let mut department = "";
        let mut is_name_field = false;
        let mut is_company_field = false;
        for word in new_value {
            if word == "Add" {
                is_name_field = true
            }

            if is_name_field {
                name = word;
            }

            if word == "To" {
                is_company_field = true
            }

            if is_company_field {
                department = word;
            }

            if word != "Add" {
                is_name_field = false
            }

            if word != "To" {
                is_company_field = false
            }
        }

        company.entry(String::from(name)).or_insert(String::from(department));
    }

    let mut all_data: Vec<(&String, &String)> = company.iter().collect();
    all_data.sort_by(|a, b| a.0.cmp(&b.0));

    let mut return_data: Vec<(String, String)> = vec![];
    for data in all_data {
        return_data.push((data.0.clone(), data.1.clone()));
    }

    return_data
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_management() {
        // Add employees
        let mut vec = vec!["Add Sally To Engineering",
                                        "Add Amir To Sales",
                                        "Add Sally To Sales",
                                        "Add Zara To Engineering",
                                        "Add Bob To Sales"];




        // Test all employees sorted across departments
        let employee_department = add_and_get_employee_department(&mut vec);
        assert_eq!(
            employee_department,
            vec![
                ("Amir".to_string(), "Sales".to_string()),
                ("Bob".to_string(), "Sales".to_string()),
                ("Sally".to_string(), "Engineering".to_string()),
                ("Zara".to_string(), "Engineering".to_string()),
            ]
        );
    }
}