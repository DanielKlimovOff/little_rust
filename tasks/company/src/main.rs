use std::io;
use company;

fn main() {
    let mut company = company::Company::new("Mango");
    let mut debug_flag = true;

    loop {
        if debug_flag {
            println!("### {}", company);
            for unit in &company.units {
                println!("##### {}", unit);
            }
        }

        println!("Enter command:");
        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Fail to read command(");
        let choise: u32 = match choise.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Command should be a number, try again");
                continue;
            },
        };

        match choise {
            0 => break,
            1 => {
                println!("Enter new name:");
                let mut company_name = String::new();
                io::stdin()
                    .read_line(&mut company_name)
                    .expect("Fail to read company name(");
                company_name = String::from(company_name.trim());

                company.rename(company_name.trim());
                println!("New company name is '{}'", company.name);
                
            },
            2 => {
                println!("Enter name for new unit:");
                let mut unit_name = String::new();
                io::stdin()
                    .read_line(&mut unit_name)
                    .expect("Fail to read first unit name(");
                unit_name = String::from(unit_name.trim());

                company.add_unit(&unit_name);
                println!("Add new unit with name '{}'", unit_name);
            },
            3 => {
                println!("Enter unit name which you whant to rename:");
                let mut old_unit_name = String::new();
                io::stdin()
                    .read_line(&mut old_unit_name)
                    .expect("Fail to read old unit name(");
                old_unit_name = String::from(old_unit_name.trim());

                println!("Enter new unit name:");
                let mut new_unit_name = String::new();
                io::stdin()
                    .read_line(&mut new_unit_name)
                    .expect("Fail to read new unit name(");
                new_unit_name = String::from(new_unit_name.trim());

                match company.rename_unit(&old_unit_name, &new_unit_name) {
                    Ok(_) => println!("Rename unit '{}' to '{}'", old_unit_name, new_unit_name),
                    Err(message) => println!("Fail {}, try again", message),
                }
            },
            4 => {
                println!("Enter unit name which you whant to remove:");
                let mut unit_name = String::new();
                io::stdin()
                    .read_line(&mut unit_name)
                    .expect("Fail to read unit name(");
                unit_name = String::from(unit_name.trim());

                match company.remove_unit(&unit_name) {
                    Ok(_) => println!("Remove unit '{}'", unit_name),
                    Err(message) => println!("Fail {}, try again", message),
                }
            },
            5 => {
                println!("Enter unit name which add new person:");
                let mut unit_name = String::new();
                io::stdin()
                    .read_line(&mut unit_name)
                    .expect("Fail to read unit name(");
                unit_name = String::from(unit_name.trim());

                println!("Enter person name:");
                let mut person_name = String::new();
                io::stdin()
                    .read_line(&mut person_name)
                    .expect("Fail to read person name(");
                person_name = String::from(person_name.trim());

                match company.add_person_to_unit(&unit_name, &person_name) {
                    Ok(_) => println!("Add new person '{}' to unit '{}'", person_name, unit_name),
                    Err(message) => println!("Fail {}, try again", message),
                }
            },
            6 => {
                println!("Enter unit name which remove person:");
                let mut unit_name = String::new();
                io::stdin()
                    .read_line(&mut unit_name)
                    .expect("Fail to read unit name(");
                unit_name = String::from(unit_name.trim());

                println!("Enter person name:");
                let mut person_name = String::new();
                io::stdin()
                    .read_line(&mut person_name)
                    .expect("Fail to read person name(");
                person_name = String::from(person_name.trim());

                match company.remove_person(&unit_name, &person_name) {
                    Ok(_) => println!("Remove person '{}' from unit '{}'", person_name, unit_name),
                    Err(message) => println!("Fail {}, try again", message),
                }
            },
            7 => {
                println!("Enter unit name which have person to transfer:");
                let mut old_unit_name = String::new();
                io::stdin()
                    .read_line(&mut old_unit_name)
                    .expect("Fail to read unit name(");
                old_unit_name = String::from(old_unit_name.trim());

                println!("Enter person name:");
                let mut person_name = String::new();
                io::stdin()
                    .read_line(&mut person_name)
                    .expect("Fail to read person name(");
                person_name = String::from(person_name.trim());

                println!("Enter unit name:");
                let mut new_unit_name = String::new();
                io::stdin()
                    .read_line(&mut new_unit_name)
                    .expect("Fail to read unit name(");
                new_unit_name = String::from(new_unit_name.trim());

                match company.transfer_person(&old_unit_name, &person_name, &new_unit_name) {
                    Ok(_) => println!("Transfer person '{}' to unit '{}'", person_name, new_unit_name),
                    Err(message) => println!("Fail {}, try again", message),
                }
            },
            8 => {
                println!("Enter person name:");
                let mut person_name = String::new();
                io::stdin()
                    .read_line(&mut person_name)
                    .expect("Fail to read person name(");
                person_name = String::from(person_name.trim());

                match company.find_person(&person_name){
                    Some(unit_name) => println!("Find person '{}' in unit '{}'", &person_name, unit_name),
                    None => println!("Fail person not found, try again"),
                }
            },
            9 => {
                println!("Switch debug mode");
                debug_flag = !debug_flag;
            }
            _ => println!("You enter wrong command, try again"),
        }
    }

    println!("Goodbye!");
}
