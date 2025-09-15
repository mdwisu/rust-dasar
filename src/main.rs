fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    let name= "muhammad dwi susanto";
    println!("Hello, {}", name);
    let mut age = 20;
    println!("age: {}", age);
    age = 21;
    println!("age: {}", age);
}

#[test]
fn shadowing() {
    let name = "dwi";
    println!("name: {}", name);
    let name = "susanto";
    println!("name: {}", name);
}

#[test]
fn comment() {
    // ini adalah comment
    /*
    ini adalah comment
    yang lebih dari satu baris
    */
    println!("Hello, comment!");
}

#[test]
fn explicit_type() {
    let name: &str = "dwi";
    let age: u8 = 20;
    let is_married: bool = false;
    let height: f32 = 170.5;
    println!("name: {}, age: {}, is_married: {}, height: {}", name, age, is_married, height);
}

#[test]
fn number(){
    let a: i32 = 10;
    println!("a: {}", a);
    let b: f64 = 20.5; 
    println!("b: {}", b);
    let c: isize = 30; 
    println!("c: {}", c);
    let d: usize = 40;
    println!("d: {}", d);
}

#[test]
fn number_conversion() {
    let a: i32 = 10;
    let b: f64 = a as f64;
    println!("b: {}", b);
    let c: u8 = 20;
    let d: i32 = c as i32;
    println!("d: {}", d);
}
