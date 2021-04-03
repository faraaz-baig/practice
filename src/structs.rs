// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // let mut c = Color(255, 135, 7);

    // c.1 = 32;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let p = Person::new("Faraaz", "Baig");
    println!("Person: {} {}", p.first_name, p.last_name);
}
