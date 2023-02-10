#[cfg(test)]
mod tests {
    // use crate::{interpreter::{env::Env, value::Value}};

    // #[test]
    // fn define_variable_in_global_scope() {
    //     let mut env = Env::new();
    //     env.define(&String::from("a"), Value::Number(5.0)).unwrap();

    //     assert_eq!(env.get(&String::from("a")), Some(&Value::Number(5.0)));
    // }

    // #[test]
    // fn define_variable_in_local_scope() {
    //     let mut env = Env::new();
    //     env.define(&String::from("a"), Value::Number(5.0)).unwrap();

    //     let local = env.enter();
    //     local.define(&String::from("b"), Value::Number(6.0)).unwrap();

    //     assert_eq!(local.get(&String::from("a")), Some(&Value::Number(5.0)));
    //     assert_eq!(local.get(&String::from("b")), Some(&Value::Number(6.0)));
    // }

    // #[test]
    // fn override_global_variable_in_nested_scope() {
    //     let mut env = Env::new();
    //     env.define(&String::from("a"), Value::Number(5.0)).unwrap();

    //     let local = env.enter();
    //     local.define(&String::from("a"), Value::Number(6.0)).unwrap();

    //     assert_eq!(local.get(&String::from("a")), Some(&Value::Number(6.0)));
    //     local.leave();

    //     assert_eq!(env.get(&String::from("a")), Some(&Value::Number(5.0)));
    // }

    // #[test]
    // fn undefined_variable_in_environment() {
    //     let env = Env::new();
    //     assert_eq!(env.get(&String::from("a")), None);
    // }

    // #[test]
    // #[should_panic]
    // fn redefinition_of_variable() {
    //     let mut env = Env::new();
    //     env.define(&String::from("a"), Value::Number(5.0)).unwrap();
    //     env.define(&String::from("a"), Value::Number(6.0)).unwrap();
    // }
}