mod lib;
use lib::*;

use std::collections::HashMap;
fn evaluate(expr: &Expr, env: &mut HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Constant(value) => *value, // 常量直接返回值
        Expr::Var(name) => *env.get(name).expect("Undefined variable"), // 查找變數的值
        Expr::Add(add) => {
            // 解釋左右操作數並相加
            evaluate(&add.left, env) + evaluate(&add.right, env)
        }
        Expr::Lambda(_) => {
            panic!("Cannot directly evaluate a lambda expression.");
        }
        Expr::Apply(apply) => {
            // 獲取函數與參數
            if let Expr::Lambda(lambda) = &*apply.function {
                let arg_value = evaluate(&apply.arg, env); // 求值參數
                env.insert(lambda.param.clone(), arg_value);    // 將參數值放入環境
                evaluate(&lambda.body, env)                     // 求值函數體
            } else {
                panic!("Attempted to apply a non-lambda expression.");
            }
        }
    }
}

fn main() {
    let mut env = HashMap::new();

    let add_expr = Expr::Add(Add {
        left: Box::new(Expr::Var("x".to_string())),
        right: Box::new(Expr::Constant(2)),
    });

    let lambda_expr = Expr::Lambda(Lambda {
        param: "x".to_string(), // 參數是 x
        body: Box::new(add_expr),
    });

    let apply_expr = Expr::Apply(Apply {
        function: Box::new(lambda_expr),   // 被應用的函數
        arg: Box::new(Expr::Constant(3)), // 傳入的參數是 3
    });

    let result = evaluate(&apply_expr, &mut env);
    println!("{:?}", env);
    // println!("Result: {}", result); // Output: 5
}