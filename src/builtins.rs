use crate::object::Object;

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "len" => Some(Object::Builtin(builtin_len)),
        _ => None,
    }
}

fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::ErrorObj(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        _ => Object::ErrorObj(format!(
            "argument to `len` not supported, got {}",
            args[0].object_type()
        )),
    }
}
