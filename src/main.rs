fn main() {
    println!("Operator precedence");

    let input = "a+b*c/d-e-f*g*h";
    println!("input : {:?}", input);

    let tokens = tokenize_string(input);
    println!("tokenize_string : {:?}", tokens);

    let tokens2 = to_tokens(&tokens);
    println!("to_tokens : {:?}", tokens2);

    let tokens3 = reverse_polish_notation(&tokens2);
    println!("reverse_polish_notation : {:?}", tokens3);

    let rpn_string = tokens_to_string(&tokens3);
    println!("rpn_string : {:?}", rpn_string);
}

fn tokenize_string(input_str: &str) -> Vec<String> {
    let mut tokens = Vec::<String>::new();
    for c in input_str.chars() {
        tokens.push(c.to_string());
    }
    tokens
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn get_priority(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Subtract => 0,
            Self::Multiply => 1,
            Self::Divide => 1,
        }
    }
    fn to_string(self) -> String {
        match self {
            Self::Add => "+".to_string(),
            Self::Subtract => "-".to_string(),
            Self::Multiply => "*".to_string(),
            Self::Divide => "/".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Operand { name: String },
    Operator { value: Operator },
}

impl Token {
    fn to_string(&self) -> String {
        match self {
            Self::Operand { name } => name.clone(),
            Self::Operator { value } => value.to_string(),
        }
    }
}

fn to_tokens(string_tokens: &Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();

    for s in string_tokens {
        tokens.push(match s.as_str() {
            "+" => Token::Operator {
                value: Operator::Add,
            },
            "-" => Token::Operator {
                value: Operator::Subtract,
            },
            "*" => Token::Operator {
                value: Operator::Multiply,
            },
            "/" => Token::Operator {
                value: Operator::Divide,
            },
            _ => Token::Operand {
                name: s.to_string(),
            },
        })
    }

    tokens
}

fn tokens_to_string(tokens: &Vec<Token>) -> String {
    tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("")
}

type OperatorStack = Vec<Operator>;

fn reverse_polish_notation(input_tokens: &Vec<Token>) -> Vec<Token> {
    let mut operator_stack = OperatorStack::new();
    let mut rpn_tokens = Vec::<Token>::new();

    for token in input_tokens {
        match token {
            Token::Operand { name: _ } => rpn_tokens.push(token.clone()),
            Token::Operator { value } => {
                let current_operator_priority = value.get_priority();
                if operator_stack.is_empty() {
                    operator_stack.push(*value);
                } else {
                    if current_operator_priority > operator_stack.last().unwrap().get_priority() {
                        operator_stack.push(*value)
                    } else {
                        while !operator_stack.is_empty() {
                            let top_op = *operator_stack.last().unwrap();
                            if top_op.get_priority() >= current_operator_priority {
                                operator_stack.pop();
                                rpn_tokens.push(Token::Operator { value: top_op });
                            } else {
                                break;
                            }
                        }
                        operator_stack.push(*value);
                    }
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let top_op = *operator_stack.last().unwrap();
        operator_stack.pop();
        rpn_tokens.push(Token::Operator { value: top_op });
    }

    rpn_tokens
}
