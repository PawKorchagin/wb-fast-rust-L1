use std::any::{Any, TypeId};

fn print_type<T: Any>(_: &T) {
    let type_id = TypeId::of::<T>();

    match type_id {
        id if id == TypeId::of::<i32>() => println!("Type is i32"),
        id if id == TypeId::of::<f64>() => println!("Type is f64"),
        id if id == TypeId::of::<String>() => println!("Type is String"),
        id if id == TypeId::of::<bool>() => println!("Type is bool"),
        _ => println!("Unknown type"),
    }
}

fn main() {
    let int_var: i32 = 42;
    let float_var: f64 = 3.14;
    let string_var: String = String::from("not String");
    let bool_var: bool = true;

    print_type(&int_var);
    print_type(&float_var);
    print_type(&string_var);
    print_type(&bool_var);
}
