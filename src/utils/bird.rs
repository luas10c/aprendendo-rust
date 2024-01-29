use std::ops::Deref;

use super::animal::Animal;

#[derive(Debug)]
pub struct Bird {
  base: Animal,

}

impl Bird {
  pub fn new(name: String) -> Self {
    Self {
      base: Animal::new("1111".into(), name, 40),
    }
  }

  pub fn sing(&self) {
    println!("{} cantou!", self.name)
  }
}

impl Deref for Bird {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
      &self.base
    }
}