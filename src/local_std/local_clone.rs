
pub fn clone_module_main() -> std::io::Result<()> {
    let person: Person = Default::default();
    let another_person = person.clone();
    println!("{:?}", another_person);
    Ok(())
}

#[derive(Clone, Debug)]
struct Person {
    firstname: String,
    lastname: String,
    age: i8
}

impl Default for Person {
    fn default() -> Self {
        Self { firstname: "Nicky".to_string(), lastname: "Hariniaina".to_string(), age: 18 }
    }
}
