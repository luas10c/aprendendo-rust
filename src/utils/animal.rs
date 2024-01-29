
#[derive(Debug)]
pub struct Animal {
  pub id: String,
  pub name: String,
  pub health: u32
}

impl Animal {
  pub fn new(id: String, name: String, health: u32) -> Self {
    Self {
      id,
      name,
      health
    }
  }

  pub fn eat(&self) {
    let health = self.health * 100 / 100;
    println!("{}", health);
  }
}