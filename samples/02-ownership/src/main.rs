#![allow(dead_code)]

struct Coffee {
    flavor: String,
    strength: u64,
}

impl Coffee {
    // A method
    #[allow(dead_code)]
    /// Returns a new blend
    fn blend(&self, with: &Coffee, new_name: String) -> Coffee {
        Coffee {
            flavor: new_name,
            strength: (self.strength + with.strength) / 2,
        }
    }

    // An associated function (no '&self' or '&mut self')
    fn print(coffee: Coffee) {
        println!("name: '{}', strength: {}", coffee.flavor, coffee.strength);
    }
}

fn main() {
    let c = blend();
    Coffee::print(c);

    play();
}

fn blend() -> Coffee {
    // Instantiate 2 struct instances
    let morning = Coffee {
        flavor: String::from("dark vader"),
        strength: 13,
    };

    let afternoon = Coffee {
        flavor: String::from("mellow yellow"),
        strength: 6,
    };

    // Last expression is the return value (no need for a return statement)
    morning.blend(&afternoon, String::from("noonish"))
}

#[test]
#[allow(unused_variables)]
fn test_simple() {
    let c = blend();
    assert_eq!(c.strength, 9);
}

#[allow(unused)]
fn play() {
    let mut str1: &str = "slice";
    let mut str2: String = String::from("abcde");

    println!("str2: {}", str2);
    // String slice
    let sub1 = &mut str2[1..3];
    println!("sub1: {}", sub1);

    // Tuple
    let example = ("val1", "val2");
    let v1 = example.0;
    let v2 = example.1;

    // Fixed-size Array
    let a: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
    let b = [1, 2, 3, 4]; // inferred type [i32; 4]
}

#[allow(dead_code)]
pub struct SomeStructWithRefs<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
    space: &'a [u8],
    newline: &'a [u8],
}

struct MyStructWithRef<'a> {
    a: u32,
    b: &'a u64,
}

impl<'a> MyStructWithRef<'a> {
    fn do_something(&self) -> u64 {
        3
    }
}

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     //...
//     x
// }

fn simple_mutability() {
    let mut name: String = String::from("Rumpelstiltskin");
    let mut name2: String = String::from("Tremotino");
    let name3: String = String::from("עוצלי");
    let name4: String = String::from("גוצלי");

    //--------------------------------------------------

    // ref cannot be re-assigned, the String it referres to can be mutated
    let immut_ref_to_mut_string: &mut String = &mut name;
    immut_ref_to_mut_string.push_str("");

    // immut_ref_to_mut_string = &mut name2; - error

    //--------------------------------------------------

    // ref can be re-assigned, the String it referres to can be mutated
    let mut mut_ref_to_mut_string: &mut String = &mut name;

    mut_ref_to_mut_string = &mut name2;
    mut_ref_to_mut_string.push_str("123"); // error

    //--------------------------------------------------

    // ref cannot be re-assigned, the String it referres to cannnot be mutated
    let immut_ref_to_immut_string: &String = &name3;

    // immut_ref_to_immut_string = &name4; // error
    // immut_ref_to_immut_string.push_str("cannot mutate"); // error

    //--------------------------------------------------

    // // ref can be re-assigned, the String it referres to cannnot be mutated
    let mut mut_ref_to_mut_string: &mut String = &mut name;
    mut_ref_to_mut_string = &mut name2;
    mut_ref_to_mut_string.push_str("can mutate");
}
