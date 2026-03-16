pub fn default_module_main() -> std::io::Result<()> {
    let person: Identity = Identity {
        firstame: Default::default(),
        lastname: Default::default(),
        age: Default::default(),
        sexe: Default::default()
    };

    let another_person: Identity = Identity {
        firstame: "Woman".to_string(),
        lastname: "Doe".to_string(),
        age: 30,
        sexe: Sexe::Female
    };

    println!("{:?}", person);

    Ok(())
}

#[derive(Default, Debug)]
struct Identity {
    firstame: String,
    lastname: String,
    age: u8,
    sexe: Sexe
}

#[derive(Debug, Default)]
enum Sexe {
    #[default]
    Male,
    Female
}

struct MyDefaultIs34 {
    random_number: u8
}

impl Default for MyDefaultIs34 {
    fn default() -> Self {
        Self { random_number: 34 }
    }
}
