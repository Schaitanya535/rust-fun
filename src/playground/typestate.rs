use anyhow::{bail, Result};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Kid;

#[derive(Debug, Clone)]
pub struct Adult {
    pub driving_skill: u8,
    pub licensed: bool,
}

#[derive(Debug, Clone)]
pub struct Person<State> {
    name: String,
    age: u8,
    status: State,
}

impl Person<Kid> {
    pub fn new(name: String, age: u8) -> Person<Kid> {
        Person::<Kid> {
            name,
            age,
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
            status: Adult {
                driving_skill: 0,
                licensed: false,
            },
            name: self.name,
            age: self.age,
        })
    }
}

impl Person<Adult> {
    pub fn gain_driving_skill(&mut self) {
        let gain = rand::thread_rng().gen_range(1..=20);
        self.status.driving_skill = self.status.driving_skill.saturating_add(gain);
    }

    pub fn get_driving_license(&mut self) -> Result<()> {
        if self.status.driving_skill < 50 {
            bail!("Need a skill of 50 to get a license, current: {}", self.status.driving_skill);
        }
        self.status.licensed = true;
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let person = Person::new("Chaitanya".to_string(), 19);
    dbg!(&person);
    let mut person = person.transform_to_adult()?;
    while person.get_driving_license().is_err() {
        person.gain_driving_skill();
    }
    dbg!(&person);
    Ok(())
}
