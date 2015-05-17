use parse;

pub fn evaluate(input: &str) -> Result<f64, parse::ParseError> {
    match parse::parse(input) {
        Ok(ast) => Ok(evaluate_node(&ast)),
        Err(err) => Err(err),
    }
}

fn evaluate_node(node: &parse::Node) -> f64 {
    match node {
        &parse::Node::Number(num) => num,
        &parse::Node::Operation{operator, ref children} => {
            match operator {
                '+' => {
                    let mut ans = 0.0;
                    for n in children {
                        ans += evaluate_node(n);
                    }
                    ans
                },
                '-' => {
                    if children.len() == 0 {
                        0.0
                    } else {
                        let mut child_iter = children.iter();
                        let mut ans = evaluate_node(child_iter.next().unwrap());
                        for n in child_iter {
                            ans -= evaluate_node(n);
                        }
                        ans
                    }
                },
                '*' => {
                    let mut ans = 1.0;
                    for n in children {
                        ans *= evaluate_node(n);
                    } ans
                },
                '/' => {
                    if children.len() == 0 {
                        1.0
                    } else {
                        let mut child_iter = children.iter();
                        let mut ans = evaluate_node(child_iter.next().unwrap());
                        for n in child_iter {
                            ans /= evaluate_node(n);
                        }
                        ans
                    }
                },
                _ => unreachable!()
            }
        },
    }
}
