use compiler::{tokenize, Token, TokenType};

#[test]

fn test_tokenize_1() {
    let input: &str = "λx.x+2";
    let result = tokenize(input);

    assert_eq!(result[0].token_type, TokenType::Lambda);
    assert_eq!(result[1].token_type, TokenType::Value);
    assert_eq!(result[2].token_type, TokenType::Equal);
    assert_eq!(result[3].token_type, TokenType::Value);
    assert_eq!(result[4].token_type, TokenType::Symbol);
    assert_eq!(result[5].token_type, TokenType::Number);

    assert_eq!(result[0].value, "λ");
    assert_eq!(result[1].value, "x");
    assert_eq!(result[2].value, ".");
    assert_eq!(result[3].value, "x");
    assert_eq!(result[4].value, "+");
    assert_eq!(result[5].value, "2");
}

#[test]
fn test_tokenize_2() {
    let input: &str = "λx.x+23";
    let result = tokenize(input);

    assert_eq!(result[0].token_type, TokenType::Lambda);
    assert_eq!(result[1].token_type, TokenType::Value);
    assert_eq!(result[2].token_type, TokenType::Equal);
    assert_eq!(result[3].token_type, TokenType::Value);
    assert_eq!(result[4].token_type, TokenType::Symbol);
    assert_eq!(result[5].token_type, TokenType::Number);

    assert_eq!(result[0].value, "λ");
    assert_eq!(result[1].value, "x");
    assert_eq!(result[2].value, ".");
    assert_eq!(result[3].value, "x");
    assert_eq!(result[4].value, "+");
    assert_eq!(result[5].value, "23");
}
