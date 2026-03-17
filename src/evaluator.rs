use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    ast::{
        BlockStatement, Expression, IfExpression, InfixExpression, PrefixExpression, Program,
        Statement,
    },
    environment::Environment,
    object::{Function, Object},
};

fn eval_identifier(node: &crate::ast::Identifier, env: &Rc<RefCell<Environment>>) -> Object {
    env.borrow()
        .get(&node.value)
        .unwrap_or_else(|| Object::ErrorObj(format!("identifier not found: {}", node.value)))
}

pub fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn eval_bang_operator_expression(right: &Object) -> Object {
    Object::Boolean(!is_truthy(right))
}

fn eval_minus_operator_expression(right: &Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(-i),
        _ => Object::ErrorObj(format!("unknown operator: -{}", right.object_type())),
    }
}

fn eval_prefix_expression(
    prefix: &PrefixExpression,
    env: &Rc<RefCell<Environment>>,
) -> Option<Object> {
    let right = prefix
        .right
        .as_ref()
        .and_then(|expr| eval_expression(expr, env))?;

    if is_error(&right) {
        return Some(right);
    }

    match prefix.operator.as_str() {
        "!" => Some(eval_bang_operator_expression(&right)),
        "-" => Some(eval_minus_operator_expression(&right)),
        _ => Some(Object::ErrorObj(format!(
            "unknown operator: {}{}",
            prefix.operator,
            right.object_type()
        ))),
    }
}

fn eval_infix_expression(
    infix: &InfixExpression,
    env: &Rc<RefCell<Environment>>,
) -> Option<Object> {
    let left = infix
        .left
        .as_ref()
        .and_then(|expr| eval_expression(expr, env))?;

    if is_error(&left) {
        return Some(left);
    }

    let right = infix
        .right
        .as_ref()
        .and_then(|expr| eval_expression(expr, env))?;

    if is_error(&right) {
        return Some(right);
    }

    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => {
            Some(eval_infix_integer_expression(*l, *r, &infix.operator))
        }
        (Object::Boolean(_), Object::Boolean(_)) => match infix.operator.as_str() {
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
            _ => Some(Object::ErrorObj(format!(
                "unknown operator: {} {} {}",
                left.object_type(),
                infix.operator,
                right.object_type()
            ))),
        },
        _ => Some(Object::ErrorObj(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            infix.operator,
            right.object_type()
        ))),
    }
}

pub fn eval_if_expression(if_exp: &IfExpression, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    let condition = if_exp
        .condition
        .as_ref()
        .and_then(|expr| eval_expression(expr, env))?;

    if is_error(&condition) {
        return Some(condition);
    }

    if is_truthy(&condition) {
        match &if_exp.consequence {
            Some(block) => eval_block_statement(block, env),
            None => Some(Object::Null),
        }
    } else if let Some(alternative) = &if_exp.alternative {
        eval_block_statement(alternative, env)
    } else {
        Some(Object::Null)
    }
}

pub fn eval_infix_integer_expression(left: i64, right: i64, operator: &str) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        ">" => Object::Boolean(left > right),
        "<" => Object::Boolean(left < right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::ErrorObj(format!("unknown operator: {}", operator)),
    }
}

pub fn eval_expression(expr: &Expression, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    match expr {
        Expression::IntegerLiteral(il) => Some(Object::Integer(il.value)),
        Expression::Boolean(b) => Some(Object::Boolean(b.value)),
        Expression::Identifier(i) => Some(eval_identifier(i, env)),
        Expression::PrefixExpression(pe) => eval_prefix_expression(pe, env),
        Expression::InfixExpression(ie) => eval_infix_expression(ie, env),
        Expression::IfExpression(if_exp) => eval_if_expression(if_exp, env),
        Expression::FunctionLiteral(fl) => {
            let parameters = fl.parameters.as_ref()?.clone();
            let body = fl.body.as_ref()?.clone();

            Some(Object::Function(Function {
                parameters,
                body,
                env: Rc::clone(env), // ← shared pointer, NOT deep copy
            }))
        }
        Expression::CallExpression(ce) => eval_call_expression(ce, env),
    }
}

pub fn eval_call_expression(
    ce: &crate::ast::CallExpression,
    env: &Rc<RefCell<Environment>>,
) -> Option<Object> {
    let function = ce
        .function
        .as_ref()
        .and_then(|expr| eval_expression(expr, env))?;

    if is_error(&function) {
        return Some(function);
    }

    let args = eval_expressions(ce.arguments.as_deref().unwrap_or(&[]), env);

    if args.len() == 1 && is_error(&args[0]) {
        return args.into_iter().next();
    }

    apply_function(function, args)
}

fn eval_expressions(exprs: &[Expression], env: &Rc<RefCell<Environment>>) -> Vec<Object> {
    let mut result = Vec::new();

    for expr in exprs {
        match eval_expression(expr, env) {
            Some(obj) if is_error(&obj) => return vec![obj],
            Some(obj) => result.push(obj),
            None => {}
        }
    }

    result
}

fn apply_function(function: Object, args: Vec<Object>) -> Option<Object> {
    match function {
        Object::Function(func) => {
            let extended_env = extended_function_env(&func, args);
            let evaluated = eval_block_statement(&func.body, &extended_env)?;
            Some(unwrap_return_value(evaluated))
        }
        _ => Some(Object::ErrorObj(format!(
            "not a function: {}",
            function.object_type()
        ))),
    }
}

fn unwrap_return_value(obj: Object) -> Object {
    match obj {
        Object::ReturnValue(val) => *val,
        _ => obj,
    }
}

fn extended_function_env(func: &Function, args: Vec<Object>) -> Rc<RefCell<Environment>> {
    let env = Environment::new_enclosed(Rc::clone(&func.env));
    for (param, arg) in func.parameters.iter().zip(args) {
        env.borrow_mut().set(param.value.clone(), arg);
    }
    env
}

pub fn eval_program(program: &Program, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    let mut result = None;

    for statement in &program.statements {
        result = eval_statement(statement, env);

        if let Some(Object::ReturnValue(val)) = result {
            return Some(*val);
        }
        if matches!(&result, Some(Object::ErrorObj(_))) {
            return result;
        }
    }

    result
}

pub fn eval_statement(statement: &Statement, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    match statement {
        Statement::Let(let_stmt) => {
            let val = let_stmt
                .value
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))?;

            if matches!(val, Object::ErrorObj(_)) {
                return Some(val);
            }

            env.borrow_mut().set(let_stmt.name.value.clone(), val);
            None
        }
        Statement::Expression(expr_stmt) => expr_stmt
            .expression
            .as_ref()
            .and_then(|expr| eval_expression(expr, env)),
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

pub fn eval_block_statement(
    block: &BlockStatement,
    env: &Rc<RefCell<Environment>>,
) -> Option<Object> {
    let mut result = None;

    for stmt in block.statements.iter().flatten() {
        result = eval_statement(stmt, env);
        if matches!(&result, Some(Object::ReturnValue(_) | Object::ErrorObj(_))) {
            return result;
        }
    }

    result
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::ErrorObj(_))
}

