#![feature(type_name_of_val)]
use std::any::{type_name, type_name_of_val};

fn main() {
    let num = 0;
    println!("{}", type_name_of_val(&num)); //retorna i32
    println!("{}", type_name::<i32>()); //retorna i32
}
