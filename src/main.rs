#![allow(dead_code)]

use anyhow::Result;
use crate::playground::typestate::Person;
mod aoc;
mod playground;
mod utils;

fn main() -> Result<()> {
    let person = Person::new("Chaitanya".to_string(), 19);
    dbg!(&person);
    let mut adult_person = person.transform_to_adult()?;
    dbg!(&adult_person);
    adult_person.get_driving_license();
    dbg!(&adult_person);
    Ok(())
}
