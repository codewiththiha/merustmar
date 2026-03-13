use crate::ast::{Expression, InfixExpression, PrefixExpression, Program, Statement};
use crate::object::{Object, ObjectType};

// Evaluate Program & Statements

pub fn eval_program(program: &Program) -> Option<Object> {
    let mut result = None;
    for statement in &program.statements {
        result = eval_statement(statement);
    }
    result
}

pub fn eval_statement(statement: &Statement) -> Option<Object> {
    // New skill learned
    match statement {
        Statement::Expression(expr_stmt) => {
            // Using `and_then` is a clean way to handle Options without nesting `if let`
            expr_stmt
                .expression
                .as_ref()
                .and_then(|expr| eval_expression(expr))
        }
        _ => None,
    }
}

// Evaluate Expression

pub fn eval_expression(expr: &Expression) -> Option<Object> {
    match expr {
        // Notice how much cleaner the initialization is now:
        Expression::IntegerLiteral(il) => Some(Object::Integer(il.value)),
        Expression::Boolean(b) => Some(Object::Boolean(b.value)),
        Expression::PrefixExpression(pe) => eval_prefix_expression(pe),
        Expression::InfixExpression(ie) => eval_infix_expression(ie),
        _ => Some(Object::Null),
    }
}

// Evaluate Prefix Expression

fn eval_prefix_expression(prefix: &PrefixExpression) -> Option<Object> {
    // The `?` operator automatically returns `None` early if anything fails,
    // keeping `right` in the correct scope!

    // recall the right can be 1 2 3 true false or value producing expression
    let right_expr = prefix.right.as_ref()?;
    let right = eval_expression(right_expr)?;

    match prefix.operator.as_str() {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_operator_expression(right)),
        _ => Some(Object::Null), // Unknown operator
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        // We can pattern match the inner boolean directly!
        Object::Boolean(b) => Object::Boolean(!b),
        Object::Null => Object::Boolean(true),
        // Everything else in Monkey is truthy, so negating it makes it false
        _ => Object::Boolean(false),
    }
}

fn eval_minus_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(-i),
        _ => Object::Null,
    }
}

fn eval_infix_expression(infix: &InfixExpression) -> Option<Object> {
    let right_expr = infix.right.as_ref()?;
    let left_expr = infix.left.as_ref()?;
    let right = eval_expression(right_expr)?;
    let left = eval_expression(left_expr)?;

    if left.object_type() == ObjectType::Integer && right.object_type() == ObjectType::Integer {
        return Some(eval_infix_integer_expression(left, right, &infix.operator));
    } else {
        match infix.operator.as_str() {
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
            _ => Some(Object::Null),
        }
    }
}

fn eval_infix_integer_expression(left: Object, right: Object, operator: &str) -> Object {
    let left_val = match left {
        Object::Integer(i) => i,
        // since implict return won't return function level return
        _ => return Object::Null,
    };

    let right_val = match right {
        Object::Integer(i) => i,
        _ => return Object::Null,
    };

    match operator {
        "+" => Object::Integer(left_val + right_val),
        "-" => Object::Integer(left_val - right_val),
        "*" => Object::Integer(left_val * right_val),
        "/" => Object::Integer(left_val / right_val),
        ">" => Object::Boolean(left_val > right_val),
        "<" => Object::Boolean(left_val < right_val),
        "==" => Object::Boolean(left_val == right_val),
        "!=" => Object::Boolean(left_val != right_val),
        _ => Object::Null,
    }
}
