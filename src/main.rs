use std::{collections::btree_map::Range, fmt::format};

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test")
}

#[test]
fn test_variable() {
    let name: &str = "Stiven Stefanus Triski Katuuk";

    println!("Hello {}", name);
}

#[test]
fn test_variable_mut() {
    let mut name = "Stiven Stefanus Triski Katuuk";
    println!("Hello {}", name);

    name = "Risna Sandi Aulia";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Stiven Stefanus Triski Katuuk";
    println!("Hello {}", name);

    // name = "10";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Stiven Stefanus Triski Katuuk";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let age: i32 = 20;

    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 10000000000000;
    println!("{}", d);

    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn augmented_assignment() {
    let mut a: i8 = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn char_type() {
    let a: char = 'a';
    let b: char = 'b';

    println!("{} {}", a,b)
}

#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data)
}

#[test]
fn accessing_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a,b,c);
}

#[test]
fn destructuring_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    let (a, b, c) = data;

    println!("{} {}, {}", a, b, c);
}

#[test]
fn mutable_table() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    data.0 = 15;
    data.1 = 20.5;
    data.2 = false;

    let (a, b, c) = data;

    println!("{} {}, {}", a, b, c);
}

fn unit() {
    println!("hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
}

#[test]
fn array() {
    let array: [i8; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);
}

#[test]
fn accessing_array() {
    let array: [i8; 5] = [1, 2, 3, 4, 5];

    let number1 = array[0];
    let number2 = array[1];

    println!("{} {}", number1, number2);
}

#[test]
fn mutable_array() {
    let mut array: [i8; 5] = [1, 2, 3, 4, 5];

    array[0] = 2;
    array[1] = 3;

    println!("{:?}", array);
}

#[test]
fn count_array_length() {
    let array: [i8; 5] = [1, 2, 3, 4, 5];


    println!("total data array {}", array.len());
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4],
    ];


    println!("{:?}", matrix);
}

const MAXIMUM_RATE: i8 = 100;

#[test]
fn constant() {
    const MINIMUM_RATE: i8 = 0;

    println!("{} {}", MINIMUM_RATE, MAXIMUM_RATE)
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Kurniawan");

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Eko");

    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name: &str = "  Stiven Trizky Katuuk  ";
    let trim: &str = name.trim();

    println!("{}", trim);
    println!("{}", name);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Stiven Trizky");
    name.push_str(" Katuuk");

    println!("{}", name);

    let budi: String = name.replace("Stiven", "Budi");
    println!("{}", budi)
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Stiven");

    let name2 = name1;

    println!("{}", name2)
}

#[test]
fn clone() {
    let name1 = String::from("Stiven");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 10;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else {
        "Bad"
    };



    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result: i8 = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Result : {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;

    'outer: loop  {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            if i  > 10 {
                break;
            }
        }

        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);

        index += 1;
    }
}

#[test]
fn for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for char in array {
        println!("Value: {}", char);
    }
}

#[test]
fn range() {
    let range = 0..5;

    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Range: {}", i)
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=5;

    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Range: {}", i)
    }
}

fn say_hello() {
    println!("Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Good Bye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("stiven", "katuuk");
    say_goodbye("Risna", "Aulia");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);

    println!("Result: {}", result);

    let result = factorial_loop(-10);

    println!("Result: {}", result);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
fn test_full_name() {
    let first_name = String::from("Eko");
    let last_name = String::from("Khannedy");

    let name = full_name(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str(" Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Eko");

    change_value(&mut value);

    println!("{}", value);
}