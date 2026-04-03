use std::{thread, time::Duration};

use rand::Rng;

use crate::{object::Object, terminal};

pub fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "len" => Some(Object::Builtin(builtin_len)),
        "first" => Some(Object::Builtin(builtin_first)),
        "last" => Some(Object::Builtin(builtin_last)),
        "rest" => Some(Object::Builtin(builtin_rest)),
        "push" => Some(Object::Builtin(builtin_push)),
        "ရေး" => Some(Object::Builtin(builtin_print)),
        // ── terminal ────────────────────────
        "terminal_init" => Some(Object::Builtin(builtin_terminal_init)),
        "terminal_end" => Some(Object::Builtin(builtin_terminal_end)),
        "clear" => Some(Object::Builtin(builtin_clear)),
        "terminal_size" => Some(Object::Builtin(builtin_terminal_size)),
        "print_at" => Some(Object::Builtin(builtin_print_at)),
        "print_at_center" => Some(Object::Builtin(builtin_print_at_center)),
        "draw_border" => Some(Object::Builtin(builtin_draw_border)),
        "flush" => Some(Object::Builtin(builtin_flush)),

        // ── input ───────────────────────────
        "read_key" => Some(Object::Builtin(builtin_read_key)),
        "poll_key" => Some(Object::Builtin(builtin_poll_key)),

        // ── utilities ───────────────────────
        "sleep" => Some(Object::Builtin(builtin_sleep)),
        "rand" => Some(Object::Builtin(builtin_rand)),
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
        Object::Array(a) if a.is_empty() => Object::Null,
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

//  Terminal builtins

fn builtin_terminal_init(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    match terminal::init() {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

fn builtin_terminal_end(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    terminal::cleanup();
    Object::Null
}

fn builtin_clear(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    match terminal::clear_screen() {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

fn builtin_terminal_size(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    match terminal::size() {
        Ok((w, h)) => Object::Array(vec![Object::Integer(w as i64), Object::Integer(h as i64)]),
        Err(e) => Object::ErrorObj(e),
    }
}

fn builtin_flush(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    match terminal::flush() {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

/// print_at(x, y, text)
fn builtin_print_at(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(3, &args) {
        return err;
    }
    let x = match &args[0] {
        Object::Integer(v) => *v,
        _ => return Object::ErrorObj("print_at: x must be INTEGER".into()),
    };
    let y = match &args[1] {
        Object::Integer(v) => *v,
        _ => return Object::ErrorObj("print_at: y must be INTEGER".into()),
    };
    let text = match &args[2] {
        Object::String(s) => s.clone(),
        other => other.inspect(),
    };
    match terminal::print_at(x as u16, y as u16, &text) {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

/// print_at_center(x, y, box_cols, box_rows, text)
fn builtin_print_at_center(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(5, &args) {
        return err;
    }
    let x = match &args[0] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("print_at_center: x must be INTEGER".into()),
    };
    let y = match &args[1] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("print_at_center: y must be INTEGER".into()),
    };
    let cols = match &args[2] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("print_at_center: cols must be INTEGER".into()),
    };
    let rows = match &args[3] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("print_at_center: rows must be INTEGER".into()),
    };
    let text = match &args[4] {
        Object::String(s) => s.clone(),
        other => other.inspect(),
    };
    match terminal::print_at_center(x, y, cols, rows, &text) {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

/// draw_border(cols, rows)  — centred on screen
fn builtin_draw_border(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(2, &args) {
        return err;
    }
    let cols = match &args[0] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("draw_border: cols must be INTEGER".into()),
    };
    let rows = match &args[1] {
        Object::Integer(v) => *v as u16,
        _ => return Object::ErrorObj("draw_border: rows must be INTEGER".into()),
    };
    match terminal::draw_border(cols, rows) {
        Ok(()) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

//  Input builtins

/// read_key() → string  (blocks until a key is pressed)
fn builtin_read_key(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(0, &args) {
        return err;
    }
    match terminal::read_key_blocking() {
        Ok(k) => Object::String(k),
        Err(e) => Object::ErrorObj(e),
    }
}

/// poll_key(timeout_ms) → string | null
fn builtin_poll_key(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }
    let ms = match &args[0] {
        Object::Integer(v) => *v as u64,
        _ => return Object::ErrorObj("poll_key: argument must be INTEGER (ms)".into()),
    };
    match terminal::poll_key(ms) {
        Ok(Some(k)) => Object::String(k),
        Ok(None) => Object::Null,
        Err(e) => Object::ErrorObj(e),
    }
}

//  Utility builtins

/// sleep(ms)
fn builtin_sleep(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(1, &args) {
        return err;
    }
    let ms = match &args[0] {
        Object::Integer(v) => *v as u64,
        _ => return Object::ErrorObj("sleep: argument must be INTEGER (ms)".into()),
    };
    thread::sleep(Duration::from_millis(ms));
    Object::Null
}

/// rand(min, max) → integer   (inclusive both ends)
fn builtin_rand(args: Vec<Object>) -> Object {
    if let Some(err) = check_arg_count(2, &args) {
        return err;
    }
    let min = match &args[0] {
        Object::Integer(v) => *v,
        _ => return Object::ErrorObj("rand: min must be INTEGER".into()),
    };
    let max = match &args[1] {
        Object::Integer(v) => *v,
        _ => return Object::ErrorObj("rand: max must be INTEGER".into()),
    };
    if min > max {
        return Object::ErrorObj(format!("rand: min ({}) > max ({})", min, max));
    }
    let val = rand::thread_rng().gen_range(min..=max);
    Object::Integer(val)
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
