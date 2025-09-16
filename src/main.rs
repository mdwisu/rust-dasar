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

#[test]
fn array() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    let first = numbers[0];
    println!("first: {}", first);

    let second = numbers[1];
    println!("second: {}", second);

    let length = numbers.len();
    println!("length: {}", length);
}

#[test]
fn array_mut() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("before: {:?}", numbers);

    numbers[0] = 10;
    numbers[1] = 20;

    println!("after: {:?}", numbers);
}

#[test]
fn array_initialization() {
    let numbers = [0; 5];
    println!("zeros: {:?}", numbers);

    let ones = [1; 5];
    println!("ones: {:?}", ones);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("matrix: {:?}", matrix);

    let first_row = matrix[0];
    println!("first_row: {:?}", first_row);

    let element = matrix[1][2];
    println!("element: {}", element);
}

#[test]
fn array_iteration() {
    let numbers = [1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("num: {}", num);
    }

    for i in 0..numbers.len() {
        println!("numbers[{}]: {}", i, numbers[i]);
    }
}

#[test]
fn constant() {
    const PI: f64 = 3.14159;
    const MAX_POINTS: u32 = 100_000;
    const APP_NAME: &str = "Rust Dasar";

    println!("PI: {}", PI);
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("APP_NAME: {}", APP_NAME);
}

#[test]
fn variable_scope() {
    // Outer scope
    let outer_var = "outer";
    println!("outer_var: {}", outer_var);

    {
        // Inner scope
        let inner_var = "inner";
        println!("outer_var di inner scope: {}", outer_var);
        println!("inner_var: {}", inner_var);
    }

    // Error: inner_var tidak bisa diakses di sini
    // println!("inner_var: {}", inner_var);
    println!("outer_var di outer scope: {}", outer_var);
}

#[test]
fn scope_with_shadowing() {
    let x = 5;
    println!("x awal: {}", x);

    {
        let x = x + 1;
        println!("x di inner scope: {}", x);
    }

    println!("x di outer scope: {}", x);
}

#[test]
fn scope_with_mutability() {
    let mut x = 5;
    println!("x awal: {}", x);

    {
        x = x + 1;
        println!("x di inner scope: {}", x);
    }

    println!("x di outer scope: {}", x);
}

#[test]
fn function_scope() {
    let a = 10;
    println!("a di main: {}", a);

    inner_function();

    // Error: b tidak bisa diakses di sini
    // println!("b: {}", b);
}

fn inner_function() {
    let b = 20;
    println!("b di inner function: {}", b);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Hello");
    println!("a: {}, b: {}", a, b);
}

fn function_b() {
    let a = 20;
    let b = String::from("World");
    println!("a: {}, b: {}", a, b);
}

#[test]
fn string_trim() {
    let name = "   dwi susanto   ";
    let trimmed_name = name.trim();
    println!("trimmed_name: '{}'", trimmed_name);
}

#[test]
fn string_type() {
    let name: &str = "dwi susanto"; // string slice
    let mut greeting: String = String::from("Hello, "); // owned string
    greeting.push_str(name);
    println!("greeting: {}", greeting);

    let muhammad = name.replace("dwi", "muhammad");
    println!("Hello, {}", muhammad);
}

#[test]
fn ownership() {
    let name = String::from("dwi susanto");
    println!("name: {}", name);

    let new_name = name;
    // println!("name: {}", name); // error: value borrowed here after move
    println!("new_name: {}", new_name);

    let another_name = new_name.clone();
    println!("new_name: {}", new_name);
    println!("another_name: {}", another_name);
}

#[test]
fn copy() {
    let x = 10;
    let y = x;
    println!("x: {}, y: {}", x, y);
}