#[derive(Clone)]
struct Coffee {
    flavor: String,
    strength: u64,
}

fn print_mut(coffee: &mut Coffee) {
    println!("name: '{}', strength: {}", coffee.flavor, coffee.strength);
}

fn print(coffee: &Coffee) {
    println!("name: '{}', strength: {}", coffee.flavor, coffee.strength);
}

#[allow(dead_code)]
fn dillute(mut coffee: Coffee) {
    coffee.strength /= 2;
}

fn main() {
    // Instantiate a struct instance
    let mut morning = Coffee {
        flavor: String::from("dark vader"),
        strength: 13,
    };

    print_mut(&mut morning);

    let other = &morning;

    print_mut(&mut morning);

    //print(&other);
}
