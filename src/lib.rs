use std::collections::HashMap;

#[derive(PartialOrd, PartialEq, Debug)]
pub enum TokenType {
    Lambda,
    Value,
    Equal,
    Number,
    Symbol,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn split_input_to_char(string: &str) -> Vec<char> {
    let string_without_space: String = string.chars().filter(|c| !c.is_whitespace()).collect();
    string_without_space.chars().collect()
}

pub fn tokenize(string: &str) -> Vec<Token> {
    let mut Tokens = Vec::new();
    let mut string_normalized = split_input_to_char(string);

    let mut is_number: bool = false;
    while !string_normalized.is_empty() {
        let token = string_normalized.remove(0);

        match token {
            'λ' => {
                is_number = false;
                Tokens.push(Token {
                    token_type: TokenType::Lambda,
                    value: token.to_string(),
                })
            }
            '0'..='9' => {
                if is_number {
                    let last_token = Tokens.pop().unwrap();
                    Tokens.push(Token {
                        token_type: TokenType::Number,
                        value: last_token.value + &token.to_string(),
                    });
                } else {
                    Tokens.push(Token {
                        token_type: TokenType::Number,
                        value: token.to_string(),
                    });
                }

                is_number = true;
            }
            '+' | '-' | '*' | '/' => {
                is_number = false;
                Tokens.push(Token {
                    token_type: TokenType::Symbol,
                    value: token.to_string(),
                })
            }
            'a'..='z' | 'A'..='Z' => {
                is_number = false;
                Tokens.push(Token {
                    token_type: TokenType::Value,
                    value: token.to_string(),
                })
            }
            '.' => {
                is_number = false;
                Tokens.push(Token {
                    token_type: TokenType::Equal,
                    value: token.to_string(),
                })
            }
            _ => {
                is_number = false;
                panic!("Invalid token: {token}")
            }
        }
    }

    Tokens
}

// ast ----

#[derive(Debug)]
pub enum Expr {
    Lambda(Lambda),
    Apply(Apply),
    Add(Add),
    Var(String),
    Constant(i32),
}
#[derive(Debug)]
pub struct Lambda {
    pub param: String,
    pub body: Box<Expr>
}
#[derive(Debug)]
pub struct Apply {
    pub function: Box<Expr>,
    pub arg: Box<Expr>,
}
#[derive(Debug)]
pub struct Add {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

pub fn evaluate(expr: &Expr, env: &mut HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Var(name) => *env.get(name).expect("Environment not found"),
        Expr::Constant(value) => *value,
        Expr::Add(add) => {
            evaluate(&add.left, env) + evaluate(&add.right, env)
        },
        Expr::Apply(apply) => {
            if let Expr::Lambda(lambda) = &*apply.function {
                let arg_value = evaluate(&*apply.arg, env);
                env.insert(lambda.param.clone(), arg_value);
                evaluate(&lambda.body, env)
            } else {
                panic!("Invalid apply function");
            }
        }
        _ => {panic!("test");}
    }
}
