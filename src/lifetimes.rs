struct User {
    name: String,
}

impl User {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Explicit lifetime elided: returning a reference tied to `&self`
    fn get_name<'a>(&'a self, _other: &str) -> &'a str {
        &self.name
    }
}

/// Lifetime elisions apply automatically for single reference inputs
fn Identity(a: &str) -> &str {
    a
}

/// Explicit lifetimes required here because Rust cannot infer
/// which input reference (`a` or `b`) is being returned.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    // 1. Comparing two string slices
    let name_a = "arunjot";
    let name_b = "arunjotsingh";
    let winner = longest(name_a, name_b);
    println!("Longest string: {winner}");

    // 2. Identity function demonstration
    let single_str = Identity("arunjot");
    println!("Identity return: {single_str}");

    // 3. Struct method lifetime demonstration
    let user = User::new("Arunjot");
    let other = String::from("Singh");

    let user_name = user.get_name(&other);
    println!("User name: {user_name}");
}
