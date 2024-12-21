use std::collections::HashMap;
use compiler::{evaluate, Apply, Expr, Lambda, Add};


/*

input_apply =
{
    function: {
        lambda: {
            param: "x",
            body: {
                left: "x",
                right: constant(2),
            }
        }
    },
    arg: constant(3),
}
 */
#[test]
fn test_evaluate() {
    let mut input_env = HashMap::new();
    let input_apply = Expr::Apply(Apply {
        function: Box::new(
            Expr::Lambda(Lambda {
                param: "x".to_string(),
                body: Box::new(Expr::Add(Add{
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Constant(2)),
                }))
            })
        ),
        arg: Box::new(Expr::Constant(3)),
    });

    let result = evaluate(&input_apply, &mut input_env);
    assert_eq!(result, 5);
}