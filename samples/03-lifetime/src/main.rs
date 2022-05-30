fn main() {
    println!("Hello, lifetime!");
}

fn life_times<T>() {
    fn print(s: &str); // elided
    fn print<'a>(s: &'a str); // expanded

    fn debug(lvl: usize, s: &str); // elided
    fn debug<'a>(lvl: usize, s: &'a str); // expanded

    fn substr(s: &str, until: usize) -> &str; // elided
    fn substr<'a>(s: &'a str, until: usize) -> &'a str; // expanded

    //fn get_str() -> &str; // ILLEGAL

    //fn frob(s: &str, t: &str) -> &str; // ILLEGAL

    fn get_mut(&mut self) -> &mut T; // elided
    fn get_mut<'a, T>(&'a mut self) -> &'a mut T;
}

fn a() {
    let a = "x";
    {
        let b = substr(a, 10);
    }
}
