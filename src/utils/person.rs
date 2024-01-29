use std::ops::Deref;

use super::animal::Animal;

#[derive(Debug)]
pub struct Person {
    base: Animal,
    pub age: i32,
    pub work: Option<String>
}

impl Person {
    pub fn new(name: String, age: i32, work: Option<String>) -> Self {
        Self {
            base: Animal::new("1000".into(), name, 100),
            age,
            work
        }
    }

    pub fn set_age(&mut self, age: i32) {
        self.age = age
    }
}

impl Deref for Person {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}