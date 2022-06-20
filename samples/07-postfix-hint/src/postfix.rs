use std::fmt::{self, Debug, Display};

pub(crate) struct PostfixConvertor {
    pub(crate) infix: Vec<Term>,
}
#[derive(Debug)]
pub enum Term {
    Operand(String),
    Operator(Operator),
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub enum Operator {
    Exp,
    Mul,
    Div,
    Add,
    Sub,
}

impl Display for PostfixConvertor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.infix.iter().map(|t| writeln!(f, "{}", t)).collect()
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Operand(o) => write!(f, "{}", o),
            Self::Operator(o) => write!(f, "{}", o),
            Term::OpenParen => write!(f, "("),
            Term::CloseParen => write!(f, ")"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Exp => write!(f, "^"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
        }
    }
}
impl Operator {
    fn precedence(&self) -> i8 {
        match self {
            Operator::Exp => 2,
            Operator::Mul | Operator::Div => 1,
            Operator::Add | Operator::Sub => 0,
        }
    }

    fn is_left_to_right(&self) -> bool {
        if let Operator::Exp = self {
            true
        } else {
            false
        }
    }
}

impl PostfixConvertor {
    pub fn read_data_from_file(path: &str) -> Result<Vec<Term>, String> {
        let input = std::fs::read_to_string(path)
            .map_err(|e| format!("failed opening file '{}' : {}", path, e.to_string()))?;
        PostfixConvertor::read_data(input)
    }

    pub fn read_data(input: String) -> Result<Vec<Term>, String> {
        let mut res = Vec::new();

        let mut iter = input.chars().peekable();
        while let Some(&ch) = iter.peek() {
            iter.next();
            match ch {
                '0'..='9' => {
                    let mut op: String = String::from(ch);
                    while let Some(digit) = iter.next_if(|&c| c >= '0' && c <= '9') {
                        op.push(digit);
                    }
                    res.push(Term::Operand(op));
                }
                '^' => {
                    res.push(Term::Operator(Operator::Exp));
                }
                '*' => {
                    res.push(Term::Operator(Operator::Mul));
                }
                '/' => {
                    res.push(Term::Operator(Operator::Div));
                }
                '+' => {
                    res.push(Term::Operator(Operator::Add));
                }
                '-' => {
                    res.push(Term::Operator(Operator::Sub));
                }
                ' ' => {}
                '(' => {
                    res.push(Term::OpenParen);
                }
                ')' => {
                    res.push(Term::CloseParen);
                }
                _ => return Err(format!("unexpected character '{}'", ch)),
            }
        }
        Ok(res)
    }

    fn add_to_result<T: Display>(res: &mut String, val: &T) {
        if !res.is_empty() {
            res.push(' ')
        }
        res.push_str(format!("{}", val).as_str());
    }

    fn pop_to_result_until(&self, stack: &mut Vec<&Term>, cur: &Operator, res: &mut String) {
        while let Some(Term::Operator(top)) = stack.last() {
            if cur.precedence() > top.precedence() {
                break;
            }
            stack.pop();
            PostfixConvertor::add_to_result(res, &format!("{}", top));
        }
    }

    pub fn to_postfix(&self) -> Result<String, String> {
        let mut res = String::from("");
        let mut stack: Vec<&Term> = Vec::new();
        for term in self.infix.iter() {
            match term {
                Term::Operand(oprd) => {
                    PostfixConvertor::add_to_result(&mut res, &format!("{}", oprd))
                }
                Term::CloseParen => {
                    // Pop operators and add to result until an open paren is reached
                    while let Some(optr) = stack.pop() {
                        if let Term::OpenParen = optr {
                            break;
                        }
                        PostfixConvertor::add_to_result(&mut res, &format!("{}", optr));
                    }
                }
                Term::OpenParen => stack.push(term),
                Term::Operator(optr) => {
                    // Compare precedence with operators on the stack
                    let last = stack.last();
                    if let Some(Term::Operator(top_optr)) = last {
                        if optr.precedence() > top_optr.precedence() {
                            // Current operator has higher precedence than the top operator - add to result
                            stack.push(term);
                        } else if optr.precedence() < top_optr.precedence() {
                            // Current operator has lower precedence than the top operator
                            // Pop and add to result until a lower precedence is found
                            stack.pop();
                            PostfixConvertor::add_to_result(&mut res, &format!("{}", top_optr));
                            self.pop_to_result_until(&mut stack, &optr, &mut res);
                            stack.push(term);
                        } else {
                            // Current operator has the same precedence as the top operator
                            if optr.is_left_to_right() {
                                // Pop and add to result until a lower precedence is found
                                self.pop_to_result_until(&mut stack, &optr, &mut res);
                                stack.push(term);
                            } else {
                                // Right-associative
                                stack.push(term);
                            }
                        }
                    } else {
                        stack.push(term);
                    }
                }
            }
        }
        // Pop remaining operators from stack
        for t in stack.iter().rev() {
            match t {
                Term::Operator(optr) => {
                    PostfixConvertor::add_to_result(&mut res, &format!("{}", optr))
                }
                _ => {
                    return Err(String::from(
                        format!("unexpected term {} on stack", t).as_str(),
                    ))
                }
            }
        }

        Ok(res)
    }
}
