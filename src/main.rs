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