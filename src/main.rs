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
