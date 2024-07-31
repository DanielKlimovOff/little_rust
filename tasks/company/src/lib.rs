use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    people: Vec<String>,
}

impl Unit {
    pub fn new(name: &str) -> Unit {
        Unit { name: String::from(name), people: Vec::new() }
    }

    pub fn count_people(&self) -> usize {
        self.people.len()
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn add_person(&mut self, person_name: &str) {
        self.people.push(String::from(person_name));
    }

    pub fn remove_person(&mut self, person_name: &str) -> Result<(), &'static str> {
        let mut index: usize = 0;
        for person in &self.people {
            if person == person_name {
                break;
            }
            index += 1;
        }
        if index == self.count_people() {
            return Err("Can not find person");
        }
        self.people.remove(index);
        Ok(())
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit '{}' ({}): {:?}", self.name, self.count_people(), self.people)
    }
}

#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub units: Vec<Unit>,
}

impl Company {
    pub fn new(name: &str) -> Company { 
        Company { name: String::from(name), units: Vec::new() as Vec<Unit> }
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

    pub fn rename_unit(&mut self, old_unit_name: &str, new_unit_name: &str) -> Result<(), &'static str> {
        let target_mut_unit = self.get_mut_unit_by_name(old_unit_name)?;
        target_mut_unit.rename(new_unit_name);
        Ok(())
    }

    pub fn remove_unit(&mut self, unit_name: &str) -> Result<(), &'static str> {
        let mut index: usize = 0;
        for unit in &self.units {
            if unit.name == unit_name {
                break;
            }
            index += 1;
        }
        if index == self.count_units() {
            return Err("Can not find unit");
        }
        self.units.remove(index);
        Ok(())
    }

    pub fn count_units(&self) -> usize {
        self.units.len()
    }

    pub fn list_of_units(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        for unit in &self.units {
            result.push(&unit.name);
        }
        result
    }

    pub fn count_people_in_unit(&self, unit_name: &str) -> Result<usize, &'static str> {
        let target_unit = self.get_unit_by_name(unit_name)?;
        Ok(target_unit.count_people())
    }

    pub fn count_people(&self) -> usize {
        let mut result: usize = 0;
        for unit in &self.units {
            result += unit.count_people();
        }
        result
    }

    pub fn add_unit(&mut self, unit_name: &str) {
        self.units.push(Unit::new(unit_name));
    }

    pub fn add_person_to_unit(&mut self, unit_name: &str, person_name: &str) -> Result<(), &'static str> {
        let target_mut_unit = self.get_mut_unit_by_name(unit_name)?;
        target_mut_unit.add_person(person_name);
        Ok(())
    }

    pub fn list_of_people_in_unit(&self, unit_name: &str) -> Result<Vec<&str>, &'static str> {
        let target_unit = self.get_unit_by_name(unit_name)?;
        let mut result: Vec<&str> = Vec::new();
        for person in &target_unit.people {
            result.push(person);
        }
        Ok(result)
    }

    pub fn remove_person(&mut self, unit_name: &str, person_name: &str) -> Result<(), &'static str> {
        let target_mut_unit = self.get_mut_unit_by_name(unit_name)?;
        target_mut_unit.remove_person(person_name)?;
        Ok(())
    }

    pub fn transfer_person(&mut self, old_unit_name: &str, person_name: &str, new_unit_name: &str) -> Result<(), &'static str> {
        self.remove_person(old_unit_name, person_name)?;
        self.add_person_to_unit(new_unit_name, person_name)?;
        Ok(())
    }

    /// This is huge kostil'. TODO change function to return Option<&str> and complete all tests 
    pub fn find_person(&self, person_name: &str) -> Option<String> {
        for unit in &self.units {
            if unit.people.contains(&String::from(person_name)) {
                return Some(unit.name.clone());
            }
        }
        None
    }

    fn get_unit_by_name(&self, unit_name: &str) -> Result<&Unit, &'static str>{
        for unit in &self.units {
            if unit.name == unit_name {
                return Ok(unit);
            }
        }
        Err("Can not find unit")
    }

    fn get_mut_unit_by_name(&mut self, unit_name: &str) -> Result<&mut Unit, &'static str>{
        for unit in &mut self.units {
            if unit.name == unit_name {
                return Ok(unit);
            }
        }
        Err("Can not find unit")
    }
}

impl Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Company '{}' ({}): {:?}", self.name, self.count_people(), self.list_of_units())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_company_with_name() {
        let capital_c = Company::new("Apple");

        assert_eq!(&capital_c.name, "Apple");
    }

    #[test]
    fn rename_company() {
        let mut capital_c = Company::new("Banana");
        capital_c.rename("Orange");

        assert_eq!(&capital_c.name, "Orange");
    }

    #[test]
    fn count_units_zero() {
        let capital_c = Company::new("Peach");

        assert_eq!(capital_c.count_units(), 0);
    }

    #[test]
    fn add_unit_with_name() {
        let mut capital_c = Company::new("Pineapple");
        capital_c.add_unit("Workers");
        capital_c.add_unit("Developers");
        capital_c.add_unit("Engineers");

        assert_eq!(capital_c.count_units(), 3);
        assert_eq!(capital_c.list_of_units(),
            vec!["Workers", "Developers", "Engineers", ]);
    }

    #[test]
    fn rename_unit() {
        let mut capital_c = Company::new("Watermelon");
        capital_c.add_unit("Developers");
        capital_c.add_unit("Workers");
        capital_c.rename_unit("Developers", "Engineers").unwrap();

        assert_eq!(capital_c.count_units(), 2);
        assert_eq!(capital_c.list_of_units(),
            vec!["Engineers", "Workers", ])
    }

    #[test]
    fn rename_unit_false() {
        let mut capital_c = Company::new("Pear");
        capital_c.add_unit("Developers");
        capital_c.add_unit("Workers");
        
        match capital_c.rename_unit("Unit name", "Engineers") {
            Ok(_) => panic!("Find and rename fake unit"),
            Err(message) => assert_eq!(message, "Can not find unit"),
        }
    }

    #[test]
    fn remove_unit() {
        let mut capital_c = Company::new("Lime");
        capital_c.add_unit("Workers");
        capital_c.add_unit("Engineers");
        capital_c.add_unit("Developers");

        assert_eq!(capital_c.count_units(), 3);

        capital_c.remove_unit("Engineers").unwrap();

        assert_eq!(capital_c.count_units(), 2);

    }

    #[test]
    fn add_person_to_unit() {
        let mut capital_c = Company::new("Grape");
        capital_c.add_unit("Workers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Workers", "Nikita").unwrap();
        
        let count_people = capital_c.count_people_in_unit("Workers").unwrap();
        assert_eq!(count_people, 2);
    }

    #[test]
    fn list_of_people() {
        let mut capital_c = Company::new("Grape");
        capital_c.add_unit("Workers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Workers", "Nikita").unwrap();
        
        assert_eq!(capital_c.list_of_people_in_unit("Workers").unwrap(), vec!["Daniel", "Nikita", ]);
    }

    #[test]
    fn count_people_in_company() {
        let mut capital_c = Company::new("Mango");
        capital_c.add_unit("Workers");
        capital_c.add_unit("Engineers");
        capital_c.add_unit("Developers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Workers", "Nikita").unwrap();
        capital_c.add_person_to_unit("Engineers", "Luna").unwrap();
        capital_c.add_person_to_unit("Engineers", "Anna").unwrap();
        capital_c.add_person_to_unit("Engineers", "Maiden").unwrap();
        capital_c.add_person_to_unit("Developers", "Ivan").unwrap();
        
        let count_people = capital_c.count_people();
        assert_eq!(count_people, 6);
        let count_engineers = capital_c.count_people_in_unit("Engineers").unwrap();
        assert_eq!(count_engineers, 3);
    }

    #[test]
    fn remove_person() {
        let mut capital_c = Company::new("Kiwi");
        capital_c.add_unit("Workers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Workers", "Nikita").unwrap();
        capital_c.add_person_to_unit("Workers", "Denis").unwrap();
        capital_c.add_person_to_unit("Workers", "Misha").unwrap();

        let count_people = capital_c.count_people_in_unit("Workers").unwrap();
        assert_eq!(count_people, 4);

        capital_c.remove_person("Workers", "Misha").unwrap();
        capital_c.remove_person("Workers", "Nikita").unwrap();

        let count_people = capital_c.count_people_in_unit("Workers").unwrap();
        assert_eq!(count_people, 2);
    }

    #[test]
    fn find_person() {
        let mut capital_c = Company::new("Melon");
        capital_c.add_unit("Workers");
        capital_c.add_unit("Engineers");
        capital_c.add_unit("Developers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Engineers", "Nikita").unwrap();
        capital_c.add_person_to_unit("Developers", "SEREGA").unwrap();

        match capital_c.find_person("SEREGA") {
            Some(unit_name) => assert_eq!(unit_name, "Developers"),
            None => panic!("Fail test find person")
        }
    }

    #[test]
    fn transfer_person() {
        let mut capital_c = Company::new("Lemon");
        capital_c.add_unit("Workers");
        capital_c.add_unit("Developers");
        capital_c.add_person_to_unit("Workers", "Daniel").unwrap();
        capital_c.add_person_to_unit("Workers", "Nikita").unwrap();
        capital_c.add_person_to_unit("Developers", "Alex").unwrap();
        
        let count_people = capital_c.count_people();
        assert_eq!(count_people, 3);
        let count_developers = capital_c.count_people_in_unit("Developers").unwrap();
        assert_eq!(count_developers, 1);

        capital_c.transfer_person("Workers", "Daniel", "Developers").unwrap();

        let count_people = capital_c.count_people();
        assert_eq!(count_people, 3);
        let count_developers = capital_c.count_people_in_unit("Developers").unwrap();
        assert_eq!(count_developers, 2);
    }
}