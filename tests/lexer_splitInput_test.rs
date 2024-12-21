use compiler::split_input_to_char;

#[test]
fn test_split_input_1() {
    let input: &str = "λx.x+2";
    let result = split_input_to_char(&input);
    assert_eq!(result, vec!['λ', 'x', '.', 'x', '+', '2']);
}

#[test]
fn test_split_input_withspace() {
    let input: &str = "λ x . x + 2";
    let result = split_input_to_char(&input);
    assert_eq!(result, vec!['λ', 'x', '.', 'x', '+', '2']);
}

// #[test]
// fn test_split_input_2() {
//     let input: &str = "(λx. x + 2) 3";
// }
