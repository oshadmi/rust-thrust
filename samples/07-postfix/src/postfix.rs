use std::fmt::{self, Debug, Display};

// Struct to handle infix to postfix conversion
pub(crate) struct PostfixConvertor {
    pub(crate) infix: Vec<Term>,
}

#[derive(Debug)]
/// Enum for Operands, Operators and parenthesis
pub enum Term {
    // PUT YOUR CODE HERE
}

#[derive(Debug)]
/// Enum for Exp, Mul, Div, Add, Sub operators
pub enum Operator {
    // PUT YOUR CODE HERE
}

impl Display for PostfixConvertor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.infix.iter().map(|t| writeln!(f, "{}", t)).collect()
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           // PUT YOUR CODE HERE
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           // PUT YOUR CODE HERE
        }
    }
}
impl Operator {
    fn precedence(&self) -> i8 {
        match self {
            // PUT YOUR CODE HERE
        }
    }

    fn is_left_to_right(&self) -> bool {
        // PUT YOUR CODE HERE
    }
}

impl PostfixConvertor {
    pub fn read_data_from_file(path: &str) -> Result<Vec<Term>, String> {
        // PUT YOUR CODE HERE
        // Use `std::fs::read_to_string`
    }

    pub fn read_data(input: String) -> Result<Vec<Term>, String> {
        let mut res = Vec::new();

        let mut iter = input.chars().peekable();
        while let Some(&ch) = iter.peek() {
            iter.next();
            match ch {
                // PUT YOUR CODE HERE
                // Handle numbers, operators and parenthesis
                // Can use `while let Some(digit) = iter.next_if(...)`
                ' ' => {} // Skip spaces
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
        // PUT YOUR CODE HERE
        // Can use stack.last() and stack.pop()
    }

    pub fn to_postfix(&self) -> Result<String, String> {
        let mut res = String::from("");
        let mut stack: Vec<&Term> = Vec::new();
        for term in self.infix.iter() {
            match term {
                // PUT YOUR CODE HERE
                
                // Operand - add to result
                
                // Close paren - Pop operators and add to result until an open paren is reached
                    
                // Open paren - push to stack
                
                // Operator
                
                    // Compare precedence with operators on the stack
                    // Current operator has higher precedence than the top operator - add to result

                    // Current operator has lower precedence than the top operator
                    // Pop and add to result until a lower precedence is found
                            
                    // Current operator has the same precedence as the top operator
                    // if Left-Associative
                        // Pop and add to result until a lower precedence is found
                    // If Right-associative push to stack

                }
            }
        }
        // Pop remaining operators from stack
        // PUT YOUR CODE HERE
        // Use Vec iter().rev()
        for ... {
            match t {
                // PUT YOUR CODE HERE
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
