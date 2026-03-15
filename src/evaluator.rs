use crate::ast::{
    BlockStatement, Expression, IfExpression, InfixExpression, PrefixExpression, Program, Statement,
};
use crate::environment::Environment;
use crate::object::{Object, ObjectType};

// Evaluate Program & Statements

pub fn eval_program(program: &Program, env: &mut Environment) -> Option<Object> {
    let mut result = None;
    for statement in &program.statements {
        if let Some(Object::ErrorObj(_)) = result {
            return result;
        }
        result = eval_statement(statement, env);
        if let Some(Object::ReturnValue(val)) = result {
            return Some(*val);
        }
    }
    result
}

pub fn eval_statement(statement: &Statement, env: &mut Environment) -> Option<Object> {
    // New skill learned
    match statement {
        Statement::Let(let_stmt) => {
            // let val = eval_expression(let_stmt.value, env)?;

            let val = eval_expression(let_stmt.value.as_ref()?, env)?;
            // If evaluating the value results in an error, bubble it up
            if let Object::ErrorObj(_) = val {
                return Some(val);
            }

            env.set(let_stmt.name.value.clone(), val);
            None // Let statements themselves don't return a value to the REPL
        }
        Statement::Expression(expr_stmt) => {
            // Using `and_then` is a clean way to handle Options without nesting `if let`
            expr_stmt
                .expression
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))
        }
        Statement::Return(r_statement) => {
            let val = r_statement
                .return_value
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))
                .unwrap_or(Object::Null);
            Some(Object::ReturnValue(Box::new(val)))
        }

        _ => None,
    }
}

// Evaluate Expression

pub fn eval_expression(expr: &Expression, env: &mut Environment) -> Option<Object> {
    match expr {
        Expression::IntegerLiteral(il) => Some(Object::Integer(il.value)),
        Expression::Identifier(i) => Some(eval_identifier(i, env)),
        Expression::Boolean(b) => Some(Object::Boolean(b.value)),
        Expression::PrefixExpression(pe) => eval_prefix_expression(pe, env),
        Expression::InfixExpression(ie) => eval_infix_expression(ie, env),
        Expression::IfExpression(if_exp) => eval_if_expression(if_exp, env),
        _ => Some(Object::ErrorObj(format!("unknown expression: {:?}", expr))),
    }
}

// Evalvuate Block Statements

pub fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Option<Object> {
    let mut result = None;
    if let Some(stmt_vec) = &block.statements {
        for stmt in stmt_vec {
            if let Some(Object::ErrorObj(_)) = result {
                return result;
            }

            result = eval_statement(stmt, env);
            // to stop immediately when we encounter return
            if let Some(Object::ReturnValue(_)) = result {
                return result;
            }
        }
    };

    return result;
}

// Evaluate Prefix Expression

fn eval_prefix_expression(prefix: &PrefixExpression, env: &mut Environment) -> Option<Object> {
    // The `?` operator automatically returns `None` early if anything fails,
    // keeping `right` in the correct scope!

    // recall the right can be 1 2 3 true false or value producing expression
    let right_expr = prefix.right.as_ref()?;
    let right = eval_expression(right_expr, env)?;

    match prefix.operator.as_str() {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_operator_expression(right)),
        _ => Some(Object::ErrorObj(format!(
            "unknown operator: {}{}",
            prefix.operator,
            right.object_type()
        ))),
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
        _ => Object::ErrorObj(format!("unknown operator: -{}", right.object_type())),
    }
}

fn eval_infix_expression(infix: &InfixExpression, env: &mut Environment) -> Option<Object> {
    let right_expr = infix.right.as_ref()?;
    let left_expr = infix.left.as_ref()?;
    let right = eval_expression(right_expr, env)?;
    let left = eval_expression(left_expr, env)?;

    if left.object_type() != right.object_type() {
        return Some(Object::ErrorObj(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            infix.operator,
            right.object_type()
        )));
    }

    if left.object_type() == ObjectType::Integer && right.object_type() == ObjectType::Integer {
        return Some(eval_infix_integer_expression(left, right, &infix.operator));
    } else {
        match infix.operator.as_str() {
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
            _ => Some(Object::ErrorObj(format!(
                "unknown operator: {} {} {}",
                left.object_type(),
                infix.operator,
                right.object_type()
            ))),
        }
    }
}

fn eval_infix_integer_expression(left: Object, right: Object, operator: &str) -> Object {
    let left_val = match left {
        Object::Integer(i) => i,
        // since implict return won't return function level return
        _ => return Object::ErrorObj(format!("expected Integer, got {}", left.object_type())),
    };

    let right_val = match right {
        Object::Integer(i) => i,
        _ => return Object::ErrorObj(format!("expected Integer, got {}", right.object_type())),
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
        _ => Object::ErrorObj(format!("unknown operator: {}", operator)),
    }
}

pub fn eval_if_expression(if_exp: &IfExpression, env: &mut Environment) -> Option<Object> {
    let condition = eval_expression(if_exp.condition.as_ref()?, env);

    if is_truthy(&condition.unwrap()) {
        if let Some(consequence) = &if_exp.consequence {
            return eval_block_statement(consequence, env);
        } else {
            return Some(Object::Null);
        }
    } else if if_exp.alternative.is_some() {
        if let Some(alternative) = &if_exp.alternative {
            return eval_block_statement(alternative, env);
        } else {
            return Some(Object::Null);
        }
    }

    Some(Object::Null)
}

pub fn is_truthy(object: &Object) -> bool {
    match object {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn eval_identifier(node: &crate::ast::Identifier, env: &Environment) -> Object {
    match env.get(&node.value) {
        Some(val) => val,
        None => Object::ErrorObj(format!("identifier not found: {}", node.value)),
    }
}
