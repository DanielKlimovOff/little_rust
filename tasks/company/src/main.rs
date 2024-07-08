use std::io;
use company;

fn main() {
    let company = company::Company::new("Mango");

    loop {
        let mut choise = String::new();

        io::stdin()
            .read_line(&mut choise)
            .expect("Error input choise");
        choise = choise.trim().to_lowercase();
        let words:Vec<&str> = choise.split_whitespace().collect();

        println!("{words:?}");

        match words[0] {
            "create" => {
                match words[1] {
                    "unit" => {
                        let unit_name = words[2];
                        let mut unit_exist = false;
                        for unit in &company.units {
                            if unit.name == unit_name {
                                unit_exist = true;
                                break;
                            }
                        }
                        if unit_exist {
                            println!("error, this unit exist");
                        }
                    },
                    _ => println!("nope"),
                }
            },
            "add" => println!("add person to unit"),
            "exit" => break,
            _ => println!("nope"),
        }
    }
}
