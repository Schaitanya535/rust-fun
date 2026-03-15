use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct Kid;

#[derive(Debug, Clone)]
pub struct Adult {
    pub can_drive: bool,
}

#[derive(Debug, Clone)]
pub struct Person<State> {
    name: String,
    age: i8,
    status: State,
}

impl Person<Kid> {
    pub fn new(name: String, age: i8) -> Person<Kid> {
        Person::<Kid> {
            name: name,
            age: age,
            status: Kid,
        }
    }

    fn can_transform_to_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn transform_to_adult(self) -> Result<Person<Adult>> {
        if !self.can_transform_to_adult() {
            bail!("Too young to be an adult");
        }

        Ok(Person::<Adult> {
            status: Adult { can_drive: false },
            name: self.name,
            age: self.age,
        })
    }
}

impl Person<Adult> {
    pub fn get_driving_license(&mut self) {
        self.status.can_drive = true
    }
}
