#![allow(dead_code)]

use crate::playground::typestate::Person;
use anyhow::Result;
mod aoc;
mod playground;
mod utils;

fn main() -> Result<()> {
    let person = Person::new("Chaitanya".to_string(), 19);
    dbg!(&person);
    let mut person = person.transform_to_adult()?;
    dbg!(&person);

    while person.get_driving_license().is_err() {
        person.gain_driving_skill();
        dbg!(&person);
    }

    dbg!(&person);
    Ok(())
}
