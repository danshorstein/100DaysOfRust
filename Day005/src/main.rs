use std::fmt::Debug;

fn main() {
    let me = build_dude("Daniel".to_string(), 69, "yes please!".to_string());
    println!("{:?}", me); 
}


#[derive(Debug)]
struct Dude {
    name: String,
    email: String,
    age: i32,
    sex: String
}

fn build_dude (name: String, age: i32, sex: String) -> Dude {
    let email = format!("{}@fuck.yeah", name).to_string();

    Dude {
          name,
          age,
          sex,
          email 
        }
}
