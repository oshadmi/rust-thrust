pub mod bar;
mod baz;
mod foo;

use bar::{do_more_stuff, do_stuff};
use baz::do_stuff as baz_do_stuff;
use baz::submodule1::subsubmodule2::func3;
use foo::do_stuff as foo_do_stuff;

fn main() {
    do_stuff();
    do_more_stuff();
    baz_do_stuff();
    foo_do_stuff();

    baz::submodule1::subsubmodule2::func3();

    let a = std::string::String::from("long");

    use std::string::String;
    let b = String::from("short");
    println!("{} and {}", a, b);
}
