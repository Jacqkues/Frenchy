use crate::{value::Value, interpret_visitor::InterpretVisitor, error::RuntimeError};

pub fn clock(_interpreter: &mut InterpretVisitor, _arguments: Vec<Value>) -> Result<Value, RuntimeError> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let time = since_the_epoch.as_secs_f64();
    Ok(Value::Number(time))
}

pub fn afficher(_interpreter: &mut InterpretVisitor, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
    for arg in arguments {
        println!("{:?}", arg);
    }
    Ok(Value::Nil)
}

pub fn lire(_interpreter: &mut InterpretVisitor, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
    for arg in arguments {
      println!("{:?}", arg.to_string());
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    Ok(Value::String(input.to_string()))
}

pub fn to_int(_interpreter: &mut InterpretVisitor, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Number(arguments[0].to_string().parse::<f64>().unwrap()))
}