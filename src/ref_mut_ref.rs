pub fn run() {
    let a = String::from("arunjot");

    // Immutable borrow
    reference(&a);

    let mut d = String::from("Arunjot");

    // Mutable borrow
    reference_mut(&mut d);

    println!("d is {}", d);
}

fn reference(a: &str) {
    println!("a is {}", a);
}

fn reference_mut(a: &mut String) {
    a.push_str("_singh");
}
