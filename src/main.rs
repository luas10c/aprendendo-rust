pub mod utils;

use utils::{person::Person, bird::Bird};

fn create_person() -> Person {
    let person: Person = Person::new("Luciano".into(), 25, Some("Software Engineer".into()));
    person
}

fn create_bird(name: String) -> Bird {
    let bird = Bird::new(name);
    bird
}

fn print_person(person: &Person) {
    println!("{:?}", person);
}

fn main() {
    let mut person: Person = create_person();
    let halwk = create_bird("Hawlk".into());

    println!("{:?}", halwk);

    halwk.sing();

    halwk.eat();

    println!("Passáro: {}", halwk.name);

    print_person(&person);

    person.set_age(26);

    println!("{:?}", person)
}
