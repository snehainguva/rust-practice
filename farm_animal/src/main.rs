
#[derive(PartialEq, Debug)]
enum FarmAnimal {
    Worm, 
    Cow, 
    Bull, 
    Chicken { num_eggs: usize}, 
    Dog {name: String},
}

impl FarmAnimal {
    fn make_a_sound(&self) -> String {
        match self {
            FarmAnimal::Bull | FarmAnimal::Cow {} => "moo".to_string(),
            FarmAnimal::Chicken { .. } => "cluck, cluck!".to_string(),
            FarmAnimal::Dog { name } => format!("{} come home!", name).to_string(),
            FarmAnimal::Worm => "WORM!!".to_string(),
            _ => "something else...".to_string(),
        }
    }
}

fn what_does_the_animal_say(animal: &FarmAnimal) {
    println!("the {:?} says: {:?}", animal, animal.make_a_sound())
}

fn main() {
    println!("Hello, world!");
    what_does_the_animal_say(&FarmAnimal::Dog{name:"Lassie".to_string()})
}
