use crate::object::Object;

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "len" => Some(Object::Builtin(builtin_len)),
        "first" => Some(Object::Builtin(builtin_first)),
        "last" => Some(Object::Builtin(builtin_last)),
        "rest" => Some(Object::Builtin(builtin_rest)),
        "push" => Some(Object::Builtin(builtin_push)),
        "ရေး" => Some(Object::Builtin(builtin_print)),
        _ => None,
    }
}

fn builtin_len(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        Object::Array(a) => Object::Integer(a.len() as i64),
        _ => Object::ErrorObj(format!(
            "argument to `len` not supported, got {}",
            args[0].object_type()
        )),
    }
}

fn builtin_first(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }
    match &args[0] {
        //// If we want explict move!
        //     Object::Array(a) if !a.is_empty() => a[0].clone(),  // non-empty array
        // Object::Array(_) => Object::Null,                   // empty array
        Object::Array(a) => a.first().cloned().unwrap_or(Object::Null),
        _ => Object::ErrorObj(format!(
            "argument to `first` must be ARRAY, got {}",
            &args[0].object_type()
        )),
    }
}

fn builtin_last(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }
    match &args[0] {
        Object::Array(a) => a.last().cloned().unwrap_or(Object::Null),
        _ => Object::ErrorObj(format!(
            "argument to `last` must be ARRAY, got {}",
            &args[0].object_type()
        )),
    }
}
fn builtin_rest(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }

    match &args[0] {
        Object::Array(a) => Object::Array(a[1..].to_vec()),
        _ => Object::ErrorObj(format!(
            "argument to `rest` must be ARRAY, got {}",
            args[0].object_type()
        )),
    }
}

fn builtin_push(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(2, &args) {
        return err;
    }
    match &args[0] {
        Object::Array(a) => {
            let mut new_element = a.clone();
            new_element.push(args[1].clone());
            Object::Array(new_element)
        }
        _ => Object::ErrorObj(format!(
            "argument to `push` must be ARRAY, got {}",
            args[0].object_type()
        )),
    }
}

fn builtin_print(args: Vec<Object>) -> Object {
    if args.is_empty() {
        return Object::ErrorObj(format!("Expected at least 1 argument got 0"));
    }
    for arg in args {
        match arg {
            Object::String(s) => print!("{}", s),
            _ => print!("{}", arg.inspect()),
        }
    }
    Object::String("".to_string())
}

// Helpers

fn check_arg_count(expected: usize, args: &[Object]) -> Option<Object> {
    if args.len() != expected {
        Some(Object::ErrorObj(format!(
            "wrong number of arguments. got={}, want={}",
            args.len(),
            expected
        )))
    } else {
        None
    }
}
