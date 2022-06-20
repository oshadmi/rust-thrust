mod postfix;

fn main() {
    let in_file = std::env::args()
        .nth(1)
        .expect("Need input text file with postfix expression");

    let terms = postfix::PostfixConvertor::read_data_from_file(in_file.as_str()).expect("failed");
    let pf = postfix::PostfixConvertor { infix: terms };
    println!("{}", pf);
    println!("{}", pf.to_postfix().unwrap());
}

#[test]
fn test_plain() {
    let input = String::from("10 ^ 2 + 30 / 2 - 5");
    let terms = postfix::PostfixConvertor::read_data(input).expect("failed parsing");
    let pf = postfix::PostfixConvertor { infix: terms };
    assert_eq!(
        pf.to_postfix().unwrap(),
        String::from("10 2 ^ 30 2 / + 5 -")
    );
}

#[test]
fn test_paren() {
    let input = String::from("10 ^ 2 + 30 / (2 - 5)");
    let terms = postfix::PostfixConvertor::read_data(input).expect("failed parsing");
    let pf = postfix::PostfixConvertor { infix: terms };
    assert_eq!(
        pf.to_postfix().unwrap(),
        String::from("10 2 ^ 30 2 5 - / +")
    );
}
