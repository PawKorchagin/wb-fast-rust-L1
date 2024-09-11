///
/// Public trait action wit method say
/// 
pub trait Action {
    fn say(&self);
}

///
/// Structure Person with public field name as String
/// 
pub struct Person {
    pub name: String
}

///
/// Implementation of trait Action for Person
/// 
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name)
    }
}

///
/// Implementation of struct Person with method new
/// 
impl Person {
    fn new(name: String) -> Self {
        Person { name: name }
    }
}

fn main() {
    let name = "PawKorchagin";
    let person = Person::new(name.to_string());

    person.say();
}
