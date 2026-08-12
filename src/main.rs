// fn main() {
//     let a = "arunjot";
//     let b = "arunjotsingh";
//     longest(&a, &b);
// }

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() { a } else { b }
// }

// Rule 1
fn main() {
    let a = "arunjot";
    longest(&a);
}

fn longest(a: &str) -> &str {
    a
}

struct User {
    name: String,
}

impl User {
    fn get_name(&self, other: &str) -> &str {
        &self.name
    }
}

fn main() {
    let user = User {
        name: String::from("Arunjot"),
    };

    let other = String::from("Singh");

    let name = user.get_name(&other);

    println!("{}", name);
}
