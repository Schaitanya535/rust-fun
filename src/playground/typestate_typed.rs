use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PersonError {
    #[error("Too young: age {age}, need 18")]
    TooYoung { age: u8 },

    #[error("Insufficient skill: {skill}, need 50")]
    InsufficientSkill { skill: u8 },
}

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

    pub fn transform_to_adult(self) -> Result<Person<Adult>, PersonError> {
        if self.age < 18 {
            return Err(PersonError::TooYoung { age: self.age });
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

    pub fn get_driving_license(&mut self) -> Result<(), PersonError> {
        if self.status.driving_skill < 50 {
            return Err(PersonError::InsufficientSkill {
                skill: self.status.driving_skill,
            });
        }
        self.status.licensed = true;
        Ok(())
    }
}

pub fn run() -> anyhow::Result<()> {
    let mut person = match Person::new("Chaitanya".to_string(), 19).transform_to_adult() {
        Ok(adult) => adult,
        Err(PersonError::TooYoung { age }) => {
            println!("Too young: {}", age);
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    while let Err(e) = person.get_driving_license() {
        match e {
            PersonError::InsufficientSkill { skill } => {
                println!("Skill too low ({}), keep practicing...", skill);
                person.gain_driving_skill();
            }
            e => return Err(e.into()),
        }
    }
    dbg!(&person);
    Ok(())
}
