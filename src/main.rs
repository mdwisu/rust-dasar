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

#[test]
fn numeric_operations() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;
    println!("a + b = {}", c);
    let d: i32 = a - b;
    println!("a - b = {}", d);
    let e: i32 = a * b;
    println!("a * b = {}", e);
    let f: i32 = b / a;
    println!("b / a = {}", f);
    let g: i32 = b % a;
    println!("b % a = {}", g);
}

#[test]
fn augmented_assignment() {
    let mut a: i32 = 10;
    println!("a: {}", a);
    a += 5;
    println!("a: {}", a);
    a -= 3;
    println!("a: {}", a);
    a *= 2;
    println!("a: {}", a);
    a /= 4;
    println!("a: {}", a);
    a %= 3;
    println!("a: {}", a);
}

#[test]
fn boolean_comparison() {
    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true: {}, is_false: {}", is_true, is_false);
    let is_greater: bool = 10 > 5;
    println!("is_greater: {}", is_greater);
    let is_less: bool = 10 < 5;
    println!("is_less: {}", is_less);
    let is_equal: bool = 10 == 10;
    println!("is_equal: {}", is_equal);
    let is_not_equal: bool = 10 != 5;
    println!("is_not_equal: {}", is_not_equal);
    let is_greater_equal: bool = 10 >= 5;
    println!("is_greater_equal: {}", is_greater_equal);
    let is_less_equal: bool = 10 <= 5;
    println!("is_less_equal: {}", is_less_equal);
}

#[test]
fn operator_boolean(){
    let a: bool = true;
    let b: bool = false;
    let c: bool = a && b;
    println!("a && b = {}", c);
    let d: bool = a || b;
    println!("a || b = {}", d);
    let e: bool = !a;
    println!("!a = {}", e);
}

#[test]
fn character() {
    let a: char = 'a';
    let b: char = 'b';
    let c: char = 'c';
    println!("a: {}, b: {}, c: {}", a, b, c);
    let d: char = '😊';
    println!("d: {}", d);
}

#[test]
fn tuple() {
    let mut person: (&str, u8, bool) = ("dwi", 20, false);
    println!("person: {:?}", person);
    let (name, age, is_married) = person;
    println!("name: {}, age: {}, is_married: {}", name, age, is_married);
    person.0 = "susanto";
    let name2 = person.0;
    let age2 = person.1;
    let is_married2 = person.2;
    println!("name: {}, age: {}, is_married: {}", name2, age2, is_married2);
}

#[test]
fn unit() {
    println!("Hello, unit!");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("result: {:?}", result);

    let test: () = ();
    println!("test: {:?}", test);
}
