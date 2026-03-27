use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{
    ast::{
        BlockStatement, Expression, IfExpression, InfixExpression, PrefixExpression, Program,
        Statement,
    },
    builtins,
    environment::Environment,
    object::{Function, HashPair, Object},
};

fn eval_identifier(node: &crate::ast::Identifier, env: &Rc<RefCell<Environment>>) -> Object {
    if let Some(val) = env.borrow().get(&node.value) {
        return val;
    }

    if let Some(builtin) = builtins::get_builtin(&node.value) {
        return builtin;
    }

    Object::ErrorObj(format!("identifier not found: {}", node.value))
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
            Some(eval_infix_integer_expression(l, r, &infix.operator))
        }
        (Object::String(l), Object::String(r)) => {
            Some(eval_infix_string_expression(l, r, &infix.operator))
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

pub fn eval_infix_string_expression(left: &String, right: &String, operator: &str) -> Object {
    match operator {
        "+" => Object::String(format!("{}{}", left, right)),
        _ => Object::ErrorObj(format!("unknown operator: STRING {} STRING", operator)),
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

pub fn eval_infix_integer_expression(left: &i64, right: &i64, operator: &str) -> Object {
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
        Expression::HashLiteral(hl) => eval_hash_literal(hl, env),
        Expression::CallExpression(ce) => eval_call_expression(ce, env),
        Expression::StringLiteral(sl) => Some(Object::String(sl.value.clone())),
        Expression::ArrayLiteral(al) => {
            let elements = eval_expressions(al.elements.as_deref().unwrap_or(&[]), env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements.into_iter().next();
            }
            Some(Object::Array(elements))
        }

        Expression::IndexExpression(ie) => {
            let left = ie
                .left
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))?;
            if is_error(&left) {
                return Some(left);
            }
            let index = ie
                .index
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))?;
            if is_error(&index) {
                return Some(index);
            }
            Some(eval_index_expression(left, index))
        }
    }
}

fn eval_hash_literal(
    node: &crate::ast::HashLiteral,
    env: &Rc<RefCell<Environment>>,
) -> Option<Object> {
    let mut pairs = HashMap::new();

    for (key_node, value_node) in &node.pairs {
        let key = eval_expression(key_node, env)?;
        if is_error(&key) {
            return Some(key);
        }

        let hash_key = match key.hash_key() {
            Some(hk) => hk,
            None => {
                return Some(Object::ErrorObj(format!(
                    "unusable as hash key: {}",
                    key.object_type()
                )));
            }
        };

        let value = eval_expression(value_node, env)?;
        if is_error(&value) {
            return Some(value);
        }

        pairs.insert(hash_key, HashPair { key, value });
    }

    Some(Object::Hash(pairs))
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    // So parser will parse even index's expression results in string or other objects that's not
    // Integer , this is the part that catch that error.
    match (&left, &index) {
        (Object::Array(_), Object::Integer(_)) => eval_array_index_expression(left, index),
        (Object::Hash(_), _) => eval_hash_index_expression(left, index),
        _ => Object::ErrorObj(format!(
            "index operator not supported: {}",
            left.object_type()
        )),
    }
}

fn eval_hash_index_expression(hash: Object, index: Object) -> Object {
    let Object::Hash(pairs) = hash else {
        return Object::Null;
    };

    let hash_key = match index.hash_key() {
        Some(hk) => hk,
        None => {
            return Object::ErrorObj(format!("unusable as hash key: {}", index.object_type()));
        }
    };

    match pairs.get(&hash_key) {
        Some(pair) => pair.value.clone(),
        None => Object::Null,
    }
}

fn eval_array_index_expression(array: Object, index: Object) -> Object {
    let Object::Array(elements) = array else {
        return Object::Null;
    };
    let Object::Integer(idx) = index else {
        return Object::Null;
    };
    let max = (elements.len() as i64) - 1;
    if idx < 0 || idx > max {
        // showing error message like index out of range might better
        return Object::Null;
    }
    elements[idx as usize].clone()
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
        Object::Builtin(func) => Some(func(args)),
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
    // moves outer to the outer by cloning (rc cloning very cheap) so the func can have both knowledge
    let env = Environment::new_enclosed(Rc::clone(&func.env));
    for (param, arg) in func.parameters.iter().zip(args) {
        // this part define new inner
        env.borrow_mut().set(param.value.clone(), arg);
    }
    env
}

pub fn eval_program(program: &Program, env: &Rc<RefCell<Environment>>) -> Option<Object> {
    let mut result = None;

    for statement in &program.statements {
        result = eval_statement(statement, env);

        // To return back immediately
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
                // value store expressions
                .value
                .as_ref()
                .and_then(|expr| eval_expression(expr, env))?;

            if matches!(val, Object::ErrorObj(_)) {
                return Some(val);
            }

            // name.value name's from identifier and there's a value
            // above value's from expressions that's evaluated to an object
            env.borrow_mut().set(let_stmt.name.value.clone(), val);
            None
        }
        Statement::MultiLet(multi_let) => {
            for (name, expr) in &multi_let.declarations {
                let val = eval_expression(expr, env)?;
                if is_error(&val) {
                    return Some(val);
                }
                env.borrow_mut().set(name.value.clone(), val);
            }
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
        // return back the wrapped (ReturnValue{Something}) back
        if matches!(&result, Some(Object::ReturnValue(_) | Object::ErrorObj(_))) {
            return result;
        }
    }

    result
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::ErrorObj(_))
}
